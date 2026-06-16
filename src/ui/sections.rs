use eframe::egui;
use crate::ui::app::App;
use crate::formats::pe::PE;

pub fn show(app: &mut App, ui: &mut egui::Ui) {
    ui.add_space(8.0);
    ui.heading("⊞ Sections");
    ui.add_space(12.0);

    if let Some(pe) = &app.pe {
        let sections = match pe {
            PE::PE32(p) => &p.sections,
            PE::PE64(p) => &p.sections,
        };

        if sections.is_empty() {
            ui.label(egui::RichText::new("No sections found.").weak().italics());
            return;
        }

        ui.label(egui::RichText::new(format!("{} Sections", sections.len())).strong().size(13.0));
        ui.add_space(8.0);

        egui::ScrollArea::both().show(ui, |ui| {
            egui::Grid::new("sections_table")
                .num_columns(8)
                .spacing([12.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    // Header row
                    ui.label(egui::RichText::new("Name").strong());
                    ui.label(egui::RichText::new("VirtualSize").strong());
                    ui.label(egui::RichText::new("VirtualAddr").strong());
                    ui.label(egui::RichText::new("RawSize").strong());
                    ui.label(egui::RichText::new("RawDataPtr").strong());
                    ui.label(egui::RichText::new("Relocs").strong());
                    ui.label(egui::RichText::new("LineNums").strong());
                    ui.label(egui::RichText::new("Characteristics").strong());
                    ui.end_row();

                    for s in sections {
                        let name = section_name(&s.name);
                        ui.label(egui::RichText::new(name).monospace().strong());
                        ui.label(egui::RichText::new(format!("0x{:08X}", s.physical_address)).monospace());
                        ui.label(egui::RichText::new(format!("0x{:08X}", s.virtual_address)).monospace());
                        ui.label(egui::RichText::new(format!("0x{:08X}", s.size_of_raw_data)).monospace());
                        ui.label(egui::RichText::new(format!("0x{:08X}", s.pointer_to_raw_data)).monospace());
                        ui.label(egui::RichText::new(format!("{}", s.number_of_relocations)).monospace());
                        ui.label(egui::RichText::new(format!("{}", s.number_of_line_numbers)).monospace());

                        let flags = section_flags(s.characteristics);
                        let flags_text = if flags.is_empty() {
                            format!("0x{:08X}", s.characteristics)
                        } else {
                            format!("0x{:08X} [{}]", s.characteristics, flags)
                        };
                        ui.label(egui::RichText::new(flags_text).monospace().size(11.0));
                        ui.end_row();
                    }
                });
        });
    } else {
        ui.label(egui::RichText::new("No PE data available.").weak().italics());
    }
}

fn section_name(name: &[u8; 8]) -> String {
    let end = name.iter().position(|&b| b == 0).unwrap_or(8);
    String::from_utf8_lossy(&name[..end]).to_string()
}

fn section_flags(chars: u32) -> String {
    let mut flags = Vec::new();
    if chars & 0x00000020 != 0 { flags.push("CODE"); }
    if chars & 0x00000040 != 0 { flags.push("IDATA"); }
    if chars & 0x00000080 != 0 { flags.push("UDATA"); }
    if chars & 0x02000000 != 0 { flags.push("DISCARDABLE"); }
    if chars & 0x04000000 != 0 { flags.push("NOT_CACHED"); }
    if chars & 0x08000000 != 0 { flags.push("NOT_PAGED"); }
    if chars & 0x10000000 != 0 { flags.push("SHARED"); }
    if chars & 0x20000000 != 0 { flags.push("EXEC"); }
    if chars & 0x40000000 != 0 { flags.push("READ"); }
    if chars & 0x80000000 != 0 { flags.push("WRITE"); }
    flags.join("|")
}
