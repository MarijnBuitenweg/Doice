use eframe::egui::Ui;

use crate::DCtx;

pub trait DoiceShow {
    fn show(&mut self, ui: &mut Ui, ctx: &DCtx);
}