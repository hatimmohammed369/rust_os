#![no_std] // NO STANDARD LIBRARY
#![no_main] // OVERWRITING THE ENTRY POINT OF THIS BINARY

use core::panic::PanicInfo;

/// This function is called on panic, it's the new panic handler
/// the default was provided by the standard library
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle] // make sure rust compiler does not mangle (change) _start name
// without this attribute the compiler will output something like
// _ZN3blog_os4_start7hb173fedf945531caE
// We need _start to be exported as _start, no change in name
// because the linker needs to know its name
// We also have to mark the function as extern "C" to tell the compiler that it should use the C
// calling convention for this function (instead of the unspecified Rust calling convention). The
// reason for naming the function _start is that this is the default entry point name for most
// systems.
// (-> !) part means this function is not allowed to return because
// it's called by no other function but invoked directly by the OS
pub extern "C" fn _start() -> ! {
    loop {}
}
