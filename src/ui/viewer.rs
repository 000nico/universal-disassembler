use eframe::egui;
use crate::ui::app::App;
use crate::formats::pe::PE;

#[derive(PartialEq, Default)]
pub enum SidebarItem {
    #[default]
    Overview,
    HexEditor,
    Disassembly,
    Strings,
    Headers,
    Sections,
    Imports,
    Exports,
    PatternScanner,
    Resources,
    Entropy,
}

pub fn show(app: &mut App, ctx: &egui::Context) {
    let sidebar_frame = egui::Frame::none()
        .fill(egui::Color32::from_rgb(24, 24, 32))
        .inner_margin(4.0);

    egui::SidePanel::left("sidebar")
        .frame(sidebar_frame)
        .resizable(false)
        .exact_width(210.0)
        .show(ctx, |ui| {
            ui.add_space(12.0);

            // ── File info header ──
            ui.horizontal(|ui| {
                ui.add_space(14.0);
                ui.vertical(|ui| {
                    if let Some(path) = &app.file_path {
                        let name = std::path::Path::new(path)
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or(path);
                        ui.label(
                            egui::RichText::new(name)
                                .strong()
                                .size(13.0)
                                .color(egui::Color32::WHITE),
                        );
                    }
                    if let Some(fmt) = &app.format {
                        ui.label(
                            egui::RichText::new(format!("{:?}", fmt))
                                .size(11.0)
                                .color(egui::Color32::from_rgb(86, 156, 214)),
                        );
                    }
                });
            });

            ui.add_space(12.0);

            // Separator
            let (sep_rect, _) =
                ui.allocate_exact_size(egui::vec2(ui.available_width(), 1.0), egui::Sense::hover());
            ui.painter()
                .rect_filled(sep_rect, 0.0, egui::Color32::from_rgb(40, 42, 54));

            ui.add_space(12.0);

            // ── ANALYSIS section ──
            section_label(ui, "ANALYSIS");
            sidebar_item(ui, &mut app.selected_view, SidebarItem::Overview,       "⬡  Overview",        None);
            sidebar_item(ui, &mut app.selected_view, SidebarItem::HexEditor,      "⬚  Hex Editor",      None);
            sidebar_item(ui, &mut app.selected_view, SidebarItem::Disassembly,    "⊟  Disassembly",     None);
            sidebar_item(ui, &mut app.selected_view, SidebarItem::PatternScanner, "⌕  Pattern Scanner", None);
            sidebar_item(ui, &mut app.selected_view, SidebarItem::Strings,        "⊞  Strings",         None);
            sidebar_item(ui, &mut app.selected_view, SidebarItem::Entropy,        "◊  Entropy",         None);

            ui.add_space(12.0);

            // ── STRUCTURE section ──
            section_label(ui, "STRUCTURE");
            sidebar_item(ui, &mut app.selected_view, SidebarItem::Headers,  "⊡  Headers",   None);

            let sections_count = pe_sections_count(&app.pe);
            let imports_count  = pe_imports_count(&app.pe);
            let exports_count  = pe_exports_count(&app.pe);

            sidebar_item(ui, &mut app.selected_view, SidebarItem::Sections,  "⊞  Sections",  sections_count);
            sidebar_item(ui, &mut app.selected_view, SidebarItem::Imports,   "↓  Imports",   imports_count);
            sidebar_item(ui, &mut app.selected_view, SidebarItem::Exports,   "↑  Exports",   exports_count);
            sidebar_item(ui, &mut app.selected_view, SidebarItem::Resources, "⬜  Resources", None);
        });

    // ── Main content panel ──
    let main_frame = egui::Frame::none()
        .fill(egui::Color32::from_rgb(22, 22, 28))
        .inner_margin(20.0);

    egui::CentralPanel::default()
        .frame(main_frame)
        .show(ctx, |ui| {
            match app.selected_view {
                SidebarItem::Overview       => crate::ui::overview::show(app, ui),
                SidebarItem::HexEditor      => crate::ui::hex_editor::show(app, ui),
                SidebarItem::Disassembly    => crate::ui::disasm_view::show(app, ui),
                SidebarItem::Strings        => crate::ui::strings_view::show(app, ui),
                SidebarItem::Headers        => crate::ui::headers::show(app, ui),
                SidebarItem::PatternScanner => crate::ui::patterns_view::show(app, ui),
                SidebarItem::Sections       => crate::ui::sections::show(app, ui),
                SidebarItem::Imports        => crate::ui::imports::show(app, ui),
                SidebarItem::Exports        => crate::ui::exports::show(app, ui),
                SidebarItem::Entropy        => crate::ui::entropy_view::show(app, ui),
                SidebarItem::Resources      => {
                    ui.label(
                        egui::RichText::new("Resources — WIP")
                            .weak()
                            .italics(),
                    );
                }
            };
        });
}

// ── Sidebar components ──

fn section_label(ui: &mut egui::Ui, text: &str) {
    ui.horizontal(|ui| {
        ui.add_space(14.0);
        ui.label(
            egui::RichText::new(text)
                .size(10.0)
                .strong()
                .color(egui::Color32::from_rgb(80, 85, 100)),
        );
    });
    ui.add_space(4.0);
}

fn sidebar_item(
    ui: &mut egui::Ui,
    selected: &mut SidebarItem,
    item: SidebarItem,
    label: &str,
    count: Option<usize>,
) {
    let is_selected = *selected == item;

    let desired_size = egui::vec2(ui.available_width(), 30.0);
    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

    if ui.is_rect_visible(rect) {
        let painter = ui.painter();

        // Background fill
        let bg = if is_selected {
            egui::Color32::from_rgb(32, 34, 48)
        } else if response.hovered() {
            egui::Color32::from_rgb(30, 30, 42)
        } else {
            egui::Color32::TRANSPARENT
        };
        painter.rect_filled(rect, 0.0, bg);

        // Left accent bar for selected item
        if is_selected {
            let bar = egui::Rect::from_min_max(
                rect.left_top(),
                egui::pos2(rect.left() + 3.0, rect.bottom()),
            );
            painter.rect_filled(bar, 1.5, egui::Color32::from_rgb(86, 156, 214));
        }

        // Label text
        let text_color = if is_selected {
            egui::Color32::WHITE
        } else if response.hovered() {
            egui::Color32::from_rgb(210, 214, 220)
        } else {
            egui::Color32::from_rgb(150, 155, 170)
        };

        painter.text(
            egui::pos2(rect.left() + 16.0, rect.center().y),
            egui::Align2::LEFT_CENTER,
            label,
            egui::FontId::proportional(12.0),
            text_color,
        );

        // Count badge
        if let Some(n) = count {
            painter.text(
                egui::pos2(rect.right() - 14.0, rect.center().y),
                egui::Align2::RIGHT_CENTER,
                n.to_string(),
                egui::FontId::proportional(10.0),
                egui::Color32::from_rgb(90, 95, 110),
            );
        }
    }

    if response.clicked() {
        *selected = item;
    }
}

// ── PE helpers ──

fn pe_sections_count(pe: &Option<PE>) -> Option<usize> {
    pe.as_ref().map(|p| match p {
        PE::PE32(p) => p.sections.len(),
        PE::PE64(p) => p.sections.len(),
    })
}

fn pe_imports_count(pe: &Option<PE>) -> Option<usize> {
    pe.as_ref().map(|p| match p {
        PE::PE32(p) => p.import_directories.len(),
        PE::PE64(p) => p.import_directories.len(),
    })
}

fn pe_exports_count(pe: &Option<PE>) -> Option<usize> {
    pe.as_ref().map(|p| match p {
        PE::PE32(p) => {
            if p.export_directory.number_of_functions > 0 { 1 } else { 0 }
        }
        PE::PE64(p) => {
            if p.export_directory.number_of_functions > 0 { 1 } else { 0 }
        }
    })
}