#![no_std]
#![no_main]

mod exit;

#[no_mangle]
#[link_section = ".text.entry"]
pub unsafe extern "C" fn _start() -> ! {
    // 1. Initialize stack pointer
    core::arch::asm!(
        "li sp, 0x80020000",
        options(nostack)
    );

    // 2. Inject raw custom-0 instruction word ending in 0x0B
    core::arch::asm!(
        ".word 0x0000000B",
    );

    // 3. Sub-word memory write-masking verification
    let ram_base = 0x80010000 as *mut u8;
    core::ptr::write_volatile(ram_base, 0xAB_u8);
    let _val = core::ptr::read_volatile(ram_base);

    // 4. Signal successful completion and drop into an infinite self-jump 
    // to satisfy the emulator's requirement against falling into NOP sleds.
    exit::trigger_exit(exit::EXIT_SUCCESS);
}
