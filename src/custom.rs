//! Custom-0 instruction wrappers for the XZXT extension.

/// Custom-0 Sub-opcode variant 0
/// Encodes to: opcode = 0x0B (custom-0), funct3 = 0x0, funct7 = 0x00
#[inline(always)]
pub unsafe fn xzxt_op_v0(rs1: u64, rs2: u64) -> u64 {
    let rd: u64;
    core::arch::asm!(
        ".insn r 0x0B, 0x0, 0x00, {}, {}, {}",
        out(reg) rd,
        in(reg) rs1,
        in(reg) rs2,
    );
    rd
}

/// Custom-0 Sub-opcode variant 1 (alternate funct3 for stress testing)
/// Encodes to: opcode = 0x0B (custom-0), funct3 = 0x1, funct7 = 0x00
#[inline(always)]
pub unsafe fn xzxt_op_v1(rs1: u64, rs2: u64) -> u64 {
    let rd: u64;
    core::arch::asm!(
        ".insn r 0x0B, 0x1, 0x00, {}, {}, {}",
        out(reg) rd,
        in(reg) rs1,
        in(reg) rs2,
    );
    rd
}
