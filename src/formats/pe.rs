use crate::formats::{read_u8, read_u16, read_u32, read_u64};

// PE Format https://www.sunshine2k.de/reversing/tuts/tut_pe.htm

#[derive(Default)]
pub struct DOSMZHeader {
    pub e_magic: u16,
    pub e_cblp: u16,
    pub e_cp: u16,
    pub e_crlc: u16,
    pub e_cparhdr: u16,
    pub e_minalloc: u16,
    pub e_maxalloc: u16,
    pub e_ss: u16,
    pub e_sp: u16,
    pub e_csum: u16,
    pub e_ip: u16,
    pub e_cs: u16,
    pub e_lfarlc: u16,
    pub e_ovno: u16,
    pub e_res: [u16; 4],
    pub e_oemid: u16,
    pub e_oeminfo: u16,
    pub e_res2: [u16; 10],
    pub e_lfanew: u32,
}

#[derive(Default)]
pub struct PEHeader {
    pub signature: u32,
    pub machine: u16,
    pub number_of_sections: u16,
    pub time_date_stamp: u32,
    pub pointer_to_symbol_table: u32,
    pub number_of_symbols: u32,
    pub size_of_optional_header: u16,
    pub characteristics: u16,
}

#[derive(Default)]
pub struct OptionalHeader32 {
    pub magic: u16,
    pub major_linker_version: u8,
    pub minor_linker_version: u8,
    pub size_of_code: u32,
    pub size_of_initialized_data: u32,
    pub size_of_unitialized_data: u32,
    pub address_of_entry_point: u32,
    pub base_of_code: u32,
    pub base_of_data: u32,
    pub image_base: u32,
    pub section_alignment: u32,
    pub file_alignment: u32,
    pub major_operating_system_version: u16,
    pub minor_operating_system_version: u16,
    pub major_image_version: u16,
    pub minor_image_version: u16,
    pub major_subsystem_version: u16,
    pub minor_subsystem_version: u16,
    pub reserved1: u32,
    pub size_of_image: u32,
    pub size_of_headers: u32,
    pub check_sum: u32,
    pub subsystem: u16,
    pub dll_characteristics: u16,
    pub size_of_stack_reserve: u32,
    pub size_of_stack_commit: u32,
    pub size_of_heap_reserve: u32,
    pub size_of_heap_commit: u32,
    pub loader_flags: u32,
    pub number_of_rva_and_sizes: u32,
    pub export_directory_va: u32,
    pub export_directory_size: u32,
    pub import_directory_va: u32,
    pub import_directory_size: u32,
    pub resource_directory_va: u32,
    pub resource_directory_size: u32,
    pub exception_directory_va: u32,
    pub exception_directory_size: u32,
    pub security_directory_va: u32,
    pub security_directory_size: u32,
    pub base_relocation_table_va: u32,
    pub base_relocation_table_size: u32,
    pub debug_directory_va: u32,
    pub debug_directory_size: u32,
    pub architecture_specific_data_va: u32,
    pub architecture_specific_data_size: u32,
    pub rva_of_gp_va: u32,
    pub rva_of_gp_size: u32,
    pub tls_directory_va: u32,
    pub tls_directory_size: u32,
    pub load_configuration_directory_va: u32,
    pub load_configuration_directory_size: u32,
    pub bound_import_directory_in_headers_va: u32,
    pub bound_import_directory_in_headers_size: u32,
    pub import_address_table_va: u32,
    pub import_address_table_size: u32,
    pub delay_load_import_descriptors_va: u32,
    pub delay_load_import_descriptors_size: u32,
    pub com_runtime_descriptor_va: u32,
    pub com_runtime_descriptor_size: u32,
    pub reserved_0_1: u32,
    pub reserved_0_2: u32,
}

#[derive(Default)]
pub struct OptionalHeader64 {
    pub magic: u16,
    pub major_linker_version: u8,
    pub minor_linker_version: u8,
    pub size_of_code: u32,
    pub size_of_initialized_data: u32,
    pub size_of_unitialized_data: u32,
    pub address_of_entry_point: u32,
    pub base_of_code: u32,
    pub image_base: u64,
    pub section_alignment: u32,
    pub file_alignment: u32,
    pub major_operating_system_version: u16,
    pub minor_operating_system_version: u16,
    pub major_image_version: u16,
    pub minor_image_version: u16,
    pub major_subsystem_version: u16,
    pub minor_subsystem_version: u16,
    pub reserved1: u32,
    pub size_of_image: u32,
    pub size_of_headers: u32,
    pub check_sum: u32,
    pub subsystem: u16,
    pub dll_characteristics: u16,
    pub size_of_stack_reserve: u64,
    pub size_of_stack_commit: u64,
    pub size_of_heap_reserve: u64,
    pub size_of_heap_commit: u64,
    pub loader_flags: u32,
    pub number_of_rva_and_sizes: u32,
    pub export_directory_va: u32,
    pub export_directory_size: u32,
    pub import_directory_va: u32,
    pub import_directory_size: u32,
    pub resource_directory_va: u32,
    pub resource_directory_size: u32,
    pub exception_directory_va: u32,
    pub exception_directory_size: u32,
    pub security_directory_va: u32,
    pub security_directory_size: u32,
    pub base_relocation_table_va: u32,
    pub base_relocation_table_size: u32,
    pub debug_directory_va: u32,
    pub debug_directory_size: u32,
    pub architecture_specific_data_va: u32,
    pub architecture_specific_data_size: u32,
    pub rva_of_gp_va: u32,
    pub rva_of_gp_size: u32,
    pub tls_directory_va: u32,
    pub tls_directory_size: u32,
    pub load_configuration_directory_va: u32,
    pub load_configuration_directory_size: u32,
    pub bound_import_directory_in_headers_va: u32,
    pub bound_import_directory_in_headers_size: u32,
    pub import_address_table_va: u32,
    pub import_address_table_size: u32,
    pub delay_load_import_descriptors_va: u32,
    pub delay_load_import_descriptors_size: u32,
    pub com_runtime_descriptor_va: u32,
    pub com_runtime_descriptor_size: u32,
    pub reserved_0_1: u32,
    pub reserved_0_2: u32,
}

#[derive(Default)]
pub struct SectionHeader {
    pub name: [u8; 8],
    pub physical_address: u32,
    pub virtual_address: u32,
    pub size_of_raw_data: u32,
    pub pointer_to_raw_data: u32,
    pub pointer_to_relocations: u32,
    pub pointer_to_line_numbers: u32,
    pub number_of_relocations: u16,
    pub number_of_line_numbers: u16,
    pub characteristics: u32,
}

#[derive(Default)]
pub struct ExportDirectory {
    pub characteristics: u32,
    pub time_date_stamp: u32,
    pub major_version: u16,
    pub minor_version: u16,
    pub name: u32,
    pub base: u32,
    pub number_of_functions: u32,
    pub number_of_names: u32,
    pub address_of_functions: u32,
    pub address_of_names: u32,
    pub address_of_name_ordinals: u32,
}

#[derive(Default)]
pub struct ImportDirectory {
    pub original_first_thunk: u32,
    pub time_date_stamp: u32,
    pub forwarder_chain: u32,
    pub name: u32,
    pub first_thunk: u32,
}

#[derive(Default)]
pub struct PE32 {
    pub dos_mz_header: DOSMZHeader,
    pub pe_header: PEHeader,
    pub optional_header: OptionalHeader32,
    pub sections: Vec<SectionHeader>,
    pub export_directory: ExportDirectory,
    pub import_directories: Vec<ImportDirectory>,
}

#[derive(Default)]
pub struct PE64 {
    pub dos_mz_header: DOSMZHeader,
    pub pe_header: PEHeader,
    pub optional_header: OptionalHeader64,
    pub sections: Vec<SectionHeader>,
    pub export_directory: ExportDirectory,
    pub import_directories: Vec<ImportDirectory>,
}

pub enum PE {
    PE32(PE32),
    PE64(PE64),
}

fn parse_dos_mz_header(aob: &[u8]) -> Result<DOSMZHeader, String> {
    let mut mzh = DOSMZHeader::default();
    mzh.e_magic = read_u16(aob, 0x00);
    mzh.e_cblp = read_u16(aob, 0x02);
    mzh.e_cp = read_u16(aob, 0x04);
    mzh.e_crlc = read_u16(aob, 0x06);
    mzh.e_cparhdr = read_u16(aob, 0x08);
    mzh.e_minalloc = read_u16(aob, 0x0A);
    mzh.e_maxalloc = read_u16(aob, 0x0C);
    mzh.e_ss = read_u16(aob, 0x0E);
    mzh.e_sp = read_u16(aob, 0x10);
    mzh.e_csum = read_u16(aob, 0x12);
    mzh.e_ip = read_u16(aob, 0x14);
    mzh.e_cs = read_u16(aob, 0x16);
    mzh.e_lfarlc = read_u16(aob, 0x18);
    mzh.e_ovno = read_u16(aob, 0x1A);
    for i in 0..4 {
        mzh.e_res[i] = read_u16(aob, 0x1C + (i * 2));
    }
    mzh.e_oemid = read_u16(aob, 0x24);
    mzh.e_oeminfo = read_u16(aob, 0x26);
    for i in 0..10 {
        mzh.e_res2[i] = read_u16(aob, 0x28 + (i * 2));
    }
    mzh.e_lfanew = read_u32(aob, 0x3C);
    Ok(mzh)
}

fn parse_pe_header(aob: &[u8], e_lfanew: usize) -> Result<PEHeader, String> {
    let mut ph = PEHeader::default();
    ph.signature = read_u32(aob, e_lfanew + 0x00);
    ph.machine = read_u16(aob, e_lfanew + 0x04);
    ph.number_of_sections = read_u16(aob, e_lfanew + 0x06);
    ph.time_date_stamp = read_u32(aob, e_lfanew + 0x08);
    ph.pointer_to_symbol_table = read_u32(aob, e_lfanew + 0x0C);
    ph.number_of_symbols = read_u32(aob, e_lfanew + 0x10);
    ph.size_of_optional_header = read_u16(aob, e_lfanew + 0x14);
    ph.characteristics = read_u16(aob, e_lfanew + 0x16);
    Ok(ph)
}

fn parse_optional_header32(aob: &[u8], base: usize) -> Result<OptionalHeader32, String> {
    let mut oh = OptionalHeader32::default();
    oh.magic = read_u16(aob, base + 0x00);
    oh.major_linker_version = read_u8(aob, base + 0x02);
    oh.minor_linker_version = read_u8(aob, base + 0x03);
    oh.size_of_code = read_u32(aob, base + 0x04);
    oh.size_of_initialized_data = read_u32(aob, base + 0x08);
    oh.size_of_unitialized_data = read_u32(aob, base + 0x0C);
    oh.address_of_entry_point = read_u32(aob, base + 0x10);
    oh.base_of_code = read_u32(aob, base + 0x14);
    oh.base_of_data = read_u32(aob, base + 0x18);
    oh.image_base = read_u32(aob, base + 0x1C);
    oh.section_alignment = read_u32(aob, base + 0x20);
    oh.file_alignment = read_u32(aob, base + 0x24);
    oh.major_operating_system_version = read_u16(aob, base + 0x28);
    oh.minor_operating_system_version = read_u16(aob, base + 0x2A);
    oh.major_image_version = read_u16(aob, base + 0x2C);
    oh.minor_image_version = read_u16(aob, base + 0x2E);
    oh.major_subsystem_version = read_u16(aob, base + 0x30);
    oh.minor_subsystem_version = read_u16(aob, base + 0x32);
    oh.reserved1 = read_u32(aob, base + 0x34);
    oh.size_of_image = read_u32(aob, base + 0x38);
    oh.size_of_headers = read_u32(aob, base + 0x3C);
    oh.check_sum = read_u32(aob, base + 0x40);
    oh.subsystem = read_u16(aob, base + 0x44);
    oh.dll_characteristics = read_u16(aob, base + 0x46);
    oh.size_of_stack_reserve = read_u32(aob, base + 0x48);
    oh.size_of_stack_commit = read_u32(aob, base + 0x4C);
    oh.size_of_heap_reserve = read_u32(aob, base + 0x50);
    oh.size_of_heap_commit = read_u32(aob, base + 0x54);
    oh.loader_flags = read_u32(aob, base + 0x58);
    oh.number_of_rva_and_sizes = read_u32(aob, base + 0x5C);
    oh.export_directory_va = read_u32(aob, base + 0x60);
    oh.export_directory_size = read_u32(aob, base + 0x64);
    oh.import_directory_va = read_u32(aob, base + 0x68);
    oh.import_directory_size = read_u32(aob, base + 0x6C);
    oh.resource_directory_va = read_u32(aob, base + 0x70);
    oh.resource_directory_size = read_u32(aob, base + 0x74);
    oh.exception_directory_va = read_u32(aob, base + 0x78);
    oh.exception_directory_size = read_u32(aob, base + 0x7C);
    oh.security_directory_va = read_u32(aob, base + 0x80);
    oh.security_directory_size = read_u32(aob, base + 0x84);
    oh.base_relocation_table_va = read_u32(aob, base + 0x88);
    oh.base_relocation_table_size = read_u32(aob, base + 0x8C);
    oh.debug_directory_va = read_u32(aob, base + 0x90);
    oh.debug_directory_size = read_u32(aob, base + 0x94);
    oh.architecture_specific_data_va = read_u32(aob, base + 0x98);
    oh.architecture_specific_data_size = read_u32(aob, base + 0x9C);
    oh.rva_of_gp_va = read_u32(aob, base + 0xA0);
    oh.rva_of_gp_size = read_u32(aob, base + 0xA4);
    oh.tls_directory_va = read_u32(aob, base + 0xA8);
    oh.tls_directory_size = read_u32(aob, base + 0xAC);
    oh.load_configuration_directory_va = read_u32(aob, base + 0xB0);
    oh.load_configuration_directory_size = read_u32(aob, base + 0xB4);
    oh.bound_import_directory_in_headers_va = read_u32(aob, base + 0xB8);
    oh.bound_import_directory_in_headers_size = read_u32(aob, base + 0xBC);
    oh.import_address_table_va = read_u32(aob, base + 0xC0);
    oh.import_address_table_size = read_u32(aob, base + 0xC4);
    oh.delay_load_import_descriptors_va = read_u32(aob, base + 0xC8);
    oh.delay_load_import_descriptors_size = read_u32(aob, base + 0xCC);
    oh.com_runtime_descriptor_va = read_u32(aob, base + 0xD0);
    oh.com_runtime_descriptor_size = read_u32(aob, base + 0xD4);
    oh.reserved_0_1 = read_u32(aob, base + 0xD8);
    oh.reserved_0_2 = read_u32(aob, base + 0xDC);
    Ok(oh)
}

fn parse_optional_header64(aob: &[u8], base: usize) -> Result<OptionalHeader64, String> {
    let mut oh = OptionalHeader64::default();
    oh.magic = read_u16(aob, base + 0x00);
    oh.major_linker_version = read_u8(aob, base + 0x02);
    oh.minor_linker_version = read_u8(aob, base + 0x03);
    oh.size_of_code = read_u32(aob, base + 0x04);
    oh.size_of_initialized_data = read_u32(aob, base + 0x08);
    oh.size_of_unitialized_data = read_u32(aob, base + 0x0C);
    oh.address_of_entry_point = read_u32(aob, base + 0x10);
    oh.base_of_code = read_u32(aob, base + 0x14);
    oh.image_base = read_u64(aob, base + 0x18);
    oh.section_alignment = read_u32(aob, base + 0x20);
    oh.file_alignment = read_u32(aob, base + 0x24);
    oh.major_operating_system_version = read_u16(aob, base + 0x28);
    oh.minor_operating_system_version = read_u16(aob, base + 0x2A);
    oh.major_image_version = read_u16(aob, base + 0x2C);
    oh.minor_image_version = read_u16(aob, base + 0x2E);
    oh.major_subsystem_version = read_u16(aob, base + 0x30);
    oh.minor_subsystem_version = read_u16(aob, base + 0x32);
    oh.reserved1 = read_u32(aob, base + 0x34);
    oh.size_of_image = read_u32(aob, base + 0x38);
    oh.size_of_headers = read_u32(aob, base + 0x3C);
    oh.check_sum = read_u32(aob, base + 0x40);
    oh.subsystem = read_u16(aob, base + 0x44);
    oh.dll_characteristics = read_u16(aob, base + 0x46);
    oh.size_of_stack_reserve = read_u64(aob, base + 0x48);
    oh.size_of_stack_commit = read_u64(aob, base + 0x50);
    oh.size_of_heap_reserve = read_u64(aob, base + 0x58);
    oh.size_of_heap_commit = read_u64(aob, base + 0x60);
    oh.loader_flags = read_u32(aob, base + 0x68);
    oh.number_of_rva_and_sizes = read_u32(aob, base + 0x6C);
    oh.export_directory_va = read_u32(aob, base + 0x70);
    oh.export_directory_size = read_u32(aob, base + 0x74);
    oh.import_directory_va = read_u32(aob, base + 0x78);
    oh.import_directory_size = read_u32(aob, base + 0x7C);
    oh.resource_directory_va = read_u32(aob, base + 0x80);
    oh.resource_directory_size = read_u32(aob, base + 0x84);
    oh.exception_directory_va = read_u32(aob, base + 0x88);
    oh.exception_directory_size = read_u32(aob, base + 0x8C);
    oh.security_directory_va = read_u32(aob, base + 0x90);
    oh.security_directory_size = read_u32(aob, base + 0x94);
    oh.base_relocation_table_va = read_u32(aob, base + 0x98);
    oh.base_relocation_table_size = read_u32(aob, base + 0x9C);
    oh.debug_directory_va = read_u32(aob, base + 0xA0);
    oh.debug_directory_size = read_u32(aob, base + 0xA4);
    oh.architecture_specific_data_va = read_u32(aob, base + 0xA8);
    oh.architecture_specific_data_size = read_u32(aob, base + 0xAC);
    oh.rva_of_gp_va = read_u32(aob, base + 0xB0);
    oh.rva_of_gp_size = read_u32(aob, base + 0xB4);
    oh.tls_directory_va = read_u32(aob, base + 0xB8);
    oh.tls_directory_size = read_u32(aob, base + 0xBC);
    oh.load_configuration_directory_va = read_u32(aob, base + 0xC0);
    oh.load_configuration_directory_size = read_u32(aob, base + 0xC4);
    oh.bound_import_directory_in_headers_va = read_u32(aob, base + 0xC8);
    oh.bound_import_directory_in_headers_size = read_u32(aob, base + 0xCC);
    oh.import_address_table_va = read_u32(aob, base + 0xD0);
    oh.import_address_table_size = read_u32(aob, base + 0xD4);
    oh.delay_load_import_descriptors_va = read_u32(aob, base + 0xD8);
    oh.delay_load_import_descriptors_size = read_u32(aob, base + 0xDC);
    oh.com_runtime_descriptor_va = read_u32(aob, base + 0xE0);
    oh.com_runtime_descriptor_size = read_u32(aob, base + 0xE4);
    oh.reserved_0_1 = read_u32(aob, base + 0xE8);
    oh.reserved_0_2 = read_u32(aob, base + 0xEC);
    Ok(oh)
}

fn parse_section_header(aob: &[u8], base: usize) -> Result<SectionHeader, String> {
    let mut sh = SectionHeader::default();
    for i in 0..8 {
        sh.name[i] = read_u8(aob, base + i);
    }
    sh.physical_address = read_u32(aob, base + 0x08);
    sh.virtual_address = read_u32(aob, base + 0x0C);
    sh.size_of_raw_data = read_u32(aob, base + 0x10);
    sh.pointer_to_raw_data = read_u32(aob, base + 0x14);
    sh.pointer_to_relocations = read_u32(aob, base + 0x18);
    sh.pointer_to_line_numbers = read_u32(aob, base + 0x1C);
    sh.number_of_relocations = read_u16(aob, base + 0x20);
    sh.number_of_line_numbers = read_u16(aob, base + 0x22);
    sh.characteristics = read_u32(aob, base + 0x24);
    Ok(sh)
}

fn parse_export_directory(aob: &[u8], base: usize) -> Result<ExportDirectory, String> {
    let mut ed = ExportDirectory::default();
    ed.characteristics = read_u32(aob, base + 0x00);
    ed.time_date_stamp = read_u32(aob, base + 0x04);
    ed.major_version = read_u16(aob, base + 0x08);
    ed.minor_version = read_u16(aob, base + 0x0A);
    ed.name = read_u32(aob, base + 0x0C);
    ed.base = read_u32(aob, base + 0x10);
    ed.number_of_functions = read_u32(aob, base + 0x14);
    ed.number_of_names = read_u32(aob, base + 0x18);
    ed.address_of_functions = read_u32(aob, base + 0x1C);
    ed.address_of_names = read_u32(aob, base + 0x20);
    ed.address_of_name_ordinals = read_u32(aob, base + 0x24);
    Ok(ed)
}

fn parse_import_directory(aob: &[u8], base: usize) -> Result<ImportDirectory, String> {
    let mut id = ImportDirectory::default();
    id.original_first_thunk = read_u32(aob, base + 0x00);
    id.time_date_stamp = read_u32(aob, base + 0x04);
    id.forwarder_chain = read_u32(aob, base + 0x08);
    id.name = read_u32(aob, base + 0x0C);
    id.first_thunk = read_u32(aob, base + 0x10);
    Ok(id)
}

fn parse_sections_and_dirs(aob: &[u8], e_lfanew: usize, optional_header_size: usize, export_va: usize, import_va: usize, num_sections: usize) -> Result<(Vec<SectionHeader>, ExportDirectory, Vec<ImportDirectory>), String> {
    let sections_base = e_lfanew + 0x18 + optional_header_size;
    let mut sections = Vec::new();

    for i in 0..num_sections {
        let offset = sections_base + (i * 0x28);
        sections.push(parse_section_header(aob, offset)?);
    }

    let export_directory = if export_va > 0 {
        parse_export_directory(aob, export_va)?
    } else {
        ExportDirectory::default()
    };

    let mut import_directories = Vec::new();
    let mut import_offset = import_va;
    while import_offset > 0 {
        let id = parse_import_directory(aob, import_offset)?;
        if id.original_first_thunk == 0 && id.name == 0 && id.first_thunk == 0 {
            break;
        }
        import_offset += 0x14;
        import_directories.push(id);
    }

    Ok((sections, export_directory, import_directories))
}

pub fn parse_pe(aob: &[u8]) -> Result<PE, String> {
    let dos_mz_header = parse_dos_mz_header(aob)?;
    let e_lfanew = dos_mz_header.e_lfanew as usize;
    let pe_header = parse_pe_header(aob, e_lfanew)?;
    let optional_base = e_lfanew + 0x18;

    let magic = read_u16(aob, optional_base);

    match magic {
        0x10B => {
            let optional_header = parse_optional_header32(aob, optional_base)?;
            let (sections, export_directory, import_directories) = parse_sections_and_dirs(
                aob,
                e_lfanew,
                pe_header.size_of_optional_header as usize,
                optional_header.export_directory_va as usize,
                optional_header.import_directory_va as usize,
                pe_header.number_of_sections as usize,
            )?;
            Ok(PE::PE32(PE32 { dos_mz_header, pe_header, optional_header, sections, export_directory, import_directories }))
        }
        0x20B => {
            let optional_header = parse_optional_header64(aob, optional_base)?;
            let (sections, export_directory, import_directories) = parse_sections_and_dirs(
                aob,
                e_lfanew,
                pe_header.size_of_optional_header as usize,
                optional_header.export_directory_va as usize,
                optional_header.import_directory_va as usize,
                pe_header.number_of_sections as usize,
            )?;
            Ok(PE::PE64(PE64 { dos_mz_header, pe_header, optional_header, sections, export_directory, import_directories }))
        }
        _ => Err(String::from("UNKNOWN_PE_TYPE"))
    }
}