//! Custom-0 instruction wrappers for the XZXT extension.

/// Custom-0 Type-R instruction wrapper.
#[inline(always)]
pub unsafe fn custom_0_op(rs1: u64, rs2: u64) -> u64 {
    let rd: u64;
    
    const OPCODE: u32 = 0x0B; // Standard custom-0 opcode
    const FUNCT3: u32 = 0x0;  // Update to match cpu_pkg.vl
    const FUNCT7: u32 = 0x00; // Update to match cpu_pkg.vl

    core::arch::asm!(
        ".insn r {opcode}, {f3}, {f7}, {rd}, {rs1}, {rs2}",
        opcode = const OPCODE,
        f3     = const FUNCT3,
        f7     = const FUNCT7,
        rd     = out(reg) rd,
        rs1    = in(reg) rs1,
        rs2    = in(reg) rs2,
    );
    rd
}
