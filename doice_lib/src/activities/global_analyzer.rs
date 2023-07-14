use egui::{Key, Modifiers};

use doice_gui::{Activity, DCtx};

#[derive(Clone, Default)]
pub struct GlobalAnalyzer {
    text_in: String,
    prev_input: String,
}

impl Activity for GlobalAnalyzer {
    fn update(&mut self, ui: &mut egui::Ui, ctx: &mut DCtx) {
        let mut plotter = ctx.data().dice_grapher.write().unwrap();

        // Runs experiment if user holds down CTRL+SHIFT
        let do_run = ui
            .input(|i| i.modifiers)
            .matches(Modifiers::CTRL | Modifiers::SHIFT);
        plotter.run_experiment(do_run);

        let field = ui
            .vertical_centered_justified(|ui| ui.text_edit_singleline(&mut self.text_in))
            .inner;

        //Reinterpret roll on change
        if field.changed() && self.text_in != self.prev_input {
            plotter.display_roll(self.text_in.as_str());
        }
        self.prev_input = self.text_in.clone();

        // Roll on confirm
        if field.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter)) {
            plotter.roll();
            field.request_focus();
        }

        // Focus dice field on press of Home
        if !field.has_focus() && ui.input(|i| i.key_pressed(Key::Home)) {
            field.request_focus();
        }

        // Plotting
        plotter.show(ui);
    }

    fn name(&self) -> &'static str {
        "Dice Roller"
    }

    fn focus(&mut self) {}

    fn lose_focus(&mut self) {}

    fn resize(&self) -> bool {
        true
    }

    fn category(&self) -> &str {
        "Utility"
    }
}
