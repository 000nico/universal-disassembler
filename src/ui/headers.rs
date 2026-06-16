use eframe::egui;
use crate::ui::app::App;
use crate::formats::pe::{PE, DOSMZHeader, PEHeader};

pub fn show(app: &mut App, ui: &mut egui::Ui) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.add_space(8.0);
        ui.heading("⊡ Headers");
        ui.add_space(12.0);

        if let Some(pe) = &app.pe {
            let dos = match pe {
                PE::PE32(p) => &p.dos_mz_header,
                PE::PE64(p) => &p.dos_mz_header,
            };
            show_dos_header(ui, dos);
            ui.add_space(8.0);

            let peh = match pe {
                PE::PE32(p) => &p.pe_header,
                PE::PE64(p) => &p.pe_header,
            };
            show_pe_coff_header(ui, peh);
            ui.add_space(8.0);

            show_optional_header(ui, pe);
        } else {
            ui.label(egui::RichText::new("No PE headers available. Load a PE file.").weak().italics());
        }
    });
}

fn hf(ui: &mut egui::Ui, label: &str, value: String) {
    ui.label(egui::RichText::new(label).weak());
    ui.label(egui::RichText::new(value).monospace());
    ui.end_row();
}

fn show_dos_header(ui: &mut egui::Ui, h: &DOSMZHeader) {
    egui::CollapsingHeader::new(egui::RichText::new("DOS MZ Header").strong().size(13.0))
        .default_open(false)
        .show(ui, |ui| {
            egui::Grid::new("dos_hdr_grid")
                .num_columns(2)
                .spacing([40.0, 3.0])
                .striped(true)
                .show(ui, |ui| {
                    hf(ui, "e_magic", format!("0x{:04X}", h.e_magic));
                    hf(ui, "e_cblp", format!("0x{:04X}", h.e_cblp));
                    hf(ui, "e_cp", format!("0x{:04X}", h.e_cp));
                    hf(ui, "e_crlc", format!("0x{:04X}", h.e_crlc));
                    hf(ui, "e_cparhdr", format!("0x{:04X}", h.e_cparhdr));
                    hf(ui, "e_minalloc", format!("0x{:04X}", h.e_minalloc));
                    hf(ui, "e_maxalloc", format!("0x{:04X}", h.e_maxalloc));
                    hf(ui, "e_ss", format!("0x{:04X}", h.e_ss));
                    hf(ui, "e_sp", format!("0x{:04X}", h.e_sp));
                    hf(ui, "e_csum", format!("0x{:04X}", h.e_csum));
                    hf(ui, "e_ip", format!("0x{:04X}", h.e_ip));
                    hf(ui, "e_cs", format!("0x{:04X}", h.e_cs));
                    hf(ui, "e_lfarlc", format!("0x{:04X}", h.e_lfarlc));
                    hf(ui, "e_ovno", format!("0x{:04X}", h.e_ovno));
                    let res = h.e_res.iter().map(|v| format!("{:04X}", v)).collect::<Vec<_>>().join(" ");
                    hf(ui, "e_res", res);
                    hf(ui, "e_oemid", format!("0x{:04X}", h.e_oemid));
                    hf(ui, "e_oeminfo", format!("0x{:04X}", h.e_oeminfo));
                    let res2 = h.e_res2.iter().map(|v| format!("{:04X}", v)).collect::<Vec<_>>().join(" ");
                    hf(ui, "e_res2", res2);
                    hf(ui, "e_lfanew", format!("0x{:08X}", h.e_lfanew));
                });
        });
}

fn show_pe_coff_header(ui: &mut egui::Ui, h: &PEHeader) {
    egui::CollapsingHeader::new(egui::RichText::new("PE / COFF Header").strong().size(13.0))
        .default_open(true)
        .show(ui, |ui| {
            egui::Grid::new("pe_coff_grid")
                .num_columns(2)
                .spacing([40.0, 3.0])
                .striped(true)
                .show(ui, |ui| {
                    hf(ui, "Signature", format!("0x{:08X}", h.signature));
                    hf(ui, "Machine", format!("0x{:04X}", h.machine));
                    hf(ui, "NumberOfSections", h.number_of_sections.to_string());
                    hf(ui, "TimeDateStamp", format!("0x{:08X}", h.time_date_stamp));
                    hf(ui, "PointerToSymbolTable", format!("0x{:08X}", h.pointer_to_symbol_table));
                    hf(ui, "NumberOfSymbols", h.number_of_symbols.to_string());
                    hf(ui, "SizeOfOptionalHeader", format!("0x{:04X}", h.size_of_optional_header));
                    hf(ui, "Characteristics", format!("0x{:04X}", h.characteristics));
                });
        });
}

fn show_optional_header(ui: &mut egui::Ui, pe: &PE) {
    // ── Standard Fields ──
    egui::CollapsingHeader::new(egui::RichText::new("Optional Header — Standard Fields").strong().size(13.0))
        .default_open(true)
        .show(ui, |ui| {
            egui::Grid::new("oh_standard_grid")
                .num_columns(2)
                .spacing([40.0, 3.0])
                .striped(true)
                .show(ui, |ui| {
                    match pe {
                        PE::PE32(p) => {
                            let oh = &p.optional_header;
                            hf(ui, "Magic", format!("0x{:04X} (PE32)", oh.magic));
                            hf(ui, "MajorLinkerVersion", oh.major_linker_version.to_string());
                            hf(ui, "MinorLinkerVersion", oh.minor_linker_version.to_string());
                            hf(ui, "SizeOfCode", format!("0x{:08X}", oh.size_of_code));
                            hf(ui, "SizeOfInitializedData", format!("0x{:08X}", oh.size_of_initialized_data));
                            hf(ui, "SizeOfUninitializedData", format!("0x{:08X}", oh.size_of_unitialized_data));
                            hf(ui, "AddressOfEntryPoint", format!("0x{:08X}", oh.address_of_entry_point));
                            hf(ui, "BaseOfCode", format!("0x{:08X}", oh.base_of_code));
                            hf(ui, "BaseOfData", format!("0x{:08X}", oh.base_of_data));
                        }
                        PE::PE64(p) => {
                            let oh = &p.optional_header;
                            hf(ui, "Magic", format!("0x{:04X} (PE32+)", oh.magic));
                            hf(ui, "MajorLinkerVersion", oh.major_linker_version.to_string());
                            hf(ui, "MinorLinkerVersion", oh.minor_linker_version.to_string());
                            hf(ui, "SizeOfCode", format!("0x{:08X}", oh.size_of_code));
                            hf(ui, "SizeOfInitializedData", format!("0x{:08X}", oh.size_of_initialized_data));
                            hf(ui, "SizeOfUninitializedData", format!("0x{:08X}", oh.size_of_unitialized_data));
                            hf(ui, "AddressOfEntryPoint", format!("0x{:08X}", oh.address_of_entry_point));
                            hf(ui, "BaseOfCode", format!("0x{:08X}", oh.base_of_code));
                        }
                    }
                });
        });

    ui.add_space(4.0);

    // ── Windows-Specific Fields ──
    egui::CollapsingHeader::new(egui::RichText::new("Optional Header — Windows-Specific").strong().size(13.0))
        .default_open(false)
        .show(ui, |ui| {
            egui::Grid::new("oh_windows_grid")
                .num_columns(2)
                .spacing([40.0, 3.0])
                .striped(true)
                .show(ui, |ui| {
                    match pe {
                        PE::PE32(p) => {
                            let oh = &p.optional_header;
                            hf(ui, "ImageBase", format!("0x{:08X}", oh.image_base));
                            hf(ui, "SectionAlignment", format!("0x{:08X}", oh.section_alignment));
                            hf(ui, "FileAlignment", format!("0x{:08X}", oh.file_alignment));
                            hf(ui, "MajorOSVersion", oh.major_operating_system_version.to_string());
                            hf(ui, "MinorOSVersion", oh.minor_operating_system_version.to_string());
                            hf(ui, "MajorImageVersion", oh.major_image_version.to_string());
                            hf(ui, "MinorImageVersion", oh.minor_image_version.to_string());
                            hf(ui, "MajorSubsystemVersion", oh.major_subsystem_version.to_string());
                            hf(ui, "MinorSubsystemVersion", oh.minor_subsystem_version.to_string());
                            hf(ui, "Reserved1", format!("0x{:08X}", oh.reserved1));
                            hf(ui, "SizeOfImage", format!("0x{:08X}", oh.size_of_image));
                            hf(ui, "SizeOfHeaders", format!("0x{:08X}", oh.size_of_headers));
                            hf(ui, "CheckSum", format!("0x{:08X}", oh.check_sum));
                            hf(ui, "Subsystem", format!("0x{:04X}", oh.subsystem));
                            hf(ui, "DllCharacteristics", format!("0x{:04X}", oh.dll_characteristics));
                            hf(ui, "SizeOfStackReserve", format!("0x{:08X}", oh.size_of_stack_reserve));
                            hf(ui, "SizeOfStackCommit", format!("0x{:08X}", oh.size_of_stack_commit));
                            hf(ui, "SizeOfHeapReserve", format!("0x{:08X}", oh.size_of_heap_reserve));
                            hf(ui, "SizeOfHeapCommit", format!("0x{:08X}", oh.size_of_heap_commit));
                            hf(ui, "LoaderFlags", format!("0x{:08X}", oh.loader_flags));
                            hf(ui, "NumberOfRvaAndSizes", oh.number_of_rva_and_sizes.to_string());
                        }
                        PE::PE64(p) => {
                            let oh = &p.optional_header;
                            hf(ui, "ImageBase", format!("0x{:016X}", oh.image_base));
                            hf(ui, "SectionAlignment", format!("0x{:08X}", oh.section_alignment));
                            hf(ui, "FileAlignment", format!("0x{:08X}", oh.file_alignment));
                            hf(ui, "MajorOSVersion", oh.major_operating_system_version.to_string());
                            hf(ui, "MinorOSVersion", oh.minor_operating_system_version.to_string());
                            hf(ui, "MajorImageVersion", oh.major_image_version.to_string());
                            hf(ui, "MinorImageVersion", oh.minor_image_version.to_string());
                            hf(ui, "MajorSubsystemVersion", oh.major_subsystem_version.to_string());
                            hf(ui, "MinorSubsystemVersion", oh.minor_subsystem_version.to_string());
                            hf(ui, "Reserved1", format!("0x{:08X}", oh.reserved1));
                            hf(ui, "SizeOfImage", format!("0x{:08X}", oh.size_of_image));
                            hf(ui, "SizeOfHeaders", format!("0x{:08X}", oh.size_of_headers));
                            hf(ui, "CheckSum", format!("0x{:08X}", oh.check_sum));
                            hf(ui, "Subsystem", format!("0x{:04X}", oh.subsystem));
                            hf(ui, "DllCharacteristics", format!("0x{:04X}", oh.dll_characteristics));
                            hf(ui, "SizeOfStackReserve", format!("0x{:016X}", oh.size_of_stack_reserve));
                            hf(ui, "SizeOfStackCommit", format!("0x{:016X}", oh.size_of_stack_commit));
                            hf(ui, "SizeOfHeapReserve", format!("0x{:016X}", oh.size_of_heap_reserve));
                            hf(ui, "SizeOfHeapCommit", format!("0x{:016X}", oh.size_of_heap_commit));
                            hf(ui, "LoaderFlags", format!("0x{:08X}", oh.loader_flags));
                            hf(ui, "NumberOfRvaAndSizes", oh.number_of_rva_and_sizes.to_string());
                        }
                    }
                });
        });

    ui.add_space(4.0);

    // ── Data Directories ──
    egui::CollapsingHeader::new(egui::RichText::new("Optional Header — Data Directories").strong().size(13.0))
        .default_open(false)
        .show(ui, |ui| {
            let dirs = extract_data_dirs(pe);
            egui::Grid::new("data_dirs_grid")
                .num_columns(3)
                .spacing([20.0, 3.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.label(egui::RichText::new("Directory").strong());
                    ui.label(egui::RichText::new("Virtual Address").strong());
                    ui.label(egui::RichText::new("Size").strong());
                    ui.end_row();

                    for (name, va, size) in &dirs {
                        let color = if *va > 0 {
                            egui::Color32::from_rgb(200, 200, 210)
                        } else {
                            egui::Color32::from_rgb(90, 90, 90)
                        };
                        ui.label(egui::RichText::new(*name).color(color));
                        ui.label(egui::RichText::new(format!("0x{:08X}", va)).monospace().color(color));
                        ui.label(egui::RichText::new(format!("0x{:08X}", size)).monospace().color(color));
                        ui.end_row();
                    }
                });
        });
}

fn extract_data_dirs(pe: &PE) -> Vec<(&'static str, u32, u32)> {
    match pe {
        PE::PE32(p) => {
            let oh = &p.optional_header;
            vec![
                ("Export Table", oh.export_directory_va, oh.export_directory_size),
                ("Import Table", oh.import_directory_va, oh.import_directory_size),
                ("Resource Table", oh.resource_directory_va, oh.resource_directory_size),
                ("Exception Table", oh.exception_directory_va, oh.exception_directory_size),
                ("Certificate Table", oh.security_directory_va, oh.security_directory_size),
                ("Base Relocation Table", oh.base_relocation_table_va, oh.base_relocation_table_size),
                ("Debug", oh.debug_directory_va, oh.debug_directory_size),
                ("Architecture", oh.architecture_specific_data_va, oh.architecture_specific_data_size),
                ("Global Ptr", oh.rva_of_gp_va, oh.rva_of_gp_size),
                ("TLS Table", oh.tls_directory_va, oh.tls_directory_size),
                ("Load Config Table", oh.load_configuration_directory_va, oh.load_configuration_directory_size),
                ("Bound Import", oh.bound_import_directory_in_headers_va, oh.bound_import_directory_in_headers_size),
                ("IAT", oh.import_address_table_va, oh.import_address_table_size),
                ("Delay Import Descriptor", oh.delay_load_import_descriptors_va, oh.delay_load_import_descriptors_size),
                ("CLR Runtime Header", oh.com_runtime_descriptor_va, oh.com_runtime_descriptor_size),
                ("Reserved", oh.reserved_0_1, oh.reserved_0_2),
            ]
        }
        PE::PE64(p) => {
            let oh = &p.optional_header;
            vec![
                ("Export Table", oh.export_directory_va, oh.export_directory_size),
                ("Import Table", oh.import_directory_va, oh.import_directory_size),
                ("Resource Table", oh.resource_directory_va, oh.resource_directory_size),
                ("Exception Table", oh.exception_directory_va, oh.exception_directory_size),
                ("Certificate Table", oh.security_directory_va, oh.security_directory_size),
                ("Base Relocation Table", oh.base_relocation_table_va, oh.base_relocation_table_size),
                ("Debug", oh.debug_directory_va, oh.debug_directory_size),
                ("Architecture", oh.architecture_specific_data_va, oh.architecture_specific_data_size),
                ("Global Ptr", oh.rva_of_gp_va, oh.rva_of_gp_size),
                ("TLS Table", oh.tls_directory_va, oh.tls_directory_size),
                ("Load Config Table", oh.load_configuration_directory_va, oh.load_configuration_directory_size),
                ("Bound Import", oh.bound_import_directory_in_headers_va, oh.bound_import_directory_in_headers_size),
                ("IAT", oh.import_address_table_va, oh.import_address_table_size),
                ("Delay Import Descriptor", oh.delay_load_import_descriptors_va, oh.delay_load_import_descriptors_size),
                ("CLR Runtime Header", oh.com_runtime_descriptor_va, oh.com_runtime_descriptor_size),
                ("Reserved", oh.reserved_0_1, oh.reserved_0_2),
            ]
        }
    }
}
