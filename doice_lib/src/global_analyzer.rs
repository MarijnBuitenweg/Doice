use egui::{Key, Modifiers};

use doice_gui::{Activity, DCtx};

#[derive(Clone, Default)]
pub struct GlobalAnalyzer {
    focus: bool,
    text_in: String,
    prev_input: String,
}

impl Activity for GlobalAnalyzer {
    fn update(&mut self, ui: &mut egui::Ui, ctx: &mut DCtx) {
        let mut plotter = ctx.data().dice_grapher.write().unwrap();

        // Runs experiment if user holds down CTRL+SHIFT
        let do_run = ui
            .input()
            .modifiers
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
        if field.lost_focus() && ui.input().key_pressed(Key::Enter) {
            plotter.roll();
            field.request_focus();
        }

        if self.focus {
            field.request_focus();
        }

        if field.has_focus() && !self.focus {
            field.surrender_focus();
        }

        // Plotting
        plotter.show(ui);
    }

    fn name(&self) -> &'static str {
        "Dice Roller"
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
        "Utility"
    }
}
