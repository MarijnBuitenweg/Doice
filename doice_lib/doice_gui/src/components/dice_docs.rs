use doice_roller::FUNCTION_DOCS;
use eframe::egui::Ui;

pub fn dice_docs(ui: &mut Ui) {
    for (name, doc) in FUNCTION_DOCS.iter() {
        ui.collapsing(*name, |ui| ui.label(*doc));
    }
}
