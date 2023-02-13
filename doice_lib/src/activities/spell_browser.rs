use doice_gui::DCtx;

use doice_gui::{
    Activity,
};
use doice_utils::ParSearch;

#[derive(Clone, Default)]
pub struct SpellBrowser {
    focus: bool,
    s_txt: String,
    displ_spells: Vec<(u32, String)>,
}

impl Activity for SpellBrowser {
    fn update(&mut self, ui: &mut egui::Ui, ctx: &mut DCtx) {
        let search_bar = ui.text_edit_singleline(&mut self.s_txt);
        let spells = &ctx.data().dnd_data.spells;
        if search_bar.changed() {
            self.displ_spells = spells
                .par_find_closest_matches(&self.s_txt, 10)
                .iter()
                .map(|(score, spell)| (*score, spell.name.clone()))
                .collect();
        }

        if self.focus {
            search_bar.request_focus();
        } else if search_bar.has_focus() {
            search_bar.surrender_focus();
        }

        for (score, spell) in self.displ_spells.iter() {
            ui.label(format!("{}\t{}", score, spell));
        }
    }

    fn name(&self) -> &str {
        "Spell Browser"
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
        "content"
    }
}
