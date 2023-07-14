use egui::{Context, Key, Modifiers};

use doice_gui::{components::DiceGrapher, Activity, DCtx};

#[derive(Clone, Default)]
pub struct DiceRoller {
    text_in: String,
    prev_input: String,
    plotter: DiceGrapher,
}

impl Activity for DiceRoller {
    fn init(&mut self, e_ctx: Context) {
        self.plotter = DiceGrapher::new(e_ctx);
    }

    fn update(&mut self, ui: &mut egui::Ui, _ctx: &mut DCtx) {
        // Runs experiment if user holds down CTRL+SHIFT
        let do_run = ui
            .input(|i| i.modifiers)
            .matches(Modifiers::CTRL | Modifiers::SHIFT);
        self.plotter.run_experiment(do_run);

        let field = ui
            .vertical_centered_justified(|ui| ui.text_edit_singleline(&mut self.text_in))
            .inner;

        //Reinterpret roll on change
        if field.changed() && self.text_in != self.prev_input {
            self.plotter.display_roll(self.text_in.as_str());
        }
        self.prev_input = self.text_in.clone();

        // Roll on confirm
        if field.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter)) {
            self.plotter.roll();
            field.request_focus();
        }

        // Focus dice field on press of Home
        if !field.has_focus() && ui.input(|i| i.key_pressed(Key::Home)) {
            field.request_focus();
        }

        // Plotting
        self.plotter.show(ui);
    }

    fn name(&self) -> &'static str {
        "Local Dice Roller"
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

// fn _make_plot(ui: &mut Ui, roller: &Roll) {
//     let dist = roller.dist();

//     if dist.is_empty() {
//         return;
//     }

//     let bars = dist
//         .iter()
//         .map(|(outcome, prob)| Bar::new(*outcome as f64, *prob))
//         .collect();
//     let chart = BarChart::new(bars)
//         .color(Color32::LIGHT_BLUE)
//         .name("Probability Distribution");

//     let width = dist
//         .keys()
//         .max()
//         .unwrap()
//         .abs_diff(*dist.keys().min().unwrap());
//     let mut height = 0.0f64;
//     for val in dist.values() {
//         if *val > height {
//             height = *val;
//         }
//     }

//     Plot::new("Roll Analyzer")
//         .data_aspect(width as f32 / height as f32)
//         .view_aspect(1.0)
//         .show(ui, |ui| ui.bar_chart(chart));
// }
