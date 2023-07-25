
#![no_std]
#![no_main]
#![feature(lang_items)]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

pub trait Termination {
    fn report(self) -> i32;
}

impl Termination for () {
    fn report(self) -> i32 {
        0
    }
}

#[lang = "start"]
fn start<T: Termination + 'static>(
    _main: fn() -> T,
    _argc: isize,
    _argv: *const *const u8,
    _uk: u8
) -> isize {
    // 42
    main();
    0
}

#[no_mangle]
fn __wasm_call_dtors() {}

#[no_mangle]
fn __wasi_proc_exit(_param: i32) {}

extern "C" {
    fn printf(input: *const u8, input2: *const u8) -> i32;
}

#[no_mangle]
fn main() {
    let hello = "Hello ";
    let hello_ptr: *const u8 = hello.as_ptr() as *const u8;

    let msg = "Rust World!\n";
    let msg_ptr: *const u8 = msg.as_ptr() as *const u8;

    unsafe {
        printf(hello_ptr, msg_ptr);
    }
}
