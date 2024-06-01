use crate::utils::bytes::*;

// Base virtual address and alignment
const VA: u64 = 0xF000_0000;
const ALIGN: u64 = 2 * 1024 * 1024;

// Sizes of elf structs
const EHDR_SZ: u64 = 64; // sizeof(Elf64_Ehdr)
const PHDR_SZ: u64 = 56; // sizeof(Elf64_Phdr)
const SHDR_SZ: u64 = 64; // sizeof(Elf64_Shdr)

// Offsets of header sections in the binary (all are placed sequentially).
const PHDR_OFS: u64 = 0 + EHDR_SZ;
const SHDR_OFS: u64 = PHDR_OFS + PHDR_SZ;
const STRTAB_OFS: u64 = SHDR_OFS + (3 * SHDR_SZ);
const CODE_OFS: u64 = STRTAB_OFS + STRTAB_SZ;

pub fn link(code: &[u8]) -> Vec<u8> {
    let code_sz = code.len() as u64;
    let image_sz = CODE_OFS + code_sz;

    let mut buf = Vec::with_capacity(image_sz as usize);

    write_elf_header(&mut buf);
    write_program_header(&mut buf, image_sz);
    write_reserved_section(&mut buf);
    write_text_section(&mut buf, code_sz);
    write_strtab_section(&mut buf);
    write_strtab(&mut buf);
    ds(&mut buf, code);

    buf
}

#[rustfmt::skip]
fn write_elf_header(buf: &mut Vec<u8>) {
    ds(buf, b"\x7fELF");     // EI_MAG
    db(buf, 2);              // EI_CLASS = ELFCLASS64
    db(buf, 1);              // EI_DATA = ELFDATA2LSB
    db(buf, 1);              // EI_VERSION = EV_CURRENT
    db(buf, 0);              // EI_OSABI = ELFOSABI_SYSV
    dq(buf, 0);              // EI_ABIVERSION = 0
    dw(buf, 2);              // e_type = ET_EXEC
    dw(buf, 62);             // e_machine = EM_X86_64
    dd(buf, 1);              // e_version = EV_CURRENT
    dq(buf, VA + CODE_OFS);  // e_entry
    dq(buf, PHDR_OFS);       // e_phoff
    dq(buf, SHDR_OFS);       // e_shoff
    dd(buf, 0);              // e_flags = n/a
    dw(buf, EHDR_SZ as u16); // e_ehsize = sizeof(Elf64_Ehdr)
    dw(buf, PHDR_SZ as u16); // e_phentsize = sizeof(Elf64_Phdr)
    dw(buf, 1);              // e_phnum
    dw(buf, SHDR_SZ as u16); // e_shentsize = sizeof(Elf64_Shdr)
    dw(buf, 3);              // e_shnum
    dw(buf, 2);              // e_shstrndx = elf_shdr[2]
}

#[rustfmt::skip]
fn write_program_header(buf: &mut Vec<u8>, file_sz: u64) {
    dd(buf, 1);        // p_type = PT_LOAD
    dd(buf, 5);        // p_flags = PF_R | PF_X
    // dd(f, 7);     // p_flags = PF_R | PF_W | PF_X
    dq(buf, 0);        // p_offset
    dq(buf, VA);       // p_vaddr
    dq(buf, 0);        // p_paddr
    dq(buf, file_sz);  // p_filesz
    dq(buf, file_sz);  // p_memsz
    dq(buf, ALIGN);    // p_align
}

fn write_reserved_section(buf: &mut Vec<u8>) {
    ds(buf, &[0; SHDR_SZ as usize]);
}

#[rustfmt::skip]
fn write_text_section(buf: &mut Vec<u8>, code_sz: u64) {
    dd(buf, 0);             // sh_name = strtab[0]
    dd(buf, 1);             // sh_type = SHT_PROGBITS
    dq(buf, 6);             // sh_flags = SHF_ALLOC | SHF_EXECINSTR
    dq(buf, VA + CODE_OFS); // sh_addr
    dq(buf, CODE_OFS);      // sh_offset
    dq(buf, code_sz);       // sh_size
    dd(buf, 0);             // sh_link
    dd(buf, 0);             // sh_info
    dq(buf, ALIGN);         // sh_addralign
    dq(buf, 0);             // sh_entsize
}

#[rustfmt::skip]
fn write_strtab_section(buf: &mut Vec<u8>) {
    dd(buf, 6);               // sh_name = strtab[1]
    dd(buf, 3);               // sh_type = SHT_STRTAB
    dq(buf, 0);               // sh_flags
    dq(buf, VA + STRTAB_OFS); // sh_addr
    dq(buf, STRTAB_OFS);      // sh_offset
    dq(buf, STRTAB_SZ);       // sh_size
    dd(buf, 0);               // sh_link
    dd(buf, 0);               // sh_info
    dq(buf, 0);               // sh_addralign
    dq(buf, 0);               // sh_entsize
}

const STRTAB_SZ: u64 = 16;
fn write_strtab(buf: &mut Vec<u8>) {
    ds(buf, b".text\0");
    ds(buf, b".shstrtab\0");
}
