use doice_gui::{Activity, DCtx, DoiceShow};

#[derive(Default, Clone)]
pub struct CharacterManager {
    focus: bool,
}

impl Activity for CharacterManager {
    fn update(&mut self, ui: &mut egui::Ui, ctx: &mut DCtx) {
        let mut character = ctx.data().character.write().unwrap();
        character.show(ui, ctx);
    }

    fn name(&self) -> &str {
        "Character manager"
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
