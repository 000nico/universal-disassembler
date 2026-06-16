use eframe::egui;
use crate::ui::app::App;

const BLOCK_SIZES: &[usize] = &[64, 128, 256, 512, 1024, 2048, 4096];

pub fn show(app: &mut App, ui: &mut egui::Ui) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.add_space(4.0);

        // ── Header ──
        section_heading(ui, "◊", "Entropy Analysis");
        ui.add_space(12.0);

        if app.binary_data.is_none() {
            ui.label(egui::RichText::new("No binary loaded. Open a file to analyze entropy.")
                .weak().italics());
            return;
        }

        // ── Block size selector ──
        let old_block_size = app.entropy_block_size;
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("Block Size:").strong().size(12.0));
            let current_label = format!("{} bytes", app.entropy_block_size);
            egui::ComboBox::from_id_salt("entropy_block_size_combo")
                .selected_text(current_label)
                .show_ui(ui, |ui| {
                    for &size in BLOCK_SIZES {
                        let label = format!("{} bytes", size);
                        ui.selectable_value(&mut app.entropy_block_size, size, label);
                    }
                });
        });

        // Recompute if block size changed
        if app.entropy_block_size != old_block_size {
            if let Some(data) = &app.binary_data {
                app.entropy_blocks = crate::tools::entropy::entropy_blocks(data, app.entropy_block_size);
            }
        }

        ui.add_space(12.0);

        if app.entropy_blocks.is_empty() {
            ui.label(egui::RichText::new("No entropy data computed.").weak());
            return;
        }

        // ── Statistics card ──
        let blocks = &app.entropy_blocks;
        let min_e = blocks.iter().cloned().fold(f64::MAX, f64::min);
        let max_e = blocks.iter().cloned().fold(f64::MIN, f64::max);
        let avg_e = blocks.iter().sum::<f64>() / blocks.len() as f64;

        card(ui, |ui| {
            ui.horizontal(|ui| {
                stat_item(ui, "Total Blocks", &blocks.len().to_string());
                ui.add_space(28.0);
                stat_item(ui, "Minimum", &format!("{:.3}", min_e));
                ui.add_space(28.0);
                stat_item(ui, "Maximum", &format!("{:.3}", max_e));
                ui.add_space(28.0);
                stat_item(ui, "Average", &format!("{:.3}", avg_e));
                ui.add_space(28.0);
                stat_item(ui, "Block Size", &format!("{} B", app.entropy_block_size));
            });
        });

        ui.add_space(16.0);

        // ── Color legend ──
        draw_legend(ui);
        ui.add_space(12.0);

        // ── Entropy bar chart ──
        draw_entropy_chart(ui, &app.entropy_blocks, app.entropy_block_size);
    });
}

// ── UI Components ──

fn section_heading(ui: &mut egui::Ui, icon: &str, text: &str) {
    ui.horizontal(|ui| {
        let (bar_rect, _) = ui.allocate_exact_size(egui::vec2(3.0, 18.0), egui::Sense::hover());
        ui.painter().rect_filled(bar_rect, 1.5, egui::Color32::from_rgb(86, 156, 214));
        ui.add_space(6.0);
        ui.label(egui::RichText::new(format!("{} {}", icon, text))
            .strong().size(18.0).color(egui::Color32::from_rgb(220, 224, 230)));
    });
}

fn card(ui: &mut egui::Ui, content: impl FnOnce(&mut egui::Ui)) {
    egui::Frame::none()
        .fill(egui::Color32::from_rgb(30, 30, 38))
        .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(44, 46, 58)))
        .inner_margin(16.0)
        .rounding(8.0)
        .show(ui, content);
}

fn stat_item(ui: &mut egui::Ui, label: &str, value: &str) {
    ui.vertical(|ui| {
        ui.label(egui::RichText::new(label).size(10.0)
            .color(egui::Color32::from_rgb(120, 125, 140)));
        ui.label(egui::RichText::new(value).size(16.0).strong().monospace());
    });
}

pub fn entropy_color(e: f64) -> egui::Color32 {
    let t = (e / 8.0).clamp(0.0, 1.0) as f32;
    if t < 0.375 {
        // Green → Yellow
        let s = t / 0.375;
        egui::Color32::from_rgb(
            (60.0 + 195.0 * s) as u8,
            (200.0 - 10.0 * s) as u8,
            (80.0 - 60.0 * s) as u8,
        )
    } else {
        // Yellow → Red
        let s = (t - 0.375) / 0.625;
        egui::Color32::from_rgb(
            255,
            (190.0 - 170.0 * s) as u8,
            (20.0 - 20.0 * s) as u8,
        )
    }
}

fn draw_legend(ui: &mut egui::Ui) {
    let legend_width = 320.0_f32;
    let legend_height = 14.0_f32;
    let num_steps = 80;

    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("0.0").size(10.0)
            .color(egui::Color32::from_rgb(120, 125, 140)).monospace());
        ui.add_space(4.0);

        let (response, painter) = ui.allocate_painter(
            egui::vec2(legend_width, legend_height),
            egui::Sense::hover(),
        );
        let rect = response.rect;

        // Draw gradient bar
        let step_w = legend_width / num_steps as f32;
        for i in 0..num_steps {
            let e = (i as f64 / num_steps as f64) * 8.0;
            let color = entropy_color(e);
            let x0 = rect.left() + i as f32 * step_w;
            let x1 = x0 + step_w + 0.5; // slight overlap to avoid gaps
            painter.rect_filled(
                egui::Rect::from_min_max(
                    egui::pos2(x0, rect.top()),
                    egui::pos2(x1.min(rect.right()), rect.bottom()),
                ),
                if i == 0 { 2.0 } else if i == num_steps - 1 { 2.0 } else { 0.0 },
                color,
            );
        }

        // Round the overall shape
        painter.rect_stroke(rect, 3.0, egui::Stroke::new(1.0, egui::Color32::from_rgb(44, 46, 58)), egui::StrokeKind::Inside);

        ui.add_space(4.0);
        ui.label(egui::RichText::new("8.0").size(10.0)
            .color(egui::Color32::from_rgb(120, 125, 140)).monospace());
        ui.add_space(12.0);
        ui.label(egui::RichText::new("Shannon entropy (bits/byte)").size(10.0)
            .color(egui::Color32::from_rgb(100, 105, 120)));
    });
}

fn draw_entropy_chart(ui: &mut egui::Ui, blocks: &[f64], block_size: usize) {
    let num_blocks = blocks.len();
    if num_blocks == 0 { return; }

    let chart_height = 240.0_f32;
    let available_width = ui.available_width();
    let bar_width = (available_width / num_blocks as f32).clamp(2.0, 18.0);
    let total_width = bar_width * num_blocks as f32;

    egui::ScrollArea::horizontal().show(ui, |ui| {
        let (response, painter) = ui.allocate_painter(
            egui::vec2(total_width.max(available_width), chart_height),
            egui::Sense::hover(),
        );
        let rect = response.rect;

        // Background
        painter.rect_filled(rect, 6.0, egui::Color32::from_rgb(22, 22, 30));
        painter.rect_stroke(rect, 6.0, egui::Stroke::new(1.0, egui::Color32::from_rgb(40, 42, 54)), egui::StrokeKind::Inside);

        // Grid lines
        for i in 0..=8 {
            let frac = i as f32 / 8.0;
            let y = rect.bottom() - frac * (chart_height - 8.0) - 4.0;
            painter.line_segment(
                [egui::pos2(rect.left() + 4.0, y), egui::pos2(rect.right() - 4.0, y)],
                egui::Stroke::new(0.4, egui::Color32::from_rgb(38, 40, 50)),
            );
            if i % 2 == 0 {
                painter.text(
                    egui::pos2(rect.left() + 6.0, y - 8.0),
                    egui::Align2::LEFT_TOP,
                    format!("{}", i),
                    egui::FontId::proportional(9.0),
                    egui::Color32::from_rgb(70, 72, 85),
                );
            }
        }

        // Draw bars
        let usable_height = chart_height - 8.0;
        for (i, &entropy) in blocks.iter().enumerate() {
            let t = (entropy / 8.0).clamp(0.0, 1.0) as f32;
            let bar_h = t * usable_height;
            let color = entropy_color(entropy);

            let x0 = rect.left() + i as f32 * bar_width;
            let gap = if bar_width > 4.0 { 0.5 } else { 0.0 };

            if bar_h > 0.5 {
                let bar_rect = egui::Rect::from_min_max(
                    egui::pos2(x0 + gap, rect.bottom() - bar_h - 4.0),
                    egui::pos2(x0 + bar_width - gap, rect.bottom() - 4.0),
                );
                let rounding = if bar_width > 6.0 { 1.5 } else { 0.0 };
                painter.rect_filled(bar_rect, rounding, color);
            }
        }

        // Hover: highlight bar + collect info for tooltip
        let hover_info: Option<(usize, f64)> = response.hover_pos().and_then(|pos| {
            let rel_x = pos.x - rect.left();
            if rel_x >= 0.0 {
                let idx = (rel_x / bar_width) as usize;
                if idx < blocks.len() {
                    // Highlight hovered bar
                    let entropy = blocks[idx];
                    let t = (entropy / 8.0).clamp(0.0, 1.0) as f32;
                    let bar_h = t * usable_height;
                    let x0 = rect.left() + idx as f32 * bar_width;

                    if bar_h > 0.5 {
                        let highlight = egui::Rect::from_min_max(
                            egui::pos2(x0, rect.bottom() - bar_h - 4.0),
                            egui::pos2(x0 + bar_width, rect.bottom() - 4.0),
                        );
                        painter.rect_stroke(
                            highlight, 1.0,
                            egui::Stroke::new(1.5, egui::Color32::WHITE),
                            egui::StrokeKind::Inside,
                        );
                    }

                    // Vertical guide line
                    let cx = x0 + bar_width / 2.0;
                    painter.line_segment(
                        [egui::pos2(cx, rect.top() + 4.0), egui::pos2(cx, rect.bottom() - 4.0)],
                        egui::Stroke::new(0.5, egui::Color32::from_rgba_premultiplied(255, 255, 255, 40)),
                    );

                    Some((idx, entropy))
                } else {
                    None
                }
            } else {
                None
            }
        });

        // Show tooltip
        if let Some((idx, entropy)) = hover_info {
            let offset_start = idx * block_size;
            let offset_end = offset_start + block_size;
            let class = if entropy < 3.0 {
                "Low — structured / repetitive data"
            } else if entropy < 6.0 {
                "Medium — executable code / data"
            } else {
                "High — compressed / encrypted"
            };

            response.on_hover_ui_at_pointer(|ui| {
                ui.set_max_width(280.0);
                ui.label(egui::RichText::new(format!("Block #{}", idx)).strong().size(13.0));
                ui.add_space(2.0);
                ui.label(egui::RichText::new(format!(
                    "0x{:08X} — 0x{:08X}", offset_start, offset_end
                )).monospace().size(11.0).color(egui::Color32::from_rgb(150, 155, 170)));
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Entropy:").size(11.0)
                        .color(egui::Color32::from_rgb(150, 155, 170)));
                    ui.label(egui::RichText::new(format!("{:.4} bits/byte", entropy))
                        .monospace().size(12.0).strong().color(entropy_color(entropy)));
                });
                ui.add_space(2.0);
                ui.label(egui::RichText::new(class).size(11.0)
                    .color(entropy_color(entropy)));
            });
        }
    });

    // ── Hover info bar (always visible, shows when hovering) ──
    ui.add_space(8.0);
    ui.label(egui::RichText::new("Hover over bars to inspect individual blocks")
        .size(10.0).color(egui::Color32::from_rgb(90, 95, 110)).italics());
}
