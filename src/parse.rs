use crate::x64::*;

pub fn size(c: u8) -> Size {
    match c {
        b'b' => Size::Byte,
        b'w' => Size::Word,
        b'l' => Size::Long,
        b'q' => Size::Quad,
        _ => panic!("invalid size"),
    }
}

pub fn reg(s: &[u8]) -> Reg {
    match s[1].is_ascii_digit() {
        true => match s[1] {
            b'8' => Reg::R8,
            b'9' => Reg::R9,
            _ => match s[2] {
                b'0' => Reg::R10,
                b'1' => Reg::R11,
                b'2' => Reg::R12,
                b'3' => Reg::R13,
                b'4' => Reg::R14,
                b'5' => Reg::R15,
                _ => panic!("invalid r reg"),
            },
        },
        false => match s[1] {
            b'a' => Reg::RAX,
            b'c' => Reg::RCX,
            b'd' => match s[2] {
                b'i' => Reg::RDI,
                b'x' | b'l' => Reg::RDX,
                _ => panic!("invalid d reg"),
            },
            b'b' => match s[2] {
                b'p' => Reg::RBP,
                b'x' | b'l' => Reg::RBX,
                _ => panic!("invalid b reg"),
            },
            b's' => match s[2] {
                b'p' => Reg::RSP,
                b'i' => Reg::RSI,
                _ => panic!("invalid s reg"),
            },
            _ => panic!("invalid reg"),
        },
    }
}

pub fn op(s: &[u8]) -> Op {
    match s {
        b"mov" => Op::Mov,
        b"syscall" => Op::Syscall,
        s if s.len() == 2 && s[0] == b'd' => match s[1] {
            b'b' => Op::Data(Size::Byte),
            b'w' => Op::Data(Size::Word),
            b'd' => Op::Data(Size::Long),
            b'q' => Op::Data(Size::Quad),
            b's' => Op::DataString,
            _ => panic!("invalid data"),
        },
        _ => panic!("invalid op"),
    }
}

pub fn imm(s: &[u8]) -> u64 {
    let s = unsafe { std::str::from_utf8_unchecked(s) };
    s.parse().unwrap()
}
