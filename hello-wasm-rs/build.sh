rustc +nightly -C link-args=-zstack-size=1024 --target wasm32-wasi main.rs

echo "Build binarydump tool .."
rm -fr build && mkdir build && cd build
cmake $WAMR_PATH/test-tools/binarydump-tool
make
cd ..


echo "Generate test_wasm.h .."
./build/binarydump -o ../main/test_wasm.h -n wasm_test_file_interp main.wasm

echo "Done"


