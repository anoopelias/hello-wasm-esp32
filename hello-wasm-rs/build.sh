# 32kb stack size
rustc -C link-self-contained=no -C link-args=-zstack-size=32768 -C link-args=--no-entry --target wasm32-wasi main.rs

echo "Generate test_wasm.h .."
./build/binarydump -o ../main/test_wasm.h -n wasm_test_file_interp main.wasm

echo "Done"


