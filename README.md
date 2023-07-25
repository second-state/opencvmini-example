# opencvmini example

[opencvmini](https://github.com/WasmEdge/WasmEdge/pull/2648) is not released plugin yet, you will need to checkout branch `plugin-opencvmini` of repository [WasmEdge](https://github.com/WasmEdge/WasmEdge), and run the following commands to install this special version:

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

Then run this repository by the command:

```shell
cargo build --target wasm32-wasi --release
WASMEDGE_PLUGIN_PATH=/usr/local/lib/wasmedge/ wasmedge --dir ./. target/wasm32-wasi/release/plugin-example.wasm
```
