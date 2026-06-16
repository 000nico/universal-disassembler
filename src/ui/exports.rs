use eframe::egui;
use crate::ui::app::App;
use crate::formats::pe::PE;

pub fn show(app: &mut App, ui: &mut egui::Ui) {
    ui.add_space(8.0);
    ui.heading("↑ Exports");
    ui.add_space(12.0);

    if let Some(pe) = &app.pe {
        let ed = match pe {
            PE::PE32(p) => &p.export_directory,
            PE::PE64(p) => &p.export_directory,
        };

        if ed.number_of_functions == 0 && ed.name == 0 {
            ui.label(egui::RichText::new("No export directory found in this binary.").weak().italics());
            return;
        }

        ui.label(egui::RichText::new("Export Directory").strong().size(13.0));
        ui.add_space(4.0);

        egui::Grid::new("export_grid")
            .num_columns(2)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                hf(ui, "Characteristics", format!("0x{:08X}", ed.characteristics));
                hf(ui, "TimeDateStamp", format!("0x{:08X}", ed.time_date_stamp));
                hf(ui, "Version", format!("{}.{}", ed.major_version, ed.minor_version));
                hf(ui, "Name RVA", format!("0x{:08X}", ed.name));
                hf(ui, "Base", format!("0x{:08X}", ed.base));
                hf(ui, "NumberOfFunctions", ed.number_of_functions.to_string());
                hf(ui, "NumberOfNames", ed.number_of_names.to_string());
                hf(ui, "AddressOfFunctions", format!("0x{:08X}", ed.address_of_functions));
                hf(ui, "AddressOfNames", format!("0x{:08X}", ed.address_of_names));
                hf(ui, "AddressOfNameOrdinals", format!("0x{:08X}", ed.address_of_name_ordinals));
            });
    } else {
        ui.label(egui::RichText::new("No PE data available.").weak().italics());
    }
}

fn hf(ui: &mut egui::Ui, label: &str, value: String) {
    ui.label(egui::RichText::new(label).weak());
    ui.label(egui::RichText::new(value).monospace());
    ui.end_row();
}
