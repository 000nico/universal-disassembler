use eframe::egui;
use crate::{ui::app::App, tools::patterns::{parse_pattern, pattern_scanner}};

pub fn show(app: &mut App, ui: &mut egui::Ui) {
    ui.heading("Pattern Scanner");
    ui.add_space(8.0);

    ui.horizontal(|ui| {
        ui.label("Pattern (hex bytes, ?? for wildcards):");
        
        let res = ui.text_edit_singleline(&mut app.pattern_input);
        
        if ui.button("Scan").clicked() || (res.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))) {
            if let Some(data) = &app.binary_data {
                if !app.pattern_input.trim().is_empty() {
                    let parsed = parse_pattern(&app.pattern_input);
                    app.pattern_results = pattern_scanner(data, &parsed);
                } else {
                    app.pattern_results.clear();
                }
            }
        }
        
        if ui.button("Clear").clicked() {
            app.pattern_input.clear();
            app.pattern_results.clear();
        }
    });

    ui.add_space(8.0);
    ui.separator();
    ui.add_space(8.0);

    if app.pattern_results.is_empty() {
        ui.label(egui::RichText::new("No results found or pattern not scanned yet.").weak());
        return;
    }

    ui.label(format!("Found {} occurrences.", app.pattern_results.len()));
    ui.add_space(8.0);

    let text_height = egui::TextStyle::Body.resolve(ui.style()).size;
    let num_rows = app.pattern_results.len();

    egui::ScrollArea::vertical().show_rows(ui, text_height, num_rows, |ui, row_range| {
        for row in row_range {
            let offset = app.pattern_results[row];
            
            let bg_color = if row % 2 == 1 {
                egui::Color32::from_rgb(37, 37, 38)
            } else {
                egui::Color32::TRANSPARENT
            };

            egui::Frame::none()
                .fill(bg_color)
                .show(ui, |ui| {
                    ui.set_min_width(ui.available_width());
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new(format!("{:08X}", offset)).monospace().color(egui::Color32::LIGHT_BLUE));
                        
                        // Show a preview of the bytes at that offset
                        if let Some(data) = &app.binary_data {
                            let end = (offset + 16).min(data.len());
                            let bytes = &data[offset..end];
                            let hex_preview: Vec<String> = bytes.iter().map(|b| format!("{:02X}", b)).collect();
                            ui.label(egui::RichText::new(hex_preview.join(" ")).monospace().color(egui::Color32::from_rgb(204, 204, 204)));
                        }
                    });
                });
        }
    });
}
