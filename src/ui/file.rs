use eframe::egui;
use crate::{formats::{Format, detect_format, pe::parse_pe, raw::open_file}, ui::app::App};

pub fn show(app: &mut App, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(ctx.screen_rect().height() / 3.0);

            // Modern Drop Zone style
            egui::Frame::none()
                .fill(egui::Color32::from_rgb(45, 45, 45))
                .stroke(egui::Stroke::new(2.0, egui::Color32::from_rgb(80, 80, 80)))
                .inner_margin(40.0)
                .rounding(8.0)
                .show(ui, |ui| {
                    ui.heading(egui::RichText::new("Drop a binary here").size(24.0).strong().color(egui::Color32::from_rgb(204, 204, 204)));
                    ui.add_space(10.0);
                    ui.label(egui::RichText::new("or click the button below to browse").size(14.0).weak());
                    
                    ui.add_space(20.0);

                    let btn = egui::Button::new(egui::RichText::new("Open Binary").size(16.0))
                        .fill(egui::Color32::from_rgb(14, 99, 156))
                        .min_size(egui::vec2(150.0, 40.0));

                    if ui.add(btn).clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            let path_str = path.display().to_string();

                            if let Ok(bytes) = std::fs::read(&path) {
                                app.format = Some(detect_format(&bytes));

                                match app.format {
                                    Some(Format::PE) => {
                                        match parse_pe(&bytes) {
                                            Ok(pe) => app.pe = Some(pe),
                                            Err(e) => eprintln!("Error parsing PE: {}", e),
                                        }
                                    }
                                    _ => {}
                                }

                                app.binary_data = Some(bytes.clone());
                                app.strings = crate::tools::strings::string_scanner(&bytes);
                            }

                            app.file_path = Some(path_str);
                        }
                    }
                });

            if let Some(path) = &app.file_path {
                ui.add_space(20.0);
                ui.label(egui::RichText::new(format!("Loaded: {}", path)).color(egui::Color32::from_rgb(100, 200, 100)));
            }
        });
    });
}