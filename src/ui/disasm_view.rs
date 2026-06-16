use eframe::egui;
use crate::ui::app::App;

pub fn show(_app: &mut App, ui: &mut egui::Ui) {
    ui.add_space(8.0);
    ui.heading("⊟ Disassembly");
    ui.add_space(16.0);

    ui.label(egui::RichText::new("Disassembly view is not yet implemented.").weak().italics());
    ui.add_space(8.0);
    ui.label(
        "This view will use the Capstone disassembler engine to provide\n\
         assembly code analysis from the binary's entry point.",
    );
}
