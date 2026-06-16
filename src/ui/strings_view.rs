use eframe::egui;
use crate::ui::app::App;

pub fn show(app: &mut App, ui: &mut egui::Ui) {
    ui.add_space(8.0);
    ui.heading("⊞ Strings");
    ui.add_space(8.0);

    if app.strings.is_empty() {
        ui.label(egui::RichText::new("No strings extracted. Load a binary file to scan for strings.").weak().italics());
        return;
    }

    // ── Filter ──
    ui.horizontal(|ui| {
        ui.label("Filter:");
        ui.add(egui::TextEdit::singleline(&mut app.string_filter).desired_width(300.0));
    });
    ui.add_space(4.0);

    // ── Build filtered list ──
    let filter_lower = app.string_filter.to_lowercase();
    let filtered: Vec<&(usize, String)> = if filter_lower.is_empty() {
        app.strings.iter().collect()
    } else {
        app.strings.iter()
            .filter(|(_, s)| s.to_lowercase().contains(&filter_lower))
            .collect()
    };

    ui.label(egui::RichText::new(format!(
        "Showing {} of {} strings",
        filtered.len(),
        app.strings.len()
    )).weak());
    ui.add_space(4.0);
    ui.separator();
    ui.add_space(4.0);

    // ── Column header ──
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("  Offset        ").monospace().strong());
        ui.label(egui::RichText::new("Len   ").monospace().strong());
        ui.label(egui::RichText::new("String").monospace().strong());
    });
    ui.separator();

    // ── Virtual-scrolled content ──
    let row_height = 18.0;
    let total = filtered.len();

    egui::ScrollArea::vertical().show_rows(ui, row_height, total, |ui, row_range| {
        for row in row_range {
            let &(offset, ref s) = filtered[row];
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
                            ui.label(
                                egui::RichText::new(format!("0x{:08X}", offset))
                                    .monospace()
                                    .color(egui::Color32::from_rgb(106, 135, 195)),
                            );
                            ui.label(
                                egui::RichText::new(format!("{:>5}", s.len()))
                                    .monospace()
                                    .weak(),
                            );
                            ui.label(egui::RichText::new(s.as_str()).monospace().color(egui::Color32::from_rgb(181, 206, 168)));
                        });
                    });
        }
    });
}
