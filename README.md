# "Hello World!" on ESP32 using WASM
This is an attempt to run WASM (Web Assembly) over ESP32 using the [wasm-micro-runtime](https://github.com/bytecodealliance/wasm-micro-runtime) project.

We will stick to a simple hello world.

## Status

- [x] WASM built using clang
- [ ] WASM built using rustc

## Instructions

### Hardware
You would need a development board. I used an ESP32 development board. Something like [this](https://robocraze.com/products/nodemcu-32-wifi-bluetooth-esp32-development-board30-pin). Theoretically any ESP32 board will suffice.

Also you would need mac / Linux / Windows computer as well for cross compilation and tooling. For my purposes I am using a Raspberry Pi 3B.

### Software setup
- Setup `esp-idf` toolchain using the instructions [here](https://docs.espressif.com/projects/esp-idf/en/latest/esp32/get-started/linux-macos-setup.html). Make sure that you add `IDF_PATH` to your user profile as mentioned [here](https://docs.espressif.com/projects/esp-idf/en/v3.3.1/get-started/add-idf_path-to-profile.html).
- Clone `wasm-micro-runtime` from [here](https://github.com/bytecodealliance/wasm-micro-runtime). Set `WAMR_PATH` to this path in your user profile.
- Install `wasi-sdk` as mentioned [here](https://github.com/WebAssembly/wasi-sdk).
    - Its okay to use release downloads but it didn't work for me on Linux. I had to compile it.
    - Set `WASI_SDK_PATH` in your profile correctly to the folder where the artifacts are untar'ed.

### Usage

Remember to bring esp-idf into path,
```
$ export $IDF_PATH/export.sh
```

Clone this repo,
```
$ git clone git@github.com:anoopelias/hello-wasm-esp32.git
$ cd hello-wasm-esp32
```

Compile wasm,
```
$ cd hello-wasm
$ ./build.sh; cd ..
```

And build and flash the esp binary,
```
$ ./build_and_run.sh
```

Once this is complete, if you run
```
$ idf.py monitor
```

You should be able to see relevant "Hello World" printed in terminal.
```
...
...
I (0) cpu_start: Starting scheduler on APP CPU.
I (0) wamr: Initialize WASM runtime
I (10) wamr: Run wamr with interpreter
I (10) wamr: Instantiate WASM runtime
I (10) wamr: run main() of the application
Hello clang world!
I (20) wamr: Deinstantiate WASM runtime
I (30) wamr: Unload WASM module
I (30) wamr: Destroy WASM runtime
I (418) wamr: Exiting...
```
