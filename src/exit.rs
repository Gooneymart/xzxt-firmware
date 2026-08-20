//! Exit routines, simulation hooks, and panic handlers.

use core::panic::PanicInfo;

const EXIT_ADDR: *mut u64 = 0x80001000 as *mut u64;

pub const EXIT_SUCCESS: u64 = 1;
pub const EXIT_FAILURE: u64 = 2;

/// Triggers the simulation exit trap with a specific status code.
pub fn trigger_exit(code: u64) -> ! {
    unsafe {
        core::ptr::write_volatile(EXIT_ADDR, code);
    }
    loop {
        unsafe {
            core::arch::asm!("wfi");
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // Signal failure via exit trap if a panic occurs
    trigger_exit(EXIT_FAILURE);
}
