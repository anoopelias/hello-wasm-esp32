
#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

extern "C" {
    fn printf(input: *const u8, input2: *const u8) -> i32;
}

#[no_mangle]
fn main() {
    let hello = "Hello ";
    let hello_ptr: *const u8 = hello.as_ptr() as *const u8;

    let msg = "Rust 4 World!\n";
    let msg_ptr: *const u8 = msg.as_ptr() as *const u8;

    unsafe {
        printf(hello_ptr, msg_ptr);
    }
}
