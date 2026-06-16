use eframe::egui;
use crate::{formats::{Format, detect_format, pe::parse_pe, raw::open_file, macho::parse_macho}, ui::app::App};

pub fn show(app: &mut App, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(ctx.screen_rect().height() / 3.5);

            // ── Premium drop zone card ──
            egui::Frame::none()
                .fill(egui::Color32::from_rgb(30, 30, 40))
                .stroke(egui::Stroke::new(1.5, egui::Color32::from_rgb(55, 58, 75)))
                .inner_margin(52.0)
                .rounding(14.0)
                .show(ui, |ui| {
                    // Large icon
                    ui.label(egui::RichText::new("⬢")
                        .size(56.0)
                        .color(egui::Color32::from_rgb(86, 156, 214)));
                    ui.add_space(12.0);

                    ui.heading(egui::RichText::new("Drop a binary here")
                        .size(26.0)
                        .strong()
                        .color(egui::Color32::from_rgb(220, 224, 230)));
                    ui.add_space(6.0);
                    ui.label(egui::RichText::new("Supports PE, ELF, Mach-O and raw binaries")
                        .size(13.0)
                        .color(egui::Color32::from_rgb(120, 125, 145)));

                    ui.add_space(28.0);

                    let btn = egui::Button::new(
                        egui::RichText::new("  Open Binary  ")
                            .size(15.0)
                            .strong()
                            .color(egui::Color32::WHITE),
                    )
                    .fill(egui::Color32::from_rgb(86, 156, 214))
                    .min_size(egui::vec2(180.0, 44.0));

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
                                    Some(Format::MachO) => {
                                        match parse_macho(&bytes) {
                                            Ok(m) => app.macho = Some(m),
                                            Err(e) => eprintln!("Error parsing Mach-O: {}", e),
                                        }
                                    }
                                    _ => {}
                                }

                                // Compute file hashes
                                let md5_digest = crate::tools::hashes::getMD5(&bytes);
                                app.hash_md5 = format!("{:x}", md5_digest);

                                let sha1_digest = crate::tools::hashes::get_sha1(&bytes);
                                app.hash_sha1 = sha1_digest
                                    .iter()
                                    .map(|b| format!("{:02x}", b))
                                    .collect::<String>();

                                let sha256_digest = crate::tools::hashes::get_sha256(&bytes);
                                app.hash_sha256 = sha256_digest
                                    .iter()
                                    .map(|b| format!("{:02x}", b))
                                    .collect::<String>();

                                // Compute entropy blocks
                                app.entropy_blocks = crate::tools::entropy::entropy_blocks(
                                    &bytes,
                                    app.entropy_block_size,
                                );

                                // Extract strings
                                app.strings = crate::tools::strings::string_scanner(&bytes);

                                // Store binary data (last, after all reads)
                                app.binary_data = Some(bytes);
                            }

                            app.file_path = Some(path_str);
                        }
                    }
                });

            if let Some(path) = &app.file_path {
                ui.add_space(16.0);
                ui.label(
                    egui::RichText::new(format!("✓  {}", path))
                        .size(12.0)
                        .color(egui::Color32::from_rgb(78, 201, 176)),
                );
            }
        });
    });
}