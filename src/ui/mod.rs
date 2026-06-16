pub mod file;
pub mod app;
pub mod viewer;
pub mod overview;
pub mod hex_editor;
pub mod disasm_view;
pub mod patterns_view;
pub mod strings_view;
pub mod headers;
pub mod sections;
pub mod imports;
pub mod exports;

use eframe::egui;
use crate::ui::app::App;

// here the app decides what is gonna show
pub fn show(app: &mut App, ctx: &egui::Context) {
    if app.binary_data.is_none() {
        file::show(app, ctx);
    }
    else {
        viewer::show(app, ctx);
    }
}