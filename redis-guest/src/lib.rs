#[allow(warnings)]
mod bindings;

use bindings::{exports::wasmredis::host::run::Guest, wasmredis::host::store as redis};

struct Component;

impl Guest for Component {
    fn run() {
        redis::set("foo", "bar".as_bytes()).unwrap();
    }
}

bindings::export!(Component with_types_in bindings);
