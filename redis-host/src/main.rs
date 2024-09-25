use redis::Commands;
use std::str;
use wasmredis::host::store::Error as StoreError;
use wasmtime::{
    component::{bindgen, Component, Linker},
    Engine, Store,
};
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

bindgen!("redis-store");

struct MyState {
    ctx: WasiCtx,
    table: ResourceTable,
    pub connection: redis::Connection,
}

impl wasmredis::host::store::Host for MyState {
    fn get(&mut self, key: String) -> Result<Option<Vec<u8>>, StoreError> {
        self.connection
            .get(&key)
            .map_err(|err| StoreError::Other(err.detail().unwrap().to_string()))
    }

    fn set(&mut self, key: String, value: Vec<u8>) -> Result<(), StoreError> {
        let result: Result<String, _> = self.connection.set(&key, value);

        result
            .map(|_| ())
            .map_err(|err| StoreError::Other(err.detail().unwrap().to_string()))
    }

    fn delete(&mut self, key: String) -> Result<Vec<u8>, StoreError> {
        self.connection
            .del(&key)
            .map_err(|err| StoreError::Other(err.detail().unwrap().to_string()))
    }
}

impl WasiView for MyState {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }

    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

fn main() {
    let redis = redis::Client::open("redis://127.0.0.1:6379").unwrap();
    let connection = redis.get_connection().unwrap();
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    RedisStore::add_to_linker(&mut linker, |state: &mut MyState| state).unwrap();
    wasmtime_wasi::add_to_linker_sync(&mut linker).unwrap();

    let mut wasi = WasiCtxBuilder::new();
    let mut store = Store::new(
        &engine,
        MyState {
            ctx: wasi.build(),
            table: ResourceTable::new(),
            connection,
        },
    );
    let bytes = include_bytes!("../../target/wasm32-wasip1/release/redis_guest.wasm");
    let component = Component::new(&engine, bytes).unwrap();
    let (_, run_interface_export) = component.export_index(None, "wasmredis:host/run").unwrap();
    let (_, run_func_export) = component
        .export_index(Some(&run_interface_export), "run")
        .unwrap();
    let instance = linker.instantiate(&mut store, &component).unwrap();
    let func = instance
        .get_typed_func::<(), ()>(&mut store, &run_func_export)
        .expect("run export not found");
    func.call(&mut store, ()).unwrap();

    let mut connection = redis.get_connection().unwrap();
    let value: Vec<u8> = connection.get("foo").unwrap();
    println!(
        "foo: {:?}",
        str::from_utf8(&value).unwrap_or("Invalid UTF-8!")
    );
}
