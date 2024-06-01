mod elf;
mod parse;
mod utils;
mod x64;

use utils::bytes::*;
use x64::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = std::time::Instant::now();
    let args = std::env::args().collect::<Vec<_>>();
    let cmd = args.get(1).expect("usage: z <cmd>");
    let args = &args[2..];

    match cmd.as_str() {
        "asm" => asm(args)?,
        cmd => panic!("unknown command: {cmd}"),
    }

    println!("finished in {:?}", start.elapsed());
    Ok(())
}

fn asm(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let input = args.get(0).expect("usage: z asm <input.z> <output>");
    let output = args.get(1).expect("usage: z asm <input.z> <output>");

    let src = std::fs::read(&input)?;
    let lines = src
        .split(|&c| c == b'\n')
        .map(|s| s.trim_ascii_start())
        .filter(|s| !s.is_empty());

    let mut labels = vec![];
    let mut code = vec![];
    for line in lines {
        compile(line, &mut labels, &mut code);
    }

    let bin = elf::link(&code);
    std::fs::write(output, &bin)?;

    Ok(())
}

fn compile(line: &[u8], labels: &mut Vec<(Vec<u8>, usize)>, code: &mut Vec<u8>) {
    let mut args = line.split(|&c| c == b' ');

    let arg = args.next().expect("missing op");

    match arg[0] {
        b'.' => {
            println!("Label {}", code.len());
            labels.push((arg[1..arg.len()].to_vec(), code.len()));
        }
        _ => {
            let op = parse::op(arg);
            match op {
                Op::Mov => {
                    let arg0 = args.next().expect("mov: missing arg0");
                    let arg1 = args.next().expect("mov: missing arg1");

                    match arg0[1] {
                        b'(' => {
                            let size = parse::size(arg0[0]);
                            let dst = parse::reg(&arg0[2..arg0.len()]);
                            unimplemented!("mov ({dst:?})/{size:?}")
                        }
                        _ => {
                            let dst = parse::reg(arg0);

                            match arg1[0] {
                                c if c.is_ascii_digit() => {
                                    let imm = parse::imm(arg1);

                                    println!("{op:?} {dst:?} {imm}");
                                    ds(code, &[rex::W | rex::reg(dst), 0xb8 | modrm::rm(dst)]);
                                    dq(code, imm);
                                }
                                b'.' => {
                                    let imm = parse::imm(arg1);

                                    println!("{op:?} {dst:?} {imm}");
                                    ds(code, &[rex::W | rex::reg(dst), 0xb8 | modrm::rm(dst)]);
                                    dq(code, imm);
                                }
                                _ => {
                                    let src = parse::reg(arg1);

                                    println!("{op:?} {dst:?} {src:?}");
                                    ds(
                                        code,
                                        &[
                                            rex::W | rex::reg(dst) | rex::rm(src),
                                            0x8b,
                                            modrm::MOD_REG | modrm::reg(dst) | modrm::rm(src),
                                        ],
                                    )
                                }
                            }
                        }
                    }
                }
                Op::Syscall => ds(code, &[0x0f, 0x05]),

                Op::Data(size) => {
                    let arg0 = args.next().expect("data: missing arg0");
                    let imm = parse::imm(arg0);
                    match size {
                        Size::Byte => db(code, imm as u8),
                        Size::Word => dw(code, imm as u16),
                        Size::Long => dd(code, imm as u32),
                        Size::Quad => dq(code, imm),
                    }
                }
                #[rustfmt::skip]
                Op::DataString => {
                    let start = line.iter().position(|&c| c == b'"').expect("data: missing start quote");
                    let end = line.iter().rposition(|&c| c == b'"').expect("data: missing end quote");
                    let s = &line[(start+1)..=(end-1)];

                    // dq(code, s.len() as u64);
                    ds(code, s);
                }
            }
        }
    }
}
