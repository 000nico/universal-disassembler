use eframe::egui;

use crate::{formats::{Format, pe::PE, macho::MachO}, ui::viewer::SidebarItem};

pub struct App {
    pub file_path: Option<String>,
    pub binary_data: Option<Vec<u8>>,
    pub format: Option<Format>,
    pub pe: Option<PE>,
    pub macho: Option<MachO>,
    pub selected_view: SidebarItem,
    pub strings: Vec<(usize, String)>,
    pub string_filter: String,
    pub pattern_input: String,
    pub pattern_results: Vec<usize>,
    pub entropy_block_size: usize,
    pub entropy_blocks: Vec<f64>,
    pub hash_md5: String,
    pub hash_sha1: String,
    pub hash_sha256: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            file_path: None,
            binary_data: None,
            format: None,
            pe: None,
            macho: None,
            selected_view: SidebarItem::default(),
            strings: Vec::new(),
            string_filter: String::new(),
            pattern_input: String::new(),
            pattern_results: Vec::new(),
            entropy_block_size: 256,
            entropy_blocks: Vec::new(),
            hash_md5: String::new(),
            hash_sha1: String::new(),
            hash_sha256: String::new(),
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, _ui: &mut egui::Ui, _frame: &mut eframe::Frame) {}

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut visuals = egui::Visuals::dark();

        // ── Premium dark palette with subtle blue undertone ──
        let bg = egui::Color32::from_rgb(22, 22, 28);
        let panel = egui::Color32::from_rgb(28, 28, 36);
        let surface = egui::Color32::from_rgb(36, 36, 46);
        let text = egui::Color32::from_rgb(210, 214, 220);
        let accent = egui::Color32::from_rgb(86, 156, 214);
        let border = egui::Color32::from_rgb(48, 50, 62);

        visuals.override_text_color = Some(text);
        visuals.panel_fill = panel;
        visuals.window_fill = bg;

        visuals.widgets.noninteractive.bg_fill = bg;
        visuals.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0, text);
        visuals.widgets.noninteractive.bg_stroke = egui::Stroke::new(0.5, border);
        visuals.widgets.noninteractive.corner_radius = egui::CornerRadius::same(6);

        visuals.widgets.inactive.bg_fill = surface;
        visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(180, 184, 190));
        visuals.widgets.inactive.bg_stroke = egui::Stroke::new(0.5, border);
        visuals.widgets.inactive.corner_radius = egui::CornerRadius::same(6);

        visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(50, 52, 66);
        visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.0, egui::Color32::WHITE);
        visuals.widgets.hovered.bg_stroke = egui::Stroke::new(1.0, accent);
        visuals.widgets.hovered.corner_radius = egui::CornerRadius::same(6);

        visuals.widgets.active.bg_fill = accent;
        visuals.widgets.active.fg_stroke = egui::Stroke::new(1.0, egui::Color32::WHITE);
        visuals.widgets.active.corner_radius = egui::CornerRadius::same(6);

        visuals.selection.bg_fill = egui::Color32::from_rgb(40, 68, 100);
        visuals.selection.stroke = egui::Stroke::new(1.0, accent);

        ctx.set_visuals(visuals);

        let mut style = (*ctx.style()).clone();
        style.spacing.item_spacing = egui::vec2(8.0, 6.0);
        style.spacing.button_padding = egui::vec2(12.0, 6.0);
        ctx.set_style(style);

        super::show(self, ctx);
    }
}