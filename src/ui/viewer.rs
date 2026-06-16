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
}

pub fn show(app: &mut App, ctx: &egui::Context) {
    // Custom sidebar frame to match VSCode dark theme
    let sidebar_frame = egui::Frame::none()
        .fill(egui::Color32::from_rgb(37, 37, 38)) // #252526
        .inner_margin(8.0);

    egui::SidePanel::left("sidebar")
        .frame(sidebar_frame)
        .resizable(false)
        .exact_width(180.0)
        .show(ctx, |ui| {
            ui.add_space(8.0);

            // Info del binario arriba
            if let Some(path) = &app.file_path {
                let name = std::path::Path::new(path)
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or(path);
                ui.label(egui::RichText::new(name).strong().size(13.0).color(egui::Color32::WHITE));
            }
            if let Some(fmt) = &app.format {
                ui.label(egui::RichText::new(format!("{:?}", fmt)).size(11.0).color(egui::Color32::from_rgb(150, 150, 150)));
            }

            ui.add_space(12.0);

            // Items de navegación
            sidebar_item(ui, &mut app.selected_view, SidebarItem::Overview,     "⬡  Overview",     None);
            sidebar_item(ui, &mut app.selected_view, SidebarItem::HexEditor,    "⬚  Hex Editor",   None);
            sidebar_item(ui, &mut app.selected_view, SidebarItem::Disassembly,  "⊟  Disassembly",  None);
            sidebar_item(ui, &mut app.selected_view, SidebarItem::PatternScanner,"⌕  Pattern Scanner", None);
            sidebar_item(ui, &mut app.selected_view, SidebarItem::Strings,      "⊞  Strings",      None);
            
            ui.add_space(8.0);
            ui.label(egui::RichText::new("STRUCTURE").size(10.0).strong().color(egui::Color32::from_rgb(100, 100, 100)));
            ui.add_space(4.0);
            
            sidebar_item(ui, &mut app.selected_view, SidebarItem::Headers,      "⊡  Headers",      None);

            // Con contadores (ejemplo con PE)
            let sections_count = pe_sections_count(&app.pe);
            let imports_count  = pe_imports_count(&app.pe);
            let exports_count  = pe_exports_count(&app.pe);

            sidebar_item(ui, &mut app.selected_view, SidebarItem::Sections,  "⊞  Sections",  sections_count);
            sidebar_item(ui, &mut app.selected_view, SidebarItem::Imports,   "↓  Imports",   imports_count);
            sidebar_item(ui, &mut app.selected_view, SidebarItem::Exports,   "↑  Exports",   exports_count);
            sidebar_item(ui, &mut app.selected_view, SidebarItem::Resources, "⬜  Resources", None);
        });

    // Panel principal frame
    let main_frame = egui::Frame::none()
        .fill(egui::Color32::from_rgb(30, 30, 30))
        .inner_margin(16.0);

    egui::CentralPanel::default().frame(main_frame).show(ctx, |ui| {
        match app.selected_view {
            SidebarItem::Overview    => crate::ui::overview::show(app, ui),
            SidebarItem::HexEditor   => crate::ui::hex_editor::show(app, ui),
            SidebarItem::Disassembly => crate::ui::disasm_view::show(app, ui),
            SidebarItem::Strings     => crate::ui::strings_view::show(app, ui),
            SidebarItem::Headers     => crate::ui::headers::show(app, ui),
            SidebarItem::PatternScanner => crate::ui::patterns_view::show(app, ui),
            SidebarItem::Sections    => crate::ui::sections::show(app, ui),
            SidebarItem::Imports     => crate::ui::imports::show(app, ui),
            SidebarItem::Exports     => crate::ui::exports::show(app, ui),
            SidebarItem::Resources   => { ui.label("Resources — WIP"); }
        };
    });
}

fn sidebar_item(
    ui: &mut egui::Ui,
    selected: &mut SidebarItem,
    item: SidebarItem,
    label: &str,
    count: Option<usize>,
) {
    let is_selected = *selected == item;
    let text = egui::RichText::new(label).size(12.0);

    ui.horizontal(|ui| {
        let btn = ui.selectable_label(is_selected, text);
        if let Some(n) = count {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(egui::RichText::new(n.to_string()).size(10.0).weak());
            });
        }
        if btn.clicked() {
            *selected = item;
        }
    });
}

// Helpers para contar desde PE
fn pe_sections_count(pe: &Option<crate::formats::pe::PE>) -> Option<usize> {
    pe.as_ref().map(|p| match p {
        PE::PE32(p) => p.sections.len(),
        PE::PE64(p) => p.sections.len(),
    })
}

fn pe_imports_count(pe: &Option<crate::formats::pe::PE>) -> Option<usize> {
    pe.as_ref().map(|p| match p {
        PE::PE32(p) => p.import_directories.len(),
        PE::PE64(p) => p.import_directories.len(),
    })
}

fn pe_exports_count(pe: &Option<crate::formats::pe::PE>) -> Option<usize> {
    // ExportDirectory no es un Vec, así que devolvemos 1 si hay exports
    pe.as_ref().map(|p| match p {
        PE::PE32(p) => if p.export_directory.number_of_functions > 0 { 1 } else { 0 },
        PE::PE64(p) => if p.export_directory.number_of_functions > 0 { 1 } else { 0 },
    })
}