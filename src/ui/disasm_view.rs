use eframe::egui;
use crate::ui::app::App;

pub fn show(app: &mut App, ui: &mut egui::Ui) {
    ui.add_space(8.0);
    ui.heading("⊟ Disassembly");
    ui.add_space(16.0);

    if app.instructions.is_empty() {
        ui.vertical_centered(|ui| {
            ui.add_space(40.0);
            ui.label(egui::RichText::new("No instructions disassembled").weak().italics().size(16.0));
            ui.add_space(8.0);
            ui.label("Load a valid binary to analyze its machine instructions.");
        });
        return;
    }

    // ── Search & Filter ──
    egui::Frame::none()
        .fill(egui::Color32::from_rgb(30, 30, 40))
        .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(50, 52, 66)))
        .inner_margin(12)
        .corner_radius(8)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("🔍").size(16.0));
                let response = ui.add(
                    egui::TextEdit::singleline(&mut app.disasm_filter)
                        .hint_text("Filter by mnemonic, operands or offset (e.g., mov, rbp, 0x1000)...")
                        .desired_width(ui.available_width() - 80.0)
                );
                if response.changed() {
                    // Filter changed
                }
                if ui.button("Clear").clicked() {
                    app.disasm_filter.clear();
                }
            });
        });

    ui.add_space(16.0);

    // Filtered instructions
    let query = app.disasm_filter.to_lowercase();
    let filtered: Vec<&crate::tools::Instruction> = app.instructions
        .iter()
        .filter(|inst| {
            if query.is_empty() {
                return true;
            }
            let offset_hex = format!("0x{:08x}", inst.offset);
            inst.mnemonic.to_lowercase().contains(&query)
                || inst.operands.to_lowercase().contains(&query)
                || offset_hex.contains(&query)
        })
        .collect();

    // Stats indicator
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(format!(
                "Showing {} / {} instructions",
                filtered.len(),
                app.instructions.len()
            ))
            .size(12.0)
            .color(egui::Color32::from_rgb(150, 155, 170)),
        );
    });
    ui.add_space(8.0);

    // ── Table Header ──
    let table_header_bg = egui::Color32::from_rgb(26, 26, 34);
    let border_color = egui::Color32::from_rgb(45, 46, 58);
    
    egui::Frame::none()
        .fill(table_header_bg)
        .stroke(egui::Stroke::new(1.0, border_color))
        .corner_radius(egui::CornerRadius {
            nw: 6,
            ne: 6,
            sw: 0,
            se: 0,
        })
        .inner_margin(egui::Margin::symmetric(12, 8))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.allocate_ui(egui::vec2(100.0, ui.available_height()), |ui| {
                    ui.label(egui::RichText::new("Offset").strong().color(egui::Color32::from_rgb(180, 185, 200)));
                });
                ui.allocate_ui(egui::vec2(160.0, ui.available_height()), |ui| {
                    ui.label(egui::RichText::new("Bytes").strong().color(egui::Color32::from_rgb(180, 185, 200)));
                });
                ui.label(egui::RichText::new("Instruction").strong().color(egui::Color32::from_rgb(180, 185, 200)));
            });
        });

    // ── Table Body (Virtual Scrolling) ──
    let row_height = 24.0;
    
    egui::Frame::none()
        .fill(egui::Color32::from_rgb(20, 20, 26))
        .stroke(egui::Stroke::new(1.0, border_color))
        .corner_radius(egui::CornerRadius {
            nw: 0,
            ne: 0,
            sw: 6,
            se: 6,
        })
        .show(ui, |ui| {
            egui::ScrollArea::vertical()
                .max_height(f32::INFINITY)
                .show_rows(ui, row_height, filtered.len(), |ui, row_range| {
                    for idx in row_range {
                        let inst = filtered[idx];
                        
                        // Alternating row background colors
                        let row_bg = if idx % 2 == 0 {
                            egui::Color32::from_rgb(22, 22, 30)
                        } else {
                            egui::Color32::from_rgb(26, 26, 36)
                        };

                        let (rect, response) = ui.allocate_exact_size(
                            egui::vec2(ui.available_width(), row_height),
                            egui::Sense::click_and_drag()
                        );

                        // Highlight hovered row
                        let fill_color = if response.hovered() {
                            egui::Color32::from_rgb(42, 44, 58)
                        } else {
                            row_bg
                        };

                        let painter = ui.painter_at(rect);
                        painter.rect_filled(rect, 0.0, fill_color);

                        // Custom row layout
                        let left_margin = rect.left() + 12.0;
                        let text_y = rect.center().y - 7.0; // center alignment offset

                        // 1. Offset
                        let offset_text = format!("0x{:08X}", inst.offset);
                        painter.text(
                            egui::pos2(left_margin, text_y),
                            egui::Align2::LEFT_TOP,
                            offset_text,
                            egui::FontId::monospace(12.0),
                            egui::Color32::from_rgb(78, 201, 176), // Teal for address
                        );

                        // 2. Bytes
                        let bytes_text = inst.bytes
                            .iter()
                            .map(|b| format!("{:02X}", b))
                            .collect::<Vec<String>>()
                            .join(" ");
                        let truncated_bytes = if bytes_text.len() > 22 {
                            format!("{}...", &bytes_text[..19])
                        } else {
                            bytes_text.clone()
                        };
                        painter.text(
                            egui::pos2(left_margin + 100.0, text_y),
                            egui::Align2::LEFT_TOP,
                            truncated_bytes,
                            egui::FontId::monospace(12.0),
                            egui::Color32::from_rgb(120, 125, 140), // Gray for bytes
                        );

                        // 3. Mnemonic & Operands
                        let mnem_color = match inst.mnemonic.as_str() {
                            "jmp" | "je" | "jne" | "jg" | "jl" | "jge" | "jle" | "js" | "jns" | "jz" | "jnz" | "call" | "ret" => {
                                egui::Color32::from_rgb(197, 134, 192) // Purple/pink for control flow
                            }
                            "add" | "sub" | "mul" | "div" | "inc" | "dec" | "shl" | "shr" | "xor" | "or" | "and" => {
                                egui::Color32::from_rgb(220, 220, 170) // Yellowish for math/logic
                            }
                            _ => egui::Color32::from_rgb(86, 156, 214), // Cyan/blue for other instructions (e.g. mov, push, pop)
                        };

                        painter.text(
                            egui::pos2(left_margin + 260.0, text_y),
                            egui::Align2::LEFT_TOP,
                            &inst.mnemonic,
                            egui::FontId::monospace(12.0),
                            mnem_color,
                        );

                        // Operands
                        // Compute offset for operands
                        let mnem_width = inst.mnemonic.len() as f32 * 7.5;
                        painter.text(
                            egui::pos2(left_margin + 260.0 + mnem_width + 12.0, text_y),
                            egui::Align2::LEFT_TOP,
                            &inst.operands,
                            egui::FontId::monospace(12.0),
                            egui::Color32::from_rgb(220, 225, 235), // White-ish for operands
                        );

                        // Optional: Tooltip on hover
                        response.on_hover_ui(|ui| {
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new("Full Bytes:").strong());
                                ui.label(egui::RichText::new(&bytes_text).monospace());
                            });
                        });
                    }
                });
        });
}
