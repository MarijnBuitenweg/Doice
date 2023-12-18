use std::rc::Rc;

use doice_gui::{
    eframe::{App, CreationContext},
    Activity, AppData, DCtx,
};
use egui::{Frame, Vec2};


use crate::activities::{CharacterManager, Notes, WideAnalyzer};
use crate::draw_topbar;

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
        let _dctx = self.context(0);
        draw_topbar(ctx, frame);

        egui::CentralPanel::default().show(ctx, |ui| {
            let size = ui.available_size();
            let group_margin = Frame::group(ui.style()).inner_margin.top * 2.0;
            let quarter: Vec2 = (size.x / 2.0 - group_margin, size.y / 2.0 - group_margin).into();
            let mut context = self.context(0);

            // Top half of the window
            ui.horizontal_top(|ui| {
                // Quarter 1: Dice roller, grapher, and docs
                ui.group(|ui| {
                    ui.set_max_size(quarter);
                    ui.set_min_size(quarter);
                    ui.vertical(|ui| self.roller.update(ui, &mut context))
                });

                // Quarter 2: Dice history
                ui.group(|ui| {
                    ui.set_max_size(quarter);
                    ui.set_min_size(quarter);
                    ui.vertical(|ui| {
                        context
                            .data()
                            .dice_grapher
                            .write()
                            .unwrap()
                            .history_mut()
                            .show_flex(ui)
                    })
                });
            });

            // Bottom half of the window
            ui.horizontal_top(|ui| {
                // Quarter 3: Initiative Tracker
                ui.group(|ui| {
                    ui.set_max_size(quarter);
                    ui.set_min_size(quarter);
                    ui.vertical(|ui| {
                        context
                            .data()
                            .dice_grapher
                            .write()
                            .unwrap()
                            .initiator_mut()
                            .show_flex(ui)
                    })
                });

                // Quarter 4: Statblock viewer
                ui.group(|ui| {
                    ui.set_max_size(quarter);
                    ui.set_min_size(quarter);
                    ui.vertical(|ui| {
                        self.manager.update(ui, &mut context);
                    })
                });
            });
        });
    }
}
