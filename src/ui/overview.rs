use eframe::egui;
use crate::ui::app::App;
use crate::formats::pe::PE;

pub fn show(app: &mut App, ui: &mut egui::Ui) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.add_space(4.0);

        // ── File Information ──
        section_heading(ui, "File Information");
        ui.add_space(4.0);
        card(ui, |ui| {
            egui::Grid::new("overview_file_grid")
                .num_columns(2)
                .spacing([40.0, 6.0])
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
        });

        ui.add_space(16.0);

        // ── File Hashes ──
        if !app.hash_md5.is_empty() {
            section_heading(ui, "File Hashes");
            ui.add_space(4.0);
            card(ui, |ui| {
                hash_row(ui, "MD5", &app.hash_md5);
                ui.add_space(6.0);
                hash_row(ui, "SHA-1", &app.hash_sha1);
                ui.add_space(6.0);
                hash_row(ui, "SHA-256", &app.hash_sha256);
            });
            ui.add_space(16.0);
        }

        // ── Entropy Overview ──
        if !app.entropy_blocks.is_empty() {
            section_heading(ui, "Entropy Overview");
            ui.add_space(4.0);

            let blocks = &app.entropy_blocks;
            let min_e = blocks.iter().cloned().fold(f64::MAX, f64::min);
            let max_e = blocks.iter().cloned().fold(f64::MIN, f64::max);
            let avg_e = blocks.iter().sum::<f64>() / blocks.len() as f64;

            card(ui, |ui| {
                ui.horizontal(|ui| {
                    stat_badge(ui, "Blocks", &blocks.len().to_string());
                    ui.add_space(24.0);
                    stat_badge(ui, "Min", &format!("{:.2}", min_e));
                    ui.add_space(24.0);
                    stat_badge(ui, "Max", &format!("{:.2}", max_e));
                    ui.add_space(24.0);
                    stat_badge(ui, "Avg", &format!("{:.2}", avg_e));
                });
            });

            ui.add_space(8.0);
            draw_mini_entropy(ui, blocks);
            ui.add_space(16.0);
        }

        // ── PE Summary ──
        if let Some(pe) = &app.pe {
            section_heading(ui, "PE Summary");
            ui.add_space(4.0);
            show_pe_summary(ui, pe);
        } else if app.binary_data.is_some() && app.pe.is_none() && app.macho.is_none() {
            ui.add_space(8.0);
            ui.label(
                egui::RichText::new("No detailed format-specific information available.")
                    .weak()
                    .italics(),
            );
        }
    });
}

// ── UI Helpers ──

fn section_heading(ui: &mut egui::Ui, text: &str) {
    ui.horizontal(|ui| {
        let (bar_rect, _) = ui.allocate_exact_size(egui::vec2(3.0, 16.0), egui::Sense::hover());
        ui.painter().rect_filled(
            bar_rect,
            1.5,
            egui::Color32::from_rgb(86, 156, 214),
        );
        ui.add_space(6.0);
        ui.label(
            egui::RichText::new(text)
                .strong()
                .size(14.0)
                .color(egui::Color32::from_rgb(220, 224, 230)),
        );
    });
}

fn card(ui: &mut egui::Ui, content: impl FnOnce(&mut egui::Ui)) {
    egui::Frame::none()
        .fill(egui::Color32::from_rgb(30, 30, 38))
        .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(44, 46, 58)))
        .inner_margin(14.0)
        .rounding(8.0)
        .show(ui, content);
}

fn field(ui: &mut egui::Ui, label: &str, value: String) {
    ui.label(
        egui::RichText::new(label)
            .color(egui::Color32::from_rgb(130, 135, 150))
            .size(12.0),
    );
    ui.label(egui::RichText::new(value).monospace().size(12.0));
    ui.end_row();
}

fn hash_row(ui: &mut egui::Ui, label: &str, hash: &str) {
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(format!("{:>7}", label))
                .monospace()
                .size(11.0)
                .color(egui::Color32::from_rgb(130, 135, 150)),
        );
        ui.add_space(12.0);
        ui.label(
            egui::RichText::new(hash)
                .monospace()
                .size(11.0)
                .color(egui::Color32::from_rgb(78, 201, 176)),
        );
    });
}

fn stat_badge(ui: &mut egui::Ui, label: &str, value: &str) {
    ui.vertical(|ui| {
        ui.label(
            egui::RichText::new(label)
                .size(10.0)
                .color(egui::Color32::from_rgb(120, 125, 140)),
        );
        ui.label(
            egui::RichText::new(value)
                .size(14.0)
                .strong()
                .monospace(),
        );
    });
}

fn entropy_color(e: f64) -> egui::Color32 {
    let t = (e / 8.0).clamp(0.0, 1.0) as f32;
    if t < 0.375 {
        let s = t / 0.375;
        egui::Color32::from_rgb(
            (60.0 + 195.0 * s) as u8,
            (200.0 - 10.0 * s) as u8,
            (80.0 - 60.0 * s) as u8,
        )
    } else {
        let s = (t - 0.375) / 0.625;
        egui::Color32::from_rgb(
            255,
            (190.0 - 170.0 * s) as u8,
            (20.0 - 20.0 * s) as u8,
        )
    }
}

fn draw_mini_entropy(ui: &mut egui::Ui, blocks: &[f64]) {
    let num = blocks.len();
    if num == 0 {
        return;
    }

    let width = ui.available_width().min(800.0);
    let height = 64.0_f32;

    let (response, painter) = ui.allocate_painter(
        egui::vec2(width, height),
        egui::Sense::hover(),
    );
    let rect = response.rect;

    // Background
    painter.rect_filled(rect, 6.0, egui::Color32::from_rgb(24, 24, 32));
    painter.rect_stroke(
        rect,
        6.0,
        egui::Stroke::new(1.0, egui::Color32::from_rgb(44, 46, 58)),
        egui::StrokeKind::Inside,
    );

    let bar_w = rect.width() / num as f32;

    for (i, &entropy) in blocks.iter().enumerate() {
        let t = (entropy / 8.0).clamp(0.0, 1.0) as f32;
        let bar_h = t * (height - 6.0);
        let color = entropy_color(entropy);

        let x0 = rect.left() + i as f32 * bar_w;
        let x1 = (x0 + bar_w).min(rect.right());

        if bar_h > 0.3 {
            let bar_rect = egui::Rect::from_min_max(
                egui::pos2(x0, rect.bottom() - bar_h - 2.0),
                egui::pos2(x1, rect.bottom() - 2.0),
            );
            painter.rect_filled(bar_rect, 0.0, color);
        }
    }
}

// ── PE Summary ──

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

    card(ui, |ui| {
        ui.label(
            egui::RichText::new(pe_type)
                .strong()
                .size(13.0)
                .color(egui::Color32::from_rgb(86, 156, 214)),
        );
        ui.add_space(8.0);

        egui::Grid::new("pe_summary_grid")
            .num_columns(2)
            .spacing([40.0, 5.0])
            .show(ui, |ui| {
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
    });
}

fn format_size(bytes: usize) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB ({} bytes)", bytes as f64 / 1024.0, bytes)
    } else {
        format!(
            "{:.2} MB ({} bytes)",
            bytes as f64 / (1024.0 * 1024.0),
            bytes
        )
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
