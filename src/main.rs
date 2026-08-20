#![no_std]
#![no_main]

mod custom;
mod exit;

#[no_mangle]
#[link_section = ".text.entry"]
pub unsafe extern "C" fn _start() -> ! {
    // Initialize stack pointer to the top of RAM region (0x80010000 + 64K)
    core::arch::asm!(
        "li sp, 0x80020000",
        options(nostack)
    );

    main();

    // Signal successful completion
    exit::trigger_exit(exit::EXIT_SUCCESS);
}

fn main() {
    // Execute a test call to your custom hardware extension
    let val1: u64 = 12;
    let val2: u64 = 34;
    let _result = unsafe { custom::custom_0_op(val1, val2) };
}
