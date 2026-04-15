#![no_std] // dont use rust standard library
#![no_main] // dont use rust's main function

use core::panic::PanicInfo;
mod vga_buffer;
static HELLO: &[u8] = b"Hello, World!";


#[unsafe(no_mangle)] //dont mange the name of the function
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();

    loop {}
}

// This function is called when the program panics. It is required by the Rust language, but we can define it to do nothing.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}