use std::collections::HashMap;

use crate::{show_trait::DoiceShow, DCtx};
use dnd_data::{
    character::Character,
    character::{ProfLvl, Skills, Stats},
};
use doice_roller::{DiceRoller, Roll};
use eframe::{
    egui::{collapsing_header::CollapsingState, Grid, Id, Layout, Ui},
    emath::Align,
};
use itertools::Itertools;

impl DoiceShow for Character<'_> {
    fn show(&mut self, ui: &mut eframe::egui::Ui, ctx: &DCtx) {
        ui.heading(&self.name);
        show_resources(self, ui, ctx);
        show_stats(self, ui, ctx);
        show_skills(self, ui, ctx);
    }
}

fn show_resources(character: &mut Character, ui: &mut Ui, _ctx: &DCtx) {
    CollapsingState::load_with_default_open(ui.ctx(), Id::new("Resources collapsible"), false)
        .show_header(ui, |ui| {
            ui.label("Resources\t\tRest:");
            if ui.button("Long").clicked() {
                character.long_rest();
            }

            if ui.button("Short").clicked() {
                character.short_rest();
            }
        })
        .body(|ui| {
            let categories = character.state.iter().group_by(|elem| elem.category());

            for (cat_opt, items) in &categories {
                if let Some(cat) = cat_opt {
                    ui.collapsing(cat, |ui| {
                        for sv in items {
                            ui.label(sv.to_string());
                        }
                    });
                } else {
                    for sv in items {
                        ui.label(sv.to_string());
                    }
                }
            }
        });
}

fn show_stats(character: &mut Character, ui: &mut Ui, ctx: &DCtx) {
    ui.collapsing("Stats", |ui| {
        character.stats.show(ui, ctx);
    });
}

impl DoiceShow for Stats {
    fn show(&mut self, ui: &mut Ui, ctx: &DCtx) {
        Grid::new("stats")
            .num_columns(3)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                for (name, stat) in self.iter_mut() {
                    ui.label(&format!("{}:", name));

                    let stat = if *stat > 25 { *stat / 10 } else { *stat };

                    let bonus = (stat as isize - 10) / 2;

                    if stat < 10 {
                        ui.label(format!("{} ({})", stat, bonus));
                    } else {
                        ui.label(format!("{} (+{})", stat, bonus));
                    }

                    roll_buttons(ui, bonus, name, ctx);
                    ui.end_row();
                }
            });
    }
}

fn show_skills(character: &mut Character, ui: &mut Ui, ctx: &DCtx) {
    ui.collapsing("Skills", |ui| {
        show_skills_helper(
            &mut character.skills,
            ui,
            &character.stats,
            character.prof_bonus,
            ctx,
        );
    });
}

pub fn show_skills_helper(skills: &mut Skills, ui: &mut Ui, stats: &Stats, prof: u8, ctx: &DCtx) {
    Grid::new("skills")
        .num_columns(3)
        .spacing([40.0, 4.0])
        .striped(true)
        .show(ui, |ui| {
            let bonuses: HashMap<_, _> = stats
                .iter()
                .map(|(name, stat)| {
                    let stat = if *stat > 25 { *stat / 10 } else { *stat };

                    let bonus = (stat as isize - 10) / 2;
                    (name, bonus)
                })
                .collect();

            for (name, skill) in skills.iter_mut() {
                ui.label(&format!("{}:", name));
                let bonus = bonuses[&skill.stat_name] + (prof * skill.prof as u8) as isize;
                let mut centre_txt = if bonus < 0 {
                    bonus.to_string()
                } else {
                    format!("+{}", bonus)
                };

                if skill.prof != ProfLvl::Nothing {
                    centre_txt += " (";
                    centre_txt += &skill.prof.to_string();
                    centre_txt += ")";
                }

                ui.label(&centre_txt);

                roll_buttons(ui, bonus, name, ctx);
                ui.end_row();
            }
        });
}

fn roll_buttons(ui: &mut Ui, bonus: isize, name: &String, ctx: &DCtx) {
    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
        let mut opt_roll = None;
        if ui.button("adv").clicked() {
            opt_roll = Some(1);
        }

        if ui.button("check").clicked() {
            opt_roll = Some(0);
        }

        if ui.button("dis").clicked() {
            opt_roll = Some(-1);
        }

        if let Some(adv) = opt_roll {
            let mut roller = ctx.data().dice_grapher.write().unwrap();
            let mut txt = format!("{} check", name);
            if adv == 1 {
                txt += " with adv";
            } else if adv == -1 {
                txt += " with disadv";
            }
            roller.display_roll(
                Roll::from_expr(DiceRoller::new(20, 1, adv).into()).with_text(&txt) + bonus,
            );
            roller.roll();
        }
    });
}
