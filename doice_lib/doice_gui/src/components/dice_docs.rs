use doice_roller::FUNCTION_DOCS;
use eframe::egui::Ui;

pub fn dice_docs(ui: &mut Ui) {
    ui.group(|ui| {
        ui.set_height(ui.available_width());
        ui.set_width(ui.available_width());
        for (name, doc) in FUNCTION_DOCS.iter() {
            ui.collapsing(*name, |ui| ui.label(*doc));
        }
    });
}
