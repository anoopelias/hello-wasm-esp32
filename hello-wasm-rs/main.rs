
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
fn main() {
    let to = "Hello rust world2\n";
    let st: *const u8 = to.as_ptr() as *const u8;

    unsafe {
        printf(st, st);
    }
}

#[no_mangle]
fn __wasm_call_dtors() {}

#[no_mangle]
fn __wasi_proc_exit(_param: i32) {}

extern "C" {
    fn printf(input: *const u8, input2: *const u8) -> i32;
}
