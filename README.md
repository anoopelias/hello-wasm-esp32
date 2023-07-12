# "Hello World!" on ESP32 using WASM 
This is an attempt to run WASM (Web Assembly) over ESP32 using the [wasm-micro-runtime](https://github.com/bytecodealliance/wasm-micro-runtime) project.

We will stick to a simple hello world.

## Status

- [ ] WASM built using clang
- [ ] WASM built using rustc

## Instructions

### Hardware
You would need a development board. I used an ESP32 development board. Something like [this](https://robocraze.com/products/nodemcu-32-wifi-bluetooth-esp32-development-board30-pin). Theoretically any ESP32 board will suffice.

Also you would need mac / Linux / Windows computer as well for cross compilation and tooling. For my purposes I am using a Raspberry Pi 3B.

### Software setup
- Setup `esp-idf` toolchain using the instructions [here](https://docs.espressif.com/projects/esp-idf/en/latest/esp32/get-started/linux-macos-setup.html). Make sure that you add `IDF_PATH` to your user profile as mentioned [here](https://docs.espressif.com/projects/esp-idf/en/v3.3.1/get-started/add-idf_path-to-profile.html).
- Clone `wasm-micro-runtime` from [here](https://github.com/bytecodealliance/wasm-micro-runtime). Set `WAMR_ROOT_DIR` to this path in your user profile.

### Usage

```
$ git clone git@github.com:anoopelias/hello-wasm-esp32.git
$ cd hello-wasm-esp32
$ export $IDF_PATH/export.sh
$ ./build_and_run.sh
```

Once this is complete, if you run 

```
$ idf.py monitor
```

You should be able to see relevant "Hello World" and memory locations printed in terminal.
