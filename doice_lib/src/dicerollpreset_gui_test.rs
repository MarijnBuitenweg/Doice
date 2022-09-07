use doice_gui::{Activity, DCtx};

#[derive(Clone, Default)]
pub struct DiceRollPresets {
    ac_text: String,
    eb_crit: bool,
}

impl Activity for DiceRollPresets {
    fn update(&mut self, ui: &mut egui::Ui, ctx: &mut DCtx) {
        let mut dice_grapher = ctx.data().dice_grapher.write().unwrap();

        if ui.button("Eldritch blast attack roll").clicked() {
            dice_grapher.display_roll("d+15");
        }

        ui.horizontal(|ui| {
            ui.checkbox(&mut self.eb_crit, "crit");
            ui.add_space(100.0f32);
            if ui.button("Eldritch blast damage roll").clicked() {
                if self.eb_crit {
                    dice_grapher.display_roll("crit(1d10)+6");
                } else {
                    dice_grapher.display_roll("1d10+6");
                }
                dice_grapher.roll();
            }
        });

        ui.horizontal(|ui| {
            ui.text_edit_singleline(&mut self.ac_text);
            if ui.button("SET AC").clicked() {
                if let Ok(ac) = self.ac_text.parse::<isize>() {
                    dice_grapher.set_dc(Some(ac));
                }
            }
        });

        let ac_opt = self.ac_text.parse().ok();
        if ui.button("Roll FULL Eldritch blast").clicked() {
            if let Some(ac) = ac_opt {
                dice_grapher.display_roll("d+15");
                let atk_roll = dice_grapher.roll().value;
                if atk_roll >= ac && atk_roll != 16 {
                    dice_grapher.display_roll("1d10+6");
                } else {
                    dice_grapher.display_roll("0");
                }
                dice_grapher.roll();
            }
        }

        ui.label(ctx.data().character.read().unwrap().to_string());
    }

    fn name(&self) -> &str {
        "Testing Apparatus"
    }

    fn focus(&mut self) {}

    fn lose_focus(&mut self) {}

    fn resize(&self) -> bool {
        true
    }

    fn category(&self) -> &str {
        "demo"
    }
}
