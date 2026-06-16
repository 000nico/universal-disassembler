use eframe::egui;
use crate::ui::app::App;
use crate::formats::pe::PE;

pub fn show(app: &mut App, ui: &mut egui::Ui) {
    ui.add_space(8.0);
    ui.heading("↓ Imports");
    ui.add_space(12.0);

    if let Some(pe) = &app.pe {
        let dirs = match pe {
            PE::PE32(p) => &p.import_directories,
            PE::PE64(p) => &p.import_directories,
        };

        if dirs.is_empty() {
            ui.label(egui::RichText::new("No import directories found.").weak().italics());
            return;
        }

        ui.label(egui::RichText::new(format!(
            "{} Import Director{}",
            dirs.len(),
            if dirs.len() == 1 { "y" } else { "ies" }
        )).strong().size(13.0));
        ui.add_space(8.0);

        egui::ScrollArea::both().show(ui, |ui| {
            egui::Grid::new("imports_table")
                .num_columns(6)
                .spacing([16.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.label(egui::RichText::new("#").strong());
                    ui.label(egui::RichText::new("OrigFirstThunk").strong());
                    ui.label(egui::RichText::new("TimeDateStamp").strong());
                    ui.label(egui::RichText::new("ForwarderChain").strong());
                    ui.label(egui::RichText::new("Name RVA").strong());
                    ui.label(egui::RichText::new("FirstThunk").strong());
                    ui.end_row();

                    for (i, id) in dirs.iter().enumerate() {
                        ui.label(egui::RichText::new(i.to_string()).monospace());
                        ui.label(egui::RichText::new(format!("0x{:08X}", id.original_first_thunk)).monospace());
                        ui.label(egui::RichText::new(format!("0x{:08X}", id.time_date_stamp)).monospace());
                        ui.label(egui::RichText::new(format!("0x{:08X}", id.forwarder_chain)).monospace());
                        ui.label(egui::RichText::new(format!("0x{:08X}", id.name)).monospace());
                        ui.label(egui::RichText::new(format!("0x{:08X}", id.first_thunk)).monospace());
                        ui.end_row();
                    }
                });
        });
    } else {
        ui.label(egui::RichText::new("No PE data available.").weak().italics());
    }
}
