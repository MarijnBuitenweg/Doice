use std::rc::Rc;

use doice_gui::{
    eframe::{App, CreationContext},
    Activity, AppData, DCtx,
};
use egui::{panel::TopBottomSide, Color32, Frame, Layout, Ui, Vec2};
use egui_extras::StripBuilder;

use crate::activities::{CharacterManager, GlobalAnalyzer, Notes, WideAnalyzer};

pub struct TailoredUI {
    /// Global application data
    data: Rc<AppData>,
    // Components
    roller: WideAnalyzer,
    manager: CharacterManager,
    notes: Notes,
}

impl TailoredUI {
    pub fn new(cc: &CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());

        Self {
            data: Rc::new(AppData::new(cc)),
            roller: WideAnalyzer::default(),
            manager: CharacterManager::default(),
            notes: Notes::default(),
        }
    }

    fn context(&self, i: usize) -> DCtx {
        DCtx::new(Some(1), false, i, Rc::clone(&self.data))
    }
}

impl App for TailoredUI {
    fn update(&mut self, ctx: &egui::Context, frame: &mut doice_gui::eframe::Frame) {
        let mut dctx = self.context(0);
        egui::TopBottomPanel::new(TopBottomSide::Top, "MAINBAR").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Doice.", |ui| ui.small_button("Quartered"));

                ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                    let exit_button = egui::widgets::Button::new("Exit").fill(Color32::DARK_RED);
                    if ui.add(exit_button).clicked() {
                        frame.close()
                    }
                })
            })
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let size = ui.available_size();
            let group_margin = Frame::group(ui.style()).inner_margin.top * 2.0;
            let quarter: Vec2 = (size.x / 2.0 - group_margin, size.y / 2.0 - group_margin).into();
            let mut context = self.context(0);
            dbg!(group_margin);

            ui.horizontal_top(|ui| {
                // Quarter 1: Dice roller, grapher, and docs
                ui.group(|ui| {
                    ui.set_max_size(quarter);
                    ui.vertical(|ui| self.roller.update(ui, &mut context))
                });

                // Quarter 2: Dice history
                ui.group(|ui| {
                    ui.set_max_size(quarter);
                    ui.vertical(|ui| {
                        context
                            .data()
                            .dice_grapher
                            .write()
                            .unwrap()
                            .history_mut()
                            .show(ui)
                    })
                });
            });
        });
    }
}
