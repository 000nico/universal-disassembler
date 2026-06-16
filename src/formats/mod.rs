// mods
pub mod raw;
pub mod pe;
pub mod elf;
#[path = "mach-o.rs"]
pub mod macho;

// shared variables
#[derive(Debug)]
pub enum Format {
    ELF,
    MachO,
    PE,
    Raw,
}

// functions
pub fn detect_format(aob: &[u8]) -> Format {
    if aob.starts_with(&[0x4D, 0x5A]){
        Format::PE
    } 
    else if aob.starts_with(&[0x7F, 0x45, 0x4C, 0x46]){
        Format::ELF
    }
    else if aob.starts_with(&[0xCE, 0xFA, 0xED, 0xFE]) ||
            aob.starts_with(&[0xCF, 0xFA, 0xED, 0xFE]) ||
            aob.starts_with(&[0xFE, 0xED, 0xFA, 0xCE]) || 
            aob.starts_with(&[0xFE, 0xED, 0xFA, 0xCF]) {
        Format::MachO
    }
    else {
        Format::Raw
    }
}

fn read_u16(aob: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes(aob[offset..offset+2].try_into().unwrap())
}

fn read_u32(aob: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(aob[offset..offset+4].try_into().unwrap())
}

fn read_u64(aob: &[u8], offset: usize) -> u64 {
    u64::from_le_bytes(aob[offset..offset+8].try_into().unwrap())
}

fn read_u8(aob: &[u8], offset: usize) -> u8 {
    aob[offset]
}