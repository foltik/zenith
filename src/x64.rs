#[derive(Debug, Clone, Copy)]
pub enum Op {
    Mov,
    Syscall,
}

#[derive(Debug, Clone, Copy)]
pub enum Reg {
    RAX,
    RCX,
    RDX,
    RBX,
    RSP,
    RBP,
    RSI,
    RDI,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Size {
    Byte,
    Word,
    Long,
    Quad,
}

#[rustfmt::skip]
pub mod rex {
    use super::*;

    const REX: u8 = 0b0100_0000;
    pub const W: u8 = REX | 0b1000;

    pub fn reg(reg: Reg) -> u8 { reg.rex_bit() << 2 }
    pub fn rm(reg: Reg)  -> u8 { reg.rex_bit() }
}

#[rustfmt::skip]
pub mod modrm {
    use super::*;

    pub const MOD_REG: u8 = 0b1100_0000;

    pub fn reg(reg: Reg) -> u8 { reg.modrm_bits() << 3 }
    pub fn rm(reg: Reg)  -> u8 { reg.modrm_bits() }
}

#[rustfmt::skip]
impl Reg {
    pub fn modrm_bits(self) -> u8 { self as u8 & 0b0111 }
    pub fn rex_bit(self)    -> u8 { self as u8 >> 3 }
}
