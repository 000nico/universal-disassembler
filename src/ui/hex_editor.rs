use eframe::egui;
use crate::ui::app::App;

pub fn show(app: &mut App, ui: &mut egui::Ui) {
    ui.add_space(8.0);
    ui.heading("⬚ Hex Editor");
    ui.add_space(8.0);

    if let Some(data) = &app.binary_data {
        let total_bytes = data.len();
        let bytes_per_row: usize = 16;
        let total_rows = (total_bytes + bytes_per_row - 1) / bytes_per_row;

        ui.label(egui::RichText::new(format!(
            "{} bytes  ·  {} rows",
            total_bytes, total_rows
        )).weak());
        ui.add_space(4.0);

        // ── Column header ──
        let header = build_header_layout();
        ui.label(header);
        ui.separator();

        // ── Virtual-scrolled hex content ──
        let row_height = 16.0;

        egui::ScrollArea::vertical().show_rows(ui, row_height, total_rows, |ui, row_range| {
            for row_idx in row_range {
                let start = row_idx * bytes_per_row;
                let end = (start + bytes_per_row).min(total_bytes);
                let row_bytes = &data[start..end];
                let job = hex_row_layout(start, row_bytes);
                
                let bg_color = if row_idx % 2 == 1 {
                    egui::Color32::from_rgb(37, 37, 38)
                } else {
                    egui::Color32::TRANSPARENT
                };

                egui::Frame::none()
                    .fill(bg_color)
                    .show(ui, |ui| {
                        ui.set_min_width(ui.available_width());
                        ui.label(job);
                    });
            }
        });
    } else {
        ui.label(egui::RichText::new("No binary data loaded.").weak().italics());
    }
}

fn build_header_layout() -> egui::text::LayoutJob {
    use egui::{Color32, FontFamily, FontId};
    use egui::text::{LayoutJob, TextFormat};

    let font = FontId::new(12.0, FontFamily::Monospace);
    let fmt = TextFormat {
        font_id: font.clone(),
        color: Color32::from_rgb(120, 120, 120),
        ..Default::default()
    };

    let mut job = LayoutJob::default();
    job.append("  Offset    ", 0.0, fmt.clone());
    for i in 0u8..16 {
        job.append(&format!("{:02X} ", i), 0.0, fmt.clone());
        if i == 7 {
            job.append(" ", 0.0, fmt.clone());
        }
    }
    job.append("  ASCII", 0.0, fmt);
    job
}

fn hex_row_layout(offset: usize, bytes: &[u8]) -> egui::text::LayoutJob {
    use egui::{Color32, FontFamily, FontId};
    use egui::text::{LayoutJob, TextFormat};

    let font = FontId::new(12.0, FontFamily::Monospace);
    let mut job = LayoutJob::default();

    // ── Offset column ──
    job.append(
        &format!("{:08X}    ", offset),
        0.0,
        TextFormat {
            font_id: font.clone(),
            color: Color32::from_rgb(106, 135, 195),
            ..Default::default()
        },
    );

    // ── Hex bytes ──
    for (i, &b) in bytes.iter().enumerate() {
        if i == 8 {
            job.append(" ", 0.0, TextFormat { font_id: font.clone(), ..Default::default() });
        }
        job.append(
            &format!("{:02X} ", b),
            0.0,
            TextFormat {
                font_id: font.clone(),
                color: byte_color(b),
                ..Default::default()
            },
        );
    }

    // ── Padding for incomplete last row ──
    if bytes.len() < 16 {
        if bytes.len() <= 8 {
            job.append(" ", 0.0, TextFormat { font_id: font.clone(), ..Default::default() });
        }
        for _ in bytes.len()..16 {
            job.append("   ", 0.0, TextFormat { font_id: font.clone(), ..Default::default() });
        }
    }

    // ── ASCII column ──
    job.append("  ", 0.0, TextFormat { font_id: font.clone(), ..Default::default() });
    for &b in bytes {
        let (ch, color) = if b >= 0x20 && b <= 0x7E {
            (b as char, Color32::from_rgb(181, 206, 168)) // VSCode string green
        } else {
            ('.', Color32::from_rgb(90, 90, 90))
        };
        job.append(
            &ch.to_string(),
            0.0,
            TextFormat {
                font_id: font.clone(),
                color,
                ..Default::default()
            },
        );
    }

    job
}

fn byte_color(b: u8) -> egui::Color32 {
    use egui::Color32;
    if b == 0x00 {
        Color32::from_rgb(90, 90, 90)       // dim for zero
    } else if b >= 0x20 && b <= 0x7E {
        Color32::from_rgb(204, 204, 204)     // white for printable ASCII
    } else {
        Color32::from_rgb(200, 140, 70)      // orange/yellow for non-printable
    }
}
