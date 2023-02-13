use egui::{ScrollArea, TextEdit};

use doice_gui::{Activity, DCtx};

#[derive(Clone, Default)]
pub struct Notes {
    focus: bool,
    text: String,
}

impl Activity for Notes {
    fn update(&mut self, ui: &mut egui::Ui, _ctx: &mut DCtx) {
        ScrollArea::vertical().show(ui, |ui| {
            let edit = ui.add(
                TextEdit::multiline(&mut self.text)
                    .desired_rows(10)
                    .lock_focus(true)
                    .desired_width(f32::INFINITY)
            );

            if self.focus && !edit.has_focus() {
                edit.request_focus();
            } 
            if edit.has_focus() && !self.focus {
                println!("dropping focus");
                edit.surrender_focus();
            }
        });
    }

    fn name(&self) -> &str {
        "notes"
    }

    fn focus(&mut self) {
        self.focus = true;
    }

    fn lose_focus(&mut self) {
        self.focus = false;
    }

    fn resize(&self) -> bool {
        true
    }

    fn category(&self) -> &str {
        "character"
    }
}