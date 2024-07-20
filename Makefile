target/wasm32-wasip1/release/redis_guest.wasm:
	cargo component build -p redis-guest --release

host: target/wasm32-wasip1/release/redis_guest.wasm
	cargo build -p redis-host --release

guest: target/wasm32-wasip1/release/redis_guest.wasm
