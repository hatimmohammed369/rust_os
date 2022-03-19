#![no_std] // NO STANDARD LIBRARY

use core::panic::PanicInfo;

/// This function is called on panic, it's the new panic handler
/// the default was provided by the standard library
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn main() {
}
