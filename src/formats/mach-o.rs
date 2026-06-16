use crate::formats::{read_u8, read_u16, read_u32, read_u64};

// Mach-O Format

#[derive(Default)]
pub struct MachOHeader32 {
    pub magic: u32,
    pub cpu_type: u32,
    pub cpu_subtype: u32,
    pub file_type: u32,
    pub n_cmds: u32,       // number of load commands
    pub size_of_cmds: u32, // size of load commands
    pub flags: u32,
}

#[derive(Default)]
pub struct MachOHeader64 {
    pub magic: u32,
    pub cpu_type: u32,
    pub cpu_subtype: u32,
    pub file_type: u32,
    pub n_cmds: u32,
    pub size_of_cmds: u32,
    pub flags: u32,
    pub reserved: u32,
}

#[derive(Default)]
pub struct LoadCommand {
    pub cmd: u32,
    pub cmd_size: u32,
}

#[derive(Default)]
pub struct Section32 {
    pub sect_name: [u8; 16],
    pub seg_name: [u8; 16],
    pub addr: u32,
    pub size: u32,
    pub offset: u32,
    pub align: u32,
    pub rel_off: u32,
    pub n_reloc: u32,
    pub flags: u32,
    pub reserved1: u32,
    pub reserved2: u32,
}

#[derive(Default)]
pub struct Section64 {
    pub sect_name: [u8; 16],
    pub seg_name: [u8; 16],
    pub addr: u64,
    pub size: u64,
    pub offset: u32,
    pub align: u32,
    pub rel_off: u32,
    pub n_reloc: u32,
    pub flags: u32,
    pub reserved1: u32,
    pub reserved2: u32,
    pub reserved3: u32,
}

#[derive(Default)]
pub struct SegmentCommand32 {
    pub cmd: u32,
    pub cmd_size: u32,
    pub seg_name: [u8; 16],
    pub vm_addr: u32,
    pub vm_size: u32,
    pub file_off: u32,
    pub file_size: u32,
    pub max_prot: u32,
    pub init_prot: u32,
    pub n_sects: u32,
    pub flags: u32,
    pub sections: Vec<Section32>,
}

#[derive(Default)]
pub struct SegmentCommand64 {
    pub cmd: u32,
    pub cmd_size: u32,
    pub seg_name: [u8; 16],
    pub vm_addr: u64,
    pub vm_size: u64,
    pub file_off: u64,
    pub file_size: u64,
    pub max_prot: u32,
    pub init_prot: u32,
    pub n_sects: u32,
    pub flags: u32,
    pub sections: Vec<Section64>,
}

#[derive(Default)]
pub struct MachO32 {
    pub header: MachOHeader32,
    pub segments: Vec<SegmentCommand32>,
    pub load_commands: Vec<LoadCommand>,
}

#[derive(Default)]
pub struct MachO64 {
    pub header: MachOHeader64,
    pub segments: Vec<SegmentCommand64>,
    pub load_commands: Vec<LoadCommand>,
}

pub enum MachO {
    MachO32(MachO32),
    MachO64(MachO64),
}

fn parse_macho_header_32(aob: &[u8]) -> Result<MachOHeader32, String> {
    let mut h = MachOHeader32::default();
    h.magic = read_u32(aob, 0x00);
    h.cpu_type = read_u32(aob, 0x04);
    h.cpu_subtype = read_u32(aob, 0x08);
    h.file_type = read_u32(aob, 0x0C);
    h.n_cmds = read_u32(aob, 0x10);
    h.size_of_cmds = read_u32(aob, 0x14);
    h.flags = read_u32(aob, 0x18);
    Ok(h)
}

fn parse_macho_header_64(aob: &[u8]) -> Result<MachOHeader64, String> {
    let mut h = MachOHeader64::default();
    h.magic = read_u32(aob, 0x00);
    h.cpu_type = read_u32(aob, 0x04);
    h.cpu_subtype = read_u32(aob, 0x08);
    h.file_type = read_u32(aob, 0x0C);
    h.n_cmds = read_u32(aob, 0x10);
    h.size_of_cmds = read_u32(aob, 0x14);
    h.flags = read_u32(aob, 0x18);
    h.reserved = read_u32(aob, 0x1C);
    Ok(h)
}

fn parse_section_32(aob: &[u8], base: usize) -> Result<Section32, String> {
    let mut s = Section32::default();
    for i in 0..16 {
        s.sect_name[i] = read_u8(aob, base + i);
    }
    for i in 0..16 {
        s.seg_name[i] = read_u8(aob, base + 0x10 + i);
    }
    s.addr = read_u32(aob, base + 0x20);
    s.size = read_u32(aob, base + 0x24);
    s.offset = read_u32(aob, base + 0x28);
    s.align = read_u32(aob, base + 0x2C);
    s.rel_off = read_u32(aob, base + 0x30);
    s.n_reloc = read_u32(aob, base + 0x34);
    s.flags = read_u32(aob, base + 0x38);
    s.reserved1 = read_u32(aob, base + 0x3C);
    s.reserved2 = read_u32(aob, base + 0x40);
    Ok(s)
}

fn parse_section_64(aob: &[u8], base: usize) -> Result<Section64, String> {
    let mut s = Section64::default();
    for i in 0..16 {
        s.sect_name[i] = read_u8(aob, base + i);
    }
    for i in 0..16 {
        s.seg_name[i] = read_u8(aob, base + 0x10 + i);
    }
    s.addr = read_u64(aob, base + 0x20);
    s.size = read_u64(aob, base + 0x28);
    s.offset = read_u32(aob, base + 0x30);
    s.align = read_u32(aob, base + 0x34);
    s.rel_off = read_u32(aob, base + 0x38);
    s.n_reloc = read_u32(aob, base + 0x3C);
    s.flags = read_u32(aob, base + 0x40);
    s.reserved1 = read_u32(aob, base + 0x44);
    s.reserved2 = read_u32(aob, base + 0x48);
    s.reserved3 = read_u32(aob, base + 0x4C);
    Ok(s)
}

fn parse_segment_command_32(aob: &[u8], base: usize) -> Result<SegmentCommand32, String> {
    let mut seg = SegmentCommand32::default();
    seg.cmd = read_u32(aob, base + 0x00);
    seg.cmd_size = read_u32(aob, base + 0x04);
    for i in 0..16 {
        seg.seg_name[i] = read_u8(aob, base + 0x08 + i);
    }
    seg.vm_addr = read_u32(aob, base + 0x18);
    seg.vm_size = read_u32(aob, base + 0x1C);
    seg.file_off = read_u32(aob, base + 0x20);
    seg.file_size = read_u32(aob, base + 0x24);
    seg.max_prot = read_u32(aob, base + 0x28);
    seg.init_prot = read_u32(aob, base + 0x2C);
    seg.n_sects = read_u32(aob, base + 0x30);
    seg.flags = read_u32(aob, base + 0x34);

    let sections_base = base + 0x38;
    for i in 0..seg.n_sects as usize {
        let section_offset = sections_base + (i * 0x44);
        seg.sections.push(parse_section_32(aob, section_offset)?);
    }

    Ok(seg)
}

fn parse_segment_command_64(aob: &[u8], base: usize) -> Result<SegmentCommand64, String> {
    let mut seg = SegmentCommand64::default();
    seg.cmd = read_u32(aob, base + 0x00);
    seg.cmd_size = read_u32(aob, base + 0x04);
    for i in 0..16 {
        seg.seg_name[i] = read_u8(aob, base + 0x08 + i);
    }
    seg.vm_addr = read_u64(aob, base + 0x18);
    seg.vm_size = read_u64(aob, base + 0x20);
    seg.file_off = read_u64(aob, base + 0x28);
    seg.file_size = read_u64(aob, base + 0x30);
    seg.max_prot = read_u32(aob, base + 0x38);
    seg.init_prot = read_u32(aob, base + 0x3C);
    seg.n_sects = read_u32(aob, base + 0x40);
    seg.flags = read_u32(aob, base + 0x44);

    let sections_base = base + 0x48;
    for i in 0..seg.n_sects as usize {
        let section_offset = sections_base + (i * 0x50);
        seg.sections.push(parse_section_64(aob, section_offset)?);
    }

    Ok(seg)
}

pub fn parse_macho(aob: &[u8]) -> Result<MachO, String> {
    let magic = read_u32(aob, 0x00);

    match magic {
        // 32-bit little endian
        0xFEEDFACE => {
            let mut macho32 = MachO32::default();
            macho32.header = parse_macho_header_32(aob)?;

            let mut offset = 0x1C; // justo despues del header 32
            for _ in 0..macho32.header.n_cmds {
                let cmd = read_u32(aob, offset);
                let cmd_size = read_u32(aob, offset + 0x04);

                macho32.load_commands.push(LoadCommand { cmd, cmd_size });

                // 0x1 = LC_SEGMENT
                if cmd == 0x1 {
                    macho32.segments.push(parse_segment_command_32(aob, offset)?);
                }

                offset += cmd_size as usize;
            }

            Ok(MachO::MachO32(macho32))
        }

        // 64-bit little endian
        0xFEEDFACF => {
            let mut macho64 = MachO64::default();
            macho64.header = parse_macho_header_64(aob)?;

            let mut offset = 0x20; // justo despues del header 64
            for _ in 0..macho64.header.n_cmds {
                let cmd = read_u32(aob, offset);
                let cmd_size = read_u32(aob, offset + 0x04);

                macho64.load_commands.push(LoadCommand { cmd, cmd_size });

                // 0x19 = LC_SEGMENT_64
                if cmd == 0x19 {
                    macho64.segments.push(parse_segment_command_64(aob, offset)?);
                }

                offset += cmd_size as usize;
            }

            Ok(MachO::MachO64(macho64))
        }

        // 32-bit big endian
        0xCEFAEDFE => Err(String::from("BIG_ENDIAN_32_NOT_SUPPORTED")),

        // 64-bit big endian
        0xCFFAEDFE => Err(String::from("BIG_ENDIAN_64_NOT_SUPPORTED")),

        _ => Err(String::from("UNKNOWN_MACHO_TYPE"))
    }
}