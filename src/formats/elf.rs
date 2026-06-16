use crate::formats::{read_u8, read_u16, read_u32, read_u64};

#[derive(Default)]
pub struct ELFHeader32 {
    pub magic: [u8; 4],
    pub class: u8,
    pub data: u8,
    pub version: u8,
    pub os_abi: u8,
    pub padding: [u8; 8],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u32,
    pub e_phoff: u32, // offset program headers
    pub e_shoff: u32, // offsets section headers
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentisze: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16
}

#[derive(Default)]
pub struct ELFHeader64 {
    pub magic: [u8; 4],
    pub class: u8,
    pub data: u8,
    pub version: u8,
    pub os_abi: u8,
    pub padding: [u8; 8],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u64,
    pub e_phoff: u64, // offset program headers
    pub e_shoff: u64, // offsets section headers
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentisze: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16
}

#[derive(Default)]
pub struct ProgramHeader32 {
    pub p_type: u32,
    pub p_offset: u32,
    pub p_vaddr: u32,
    pub p_paddr: u32,
    pub p_filesz: u32,
    pub p_memz: u32,
    pub p_flags: u32,
    pub p_align: u32
}

#[derive(Default)]
pub struct ProgramHeader64 {
    pub p_type: u32,
    pub p_offset: u32,
    pub p_vaddr: u64,
    pub p_paddr: u64,
    pub p_filesz: u64,
    pub p_memz: u64,
    pub p_flags: u64,
    pub p_align: u64
}

#[derive(Default)]
pub struct SectionHeader32 {
    pub sh_name: u32,
    pub sh_type: u32,
    pub sh_flags: u32,
    pub sh_addr: u32,
    pub sh_offset: u32,
    pub sh_size: u32,
    pub sh_link: u32,
    pub sh_info: u32,
    pub sh_addralign: u32,
    pub sh_entsize: u32
}

#[derive(Default)]
pub struct SectionHeader64 {
    pub sh_name: u32,
    pub sh_type: u32,
    pub sh_flags: u64,
    pub sh_addr: u64,
    pub sh_offset: u64,
    pub sh_size: u64,
    pub sh_link: u32,
    pub sh_info: u32,
    pub sh_addralign: u64,
    pub sh_entsize: u64
}

#[derive(Default)]
pub struct ELF32 {
    pub elf_header: ELFHeader32,
    pub program_header: Vec<ProgramHeader32>,
    pub sections: Vec<SectionHeader32>,
}

#[derive(Default)]
pub struct ELF64 {
    pub elf_header: ELFHeader64,
    pub program_header: Vec<ProgramHeader64>,
    pub sections: Vec<SectionHeader64>,
}

pub enum ELF {
    ELF32(ELF32),
    ELF64(ELF64)
}

fn parse_elf_header_32(aob: &[u8]) -> Result<ELFHeader32, String> {
    let mut eh32 = ELFHeader32::default();

    for i in 0..4 {
        eh32.magic[i] = read_u8(aob, 0x00 + i * 2);
    }

    eh32.class = read_u8(aob, 0x04);
    eh32.data = read_u8(aob, 0x05);
    eh32.version = read_u8(aob, 0x06);
    eh32.os_abi = read_u8(aob, 0x07);
    
    for i in 0..8 {
        eh32.padding[i] = read_u8(aob, 0x08 + i * 2);
    }

    eh32.e_type = read_u16(aob, 0x10);
    eh32.e_machine = read_u16(aob, 0x12);
    eh32.e_version = read_u32(aob, 0x14);
    eh32.e_entry = read_u32(aob, 0x18);
    eh32.e_phoff = read_u32(aob, 0x1C);
    eh32.e_shoff = read_u32(aob, 0x20);
    eh32.e_flags = read_u32(aob, 0x24);
    eh32.e_ehsize = read_u16(aob, 0x28);
    eh32.e_phentisze = read_u16(aob, 0x2A);
    eh32.e_phnum = read_u16(aob, 0x2C);
    eh32.e_shentsize = read_u16(aob, 0x2E);
    eh32.e_shnum = read_u16(aob, 0x30);
    eh32.e_shstrndx = read_u16(aob, 0x32);

    Ok(eh32)
}

fn parse_elf_header_64(aob: &[u8]) -> Result<ELFHeader64, String> {
    let mut eh64 = ELFHeader64::default();

    for i in 0..4 {
        eh64.magic[i] = read_u8(aob, 0x00 + i * 2);
    }

    eh64.class = read_u8(aob, 0x04);
    eh64.data = read_u8(aob, 0x05);
    eh64.version = read_u8(aob, 0x06);
    eh64.os_abi = read_u8(aob, 0x07);
    
    for i in 0..8 {
        eh64.padding[i] = read_u8(aob, 0x08 + i * 2);
    }

    eh64.e_type = read_u16(aob, 0x10);
    eh64.e_machine = read_u16(aob, 0x12);
    eh64.e_version = read_u32(aob, 0x14);
    eh64.e_entry = read_u64(aob, 0x18);
    eh64.e_phoff = read_u64(aob, 0x20);
    eh64.e_shoff = read_u64(aob, 0x28);
    eh64.e_flags = read_u32(aob, 0x30);
    eh64.e_ehsize = read_u16(aob, 0x34);
    eh64.e_phentisze = read_u16(aob, 0x36);
    eh64.e_phnum = read_u16(aob, 0x38);
    eh64.e_shentsize = read_u16(aob, 0x3A);
    eh64.e_shnum = read_u16(aob, 0x3C);
    eh64.e_shstrndx = read_u16(aob, 0x3E);

    Ok(eh64)
}

fn parse_program_header_32(aob: &[u8], offset: usize) -> Result<ProgramHeader32, String> {
    let mut ph32 = ProgramHeader32::default();

    ph32.p_type = read_u32(aob, offset + 0x00);
    ph32.p_offset = read_u32(aob, offset + 0x04);
    ph32.p_vaddr = read_u32(aob, offset + 0x08);
    ph32.p_paddr = read_u32(aob, offset + 0x0C);
    ph32.p_filesz = read_u32(aob, offset + 0x10);
    ph32.p_memz = read_u32(aob, offset + 0x14);
    ph32.p_flags = read_u32(aob, offset + 0x18);
    ph32.p_align = read_u32(aob, offset + 0x1C);

    Ok(ph32)
}

fn parse_program_header_64(aob: &[u8], offset: usize) -> Result<ProgramHeader64, String> {
    let mut ph64 = ProgramHeader64::default();

    ph64.p_type = read_u32(aob, offset + 0x00);
    ph64.p_offset = read_u32(aob, offset + 0x04);
    ph64.p_vaddr = read_u64(aob, offset + 0x08);
    ph64.p_paddr = read_u64(aob, offset + 0x10);
    ph64.p_filesz = read_u64(aob, offset + 0x18);
    ph64.p_memz = read_u64(aob, offset + 0x20);
    ph64.p_flags = read_u64(aob, offset + 0x28);
    ph64.p_align = read_u64(aob, offset + 0x30);

    Ok(ph64)
}

fn parse_section_header32(aob: &[u8], offset: usize) -> Result<SectionHeader32, String> {
    let mut sh32 = SectionHeader32::default();

    sh32.sh_name = read_u32(aob, offset + 0x00);
    sh32.sh_type = read_u32(aob, offset + 0x04);
    sh32.sh_flags = read_u32(aob, offset + 0x08);
    sh32.sh_addr = read_u32(aob, offset + 0x0C);
    sh32.sh_offset = read_u32(aob, offset + 0x10);
    sh32.sh_size = read_u32(aob, offset + 0x14);
    sh32.sh_link = read_u32(aob, offset + 0x18);
    sh32.sh_info = read_u32(aob, offset + 0x1C);
    sh32.sh_addralign = read_u32(aob, offset + 0x20);
    sh32.sh_entsize = read_u32(aob, offset + 0x24);

    Ok(sh32)
}

fn parse_section_header64(aob: &[u8], offset: usize) -> Result<SectionHeader64, String> {
    let mut sh64 = SectionHeader64::default();

    sh64.sh_name = read_u32(aob, offset + 0x00);
    sh64.sh_type = read_u32(aob, offset + 0x04);
    sh64.sh_flags = read_u64(aob, offset + 0x08);
    sh64.sh_addr = read_u64(aob, offset + 0x0C);
    sh64.sh_offset = read_u64(aob, offset + 0x10);
    sh64.sh_size = read_u64(aob, offset + 0x14);
    sh64.sh_link = read_u32(aob, offset + 0x18);
    sh64.sh_info = read_u32(aob, offset + 0x1C);
    sh64.sh_addralign = read_u64(aob, offset + 0x20);
    sh64.sh_entsize = read_u64(aob, offset + 0x24);

    Ok(sh64)
}

pub fn parse_elf(aob: &[u8]) -> Result<ELF, String> {
    match aob[0x04] {
        /*32 bits*/ 1 => {
            let mut elf32 = ELF32::default();

            elf32.elf_header = parse_elf_header_32(aob)?;

            for i in 0..elf32.elf_header.e_phnum as usize { 
                let offset = elf32.elf_header.e_phoff as usize + (i * 0x20);
                elf32.program_header.push(parse_program_header_32(aob, offset)?);
            }

            for i in 0..elf32.elf_header.e_shnum as usize { 
                let offset = elf32.elf_header.e_shoff as usize + (i * 0x20);
                elf32.sections.push(parse_section_header32(aob, offset)?);
            }

            Ok(ELF::ELF32(elf32))
        }

        /*64 bits*/ 2 => {
            let mut elf64 = ELF64::default();

            elf64.elf_header = parse_elf_header_64(aob)?;

            for i in 0..elf64.elf_header.e_phnum as usize { 
                let offset = elf64.elf_header.e_phoff as usize + (i * 0x20);
                elf64.program_header.push(parse_program_header_64(aob, offset)?);
            }

            for i in 0..elf64.elf_header.e_shnum as usize { 
                let offset = elf64.elf_header.e_shoff as usize + (i * 0x20);
                elf64.sections.push(parse_section_header64(aob, offset)?);
            }

            Ok(ELF::ELF64(elf64))
        }

        /*unknown*/ _ => {
            Err(String::from("UNKOWN_ELF_TYPE"))
        }
    }
}
