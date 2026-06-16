mod formats;
mod tools;
mod ui;

use eframe::egui;
use crate::ui::app::App;

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_maximized(true),
        ..Default::default()
    };

    eframe::run_native(
        "Decompiler",
        options,
        Box::new(|_| Ok(Box::new(App::default()))),
    ).unwrap();
}