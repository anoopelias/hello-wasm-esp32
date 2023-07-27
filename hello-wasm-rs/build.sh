# 32kb stack size
rustc -C link-self-contained=no -C link-args=-zstack-size=32768 -C link-args=--no-entry --target wasm32-wasi main.rs

echo "Build binarydump tool .."
rm -fr build && mkdir build && cd build
cmake $WAMR_PATH/test-tools/binarydump-tool
make
cd ..

echo "Generate test_wasm.h .."
./build/binarydump -o ../main/test_wasm.h -n wasm_test_file_interp main.wasm

echo "Done"


