use eframe::egui;
use crate::ui::app::App;
use crate::formats::pe::PE;

pub fn show(app: &mut App, ui: &mut egui::Ui) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.add_space(8.0);
        ui.heading("⬡ Overview");
        ui.add_space(12.0);

        // ── File Information ──
        ui.label(egui::RichText::new("File Information").strong().size(13.0));
        ui.add_space(4.0);
        egui::Grid::new("overview_file_grid")
            .num_columns(2)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                if let Some(path) = &app.file_path {
                    field(ui, "Path", path.clone());
                    if let Some(name) = std::path::Path::new(path)
                        .file_name()
                        .and_then(|n| n.to_str())
                    {
                        field(ui, "File Name", name.to_string());
                    }
                }
                if let Some(data) = &app.binary_data {
                    field(ui, "File Size", format_size(data.len()));
                }
                if let Some(fmt) = &app.format {
                    field(ui, "Format", format!("{:?}", fmt));
                }
                if !app.strings.is_empty() {
                    field(ui, "Strings Found", app.strings.len().to_string());
                }
            });

        ui.add_space(16.0);
        ui.separator();
        ui.add_space(12.0);

        // ── PE Summary ──
        if let Some(pe) = &app.pe {
            show_pe_summary(ui, pe);
        } else {
            ui.label(
                egui::RichText::new("No detailed format information available.")
                    .weak()
                    .italics(),
            );
        }
    });
}

fn show_pe_summary(ui: &mut egui::Ui, pe: &PE) {
    let (pe_type, machine, num_sections, timestamp, characteristics,
         entry_point, image_base_str, subsystem, dll_chars,
         linker_ver, size_of_image, size_of_headers,
         num_imports, num_export_funcs) = match pe {
        PE::PE32(p) => (
            "PE32 (32-bit)",
            p.pe_header.machine,
            p.pe_header.number_of_sections,
            p.pe_header.time_date_stamp,
            p.pe_header.characteristics,
            p.optional_header.address_of_entry_point,
            format!("0x{:08X}", p.optional_header.image_base),
            p.optional_header.subsystem,
            p.optional_header.dll_characteristics,
            format!("{}.{}", p.optional_header.major_linker_version, p.optional_header.minor_linker_version),
            p.optional_header.size_of_image,
            p.optional_header.size_of_headers,
            p.import_directories.len(),
            p.export_directory.number_of_functions,
        ),
        PE::PE64(p) => (
            "PE32+ (64-bit)",
            p.pe_header.machine,
            p.pe_header.number_of_sections,
            p.pe_header.time_date_stamp,
            p.pe_header.characteristics,
            p.optional_header.address_of_entry_point,
            format!("0x{:016X}", p.optional_header.image_base),
            p.optional_header.subsystem,
            p.optional_header.dll_characteristics,
            format!("{}.{}", p.optional_header.major_linker_version, p.optional_header.minor_linker_version),
            p.optional_header.size_of_image,
            p.optional_header.size_of_headers,
            p.import_directories.len(),
            p.export_directory.number_of_functions,
        ),
    };

    ui.label(egui::RichText::new(format!("PE Summary — {}", pe_type)).strong().size(13.0));
    ui.add_space(4.0);

    egui::Grid::new("pe_summary_grid")
        .num_columns(2)
        .spacing([40.0, 4.0])
        .striped(true)
        .show(ui, |ui| {
            field(ui, "Type", pe_type.to_string());
            field(ui, "Machine", format!("0x{:04X} ({})", machine, machine_name(machine)));
            field(ui, "Entry Point", format!("0x{:08X}", entry_point));
            field(ui, "Image Base", image_base_str);
            field(ui, "Sections", num_sections.to_string());
            field(ui, "TimeDateStamp", format!("0x{:08X}", timestamp));
            field(ui, "Characteristics", format!("0x{:04X}", characteristics));
            let flags = characteristics_flags(characteristics);
            if !flags.is_empty() {
                field(ui, "  Flags", flags.join(", "));
            }
            field(ui, "Subsystem", format!("0x{:04X} ({})", subsystem, subsystem_name(subsystem)));
            field(ui, "DLL Characteristics", format!("0x{:04X}", dll_chars));
            field(ui, "Linker Version", linker_ver);
            field(ui, "Size of Image", format!("0x{:08X}", size_of_image));
            field(ui, "Size of Headers", format!("0x{:08X}", size_of_headers));
            field(ui, "Import Directories", num_imports.to_string());
            field(ui, "Exported Functions", num_export_funcs.to_string());
        });
}

fn field(ui: &mut egui::Ui, label: &str, value: String) {
    ui.label(egui::RichText::new(label).weak());
    ui.label(egui::RichText::new(value).monospace());
    ui.end_row();
}

fn format_size(bytes: usize) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB ({} bytes)", bytes as f64 / 1024.0, bytes)
    } else {
        format!("{:.2} MB ({} bytes)", bytes as f64 / (1024.0 * 1024.0), bytes)
    }
}

fn machine_name(machine: u16) -> &'static str {
    match machine {
        0x0 => "Unknown",
        0x14c => "i386",
        0x166 => "MIPS R4000",
        0x1a2 => "Hitachi SH3",
        0x1a6 => "Hitachi SH4",
        0x1c0 => "ARM",
        0x1c4 => "ARM Thumb-2",
        0x200 => "IA-64",
        0x8664 => "AMD64",
        0xAA64 => "ARM64",
        _ => "Other",
    }
}

fn subsystem_name(subsystem: u16) -> &'static str {
    match subsystem {
        0 => "Unknown",
        1 => "Native",
        2 => "Windows GUI",
        3 => "Windows CUI",
        5 => "OS/2 CUI",
        7 => "POSIX CUI",
        9 => "Windows CE GUI",
        10 => "EFI Application",
        11 => "EFI Boot Service Driver",
        12 => "EFI Runtime Driver",
        13 => "EFI ROM",
        14 => "XBOX",
        16 => "Windows Boot Application",
        _ => "Other",
    }
}

fn characteristics_flags(chars: u16) -> Vec<&'static str> {
    let mut flags = Vec::new();
    if chars & 0x0001 != 0 { flags.push("RELOCS_STRIPPED"); }
    if chars & 0x0002 != 0 { flags.push("EXECUTABLE_IMAGE"); }
    if chars & 0x0004 != 0 { flags.push("LINE_NUMS_STRIPPED"); }
    if chars & 0x0008 != 0 { flags.push("LOCAL_SYMS_STRIPPED"); }
    if chars & 0x0020 != 0 { flags.push("LARGE_ADDRESS_AWARE"); }
    if chars & 0x0100 != 0 { flags.push("32BIT_MACHINE"); }
    if chars & 0x0200 != 0 { flags.push("DEBUG_STRIPPED"); }
    if chars & 0x2000 != 0 { flags.push("DLL"); }
    flags
}
