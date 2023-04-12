all: target/wasm32-wasi/release/plugin-example.wasm
	WASMEDGE_PLUGIN_PATH=/usr/local/lib/wasmedge/ \
		wasmedge --dir ./. target/wasm32-wasi/release/plugin-example.wasm
.PHONY: all

target/wasm32-wasi/release/plugin-example.wasm: src/main.rs
	cargo build --target wasm32-wasi --release
