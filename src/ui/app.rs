use eframe::egui;

use crate::{formats::{Format, pe::PE}, ui::viewer::SidebarItem};

#[derive(Default)]
pub struct App {
    pub file_path: Option<String>,
    pub binary_data: Option<Vec<u8>>,
    pub format: Option<Format>,
    pub pe: Option<PE>,
    pub selected_view: SidebarItem,
    pub strings: Vec<(usize, String)>,
    pub string_filter: String,
    pub pattern_input: String,
    pub pattern_results: Vec<usize>,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut visuals = egui::Visuals::dark();
        let bg_color = egui::Color32::from_rgb(30, 30, 30);
        let panel_bg = egui::Color32::from_rgb(37, 37, 38);
        let text_color = egui::Color32::from_rgb(204, 204, 204);
        let accent_color = egui::Color32::from_rgb(14, 99, 156);
        
        visuals.override_text_color = Some(text_color);
        visuals.panel_fill = panel_bg;
        visuals.window_fill = bg_color;
        visuals.widgets.noninteractive.bg_fill = bg_color;
        visuals.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0, text_color);
        visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(45, 45, 45);
        visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(60, 60, 60);
        visuals.widgets.active.bg_fill = accent_color;
        visuals.selection.bg_fill = accent_color;
        ctx.set_visuals(visuals);

        let mut style = (*ctx.style()).clone();
        style.spacing.item_spacing = egui::vec2(8.0, 8.0);
        style.spacing.button_padding = egui::vec2(8.0, 6.0);
        style.visuals.widgets.noninteractive.corner_radius = egui::CornerRadius::same(4);
        style.visuals.widgets.inactive.corner_radius = egui::CornerRadius::same(4);
        style.visuals.widgets.hovered.corner_radius = egui::CornerRadius::same(4);
        style.visuals.widgets.active.corner_radius = egui::CornerRadius::same(4);
        ctx.set_style(style);

        super::show(self, ctx);
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Required by older versions or custom traits depending on eframe version
    }
}