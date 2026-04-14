#![no_std] // dont use rust standard library
#![no_main] // dont use rust's main function

use core::panic::PanicInfo;
static HELLO: &[u8] = b"Hello, World!";


#[unsafe(no_mangle)] //dont mange the name of the function
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8; 

    for(i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte; // write the byte to the VGA buffer
            *vga_buffer.offset(i as isize * 2 + 1) = 0x0f; // set the color to white on black
        }
    }
    loop {}
}

// This function is called when the program panics. It is required by the Rust language, but we can define it to do nothing.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}