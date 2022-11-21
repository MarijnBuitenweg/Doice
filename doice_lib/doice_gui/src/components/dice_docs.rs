use doice_roller::FUNCTION_DOCS;
use eframe::egui::Ui;

/// Handles the ui for the function docs in the diceroller
pub fn dice_docs(ui: &mut Ui) {
    ui.group(|ui| {
        // Fill available space
        ui.set_height(ui.available_width());
        ui.set_width(ui.available_width());
        // Add all function docs, in whatever order they're defined
        for (name, doc) in FUNCTION_DOCS.iter() {
            ui.collapsing(*name, |ui| ui.label(*doc));
        }
    });
}
