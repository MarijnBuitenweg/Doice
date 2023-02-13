use doice_gui::{Activity, DCtx};

#[derive(Default, Clone)]
pub struct StarfuryYeeter {
    focus: bool,
}

impl Activity for StarfuryYeeter {
    fn update(&mut self, ui: &mut egui::Ui, _ctx: &mut DCtx) {
        ui.heading("This will be implemented later!");
    }

    fn name(&self) -> &'static str {
        "Starfury Bombardment"
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
        "Special"
    }
}
