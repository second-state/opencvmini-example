# opencvmini example

> **Warning**
> This repository is moved to https://github.com/second-state/opencvmini/tree/main/example/object-detection

## Dependencies

[opencvmini](https://github.com/WasmEdge/WasmEdge/tree/master/plugins/wasmedge_opencvmini) is a wasmedge plugin, you might like to clone repository [WasmEdge](https://github.com/WasmEdge/WasmEdge), and run the following commands to install this plugin.

```shell
# In repository wasmedge
mkdir build && cd build
cmake -DCMAKE_BUILD_TYPE=Release -DWASMEDGE_BUILD_PLUGINS=ON \
  -DWASMEDGE_PLUGIN_OPENCVMINI=ON \
  -DWASMEDGE_PLUGIN_TENSORFLOWLITE=ON \
  -DWASMEDGE_PLUGIN_IMAGE=ON \
  -GNinja ..
ninja
ninja install # might need `sudo`
```

## Run this repository

By the command:

```shell
cargo build --target wasm32-wasi --release
WASMEDGE_PLUGIN_PATH=/usr/local/lib/wasmedge/ wasmedge --dir ./. target/wasm32-wasi/release/plugin-example.wasm
```
