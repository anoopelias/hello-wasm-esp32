
#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

extern "C" {
    fn puts(input: *const u8) -> i32;
}

#[no_mangle]
fn main() {
    let hello = "Hello Rust World!";
    let hello_ptr: *const u8 = hello.as_ptr() as *const u8;

    unsafe {
        puts(hello_ptr);
    }
}
