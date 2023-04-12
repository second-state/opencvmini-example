# opencvmini example

opencvmini is not released plugin yet, you will need to checkout branch `plugin-opencvmini` of repository [WasmEdge](https://github.com/WasmEdge/WasmEdge), and run the following commands to install this special version:

```shell
# In repository wasmedge
mkdir build && cd build
cmake \
  -DCMAKE_BUILD_TYPE=Release \
  -DWASMEDGE_BUILD_PLUGINS=ON \
  -DWASMEDGE_PLUGIN_OPENCVMINI=ON \
  ..
make -j
cmake --install . # might need `sudo`
```

Then in this repository runs

```shell
make
```
