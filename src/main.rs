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

const LABEL_MAGIC: i32 = 0x789AB000; // enough for 4096 labels
const LABEL_MASK: i32 = 0x00000FFF;

fn asm(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let input = args.get(0).expect("usage: z asm <input.z> <output>");
    let output = args.get(1).expect("usage: z asm <input.z> <output>");

    let src = std::fs::read(&input)?;
    let lines = src
        .split(|&c| c == b'\n')
        .map(|s| s.trim_ascii_start())
        .filter(|s| !s.is_empty());

    let mut labels = vec![];
    for line in lines.clone() {
        if line[0] == b'.' {
            labels.push((line.to_vec(), 0));
        }
    }

    let mut code = vec![];
    let mut n_labels = 0;
    for line in lines {
        let mut args = line.split(|&c| c == b' ');
        let arg = args.next().expect("missing op");

        match arg[0] {
            b'.' => {
                println!("Label {}", code.len());
                labels[n_labels].1 = code.len() as i32;
                n_labels += 1;
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
                                        ds(
                                            &mut code,
                                            &[rex::W | rex::reg(dst), 0xb8 | modrm::rm(dst)],
                                        );
                                        dq(&mut code, imm);
                                    }
                                    b'.' => {
                                        let label_i = labels
                                            .iter()
                                            .position(|(label, _)| label == arg1)
                                            .expect("no such label")
                                            as i32;

                                        ds(
                                            &mut code,
                                            &[
                                                rex::W | rex::reg(dst),
                                                0x8d,
                                                0b00000101 | modrm::reg(dst),
                                            ],
                                        );

                                        dd(&mut code, (LABEL_MAGIC | (label_i & 0xFFF)) as u32);
                                    }
                                    _ => {
                                        let src = parse::reg(arg1);

                                        println!("{op:?} {dst:?} {src:?}");
                                        ds(
                                            &mut code,
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
                    Op::Syscall => ds(&mut code, &[0x0f, 0x05]),

                    Op::Data(size) => {
                        let arg0 = args.next().expect("data: missing arg0");
                        let imm = parse::imm(arg0);
                        match size {
                            Size::Byte => db(&mut code, imm as u8),
                            Size::Word => dw(&mut code, imm as u16),
                            Size::Long => dd(&mut code, imm as u32),
                            Size::Quad => dq(&mut code, imm),
                        }
                    }
                    #[rustfmt::skip]
                    Op::DataString => {
                        let start = line.iter().position(|&c| c == b'"').expect("data: missing start quote");
                        let end = line.iter().rposition(|&c| c == b'"').expect("data: missing end quote");
                        let s = &line[(start+1)..=(end-1)];
                        ds(&mut code, s);
                    }
                }
            }
        }
    }

    for i in 0..(code.len() - 3) {
        let word: &mut i32 = unsafe { &mut *(code.as_mut_ptr().wrapping_add(i) as *mut i32) };

        if *word & !LABEL_MASK == LABEL_MAGIC {
            let label_i = *word & LABEL_MASK;

            let label_ptr = labels[label_i as usize].1;
            let next_instr_ptr = (i + 4) as i32; // skip the imm32 itself
            *word = label_ptr - next_instr_ptr;
        }
    }

    let bin = elf::link(&code);
    std::fs::write(output, &bin)?;

    Ok(())
}
