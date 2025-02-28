#![no_std]
#![no_main]

use core::panic::PanicInfo;
use nextkernel; // Import kernel functions
use gui;

/// Panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
 loop {}
}

/// OS entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
 // Initialize kernel (calls nextkernel/src/boot.rs)
 nextkernel::start_kernel();

 // Start GUI
 gui::gui().expect("GUI failed");

 loop {}
}
