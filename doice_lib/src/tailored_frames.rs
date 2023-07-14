use std::rc::Rc;

use doice_gui::{
    eframe::{App, CreationContext},
    Activity, AppData, DCtx,
};
use egui::{panel::TopBottomSide, Color32, Frame, Id, Layout, Pos2, Sense, Ui, Vec2};
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
            let mainbar = ui.interact(
                ui.clip_rect(),
                Id::from("MAINBARRECT"),
                Sense::click_and_drag(),
            );
            if mainbar.drag_started() {
                frame.drag_window()
            }
            if mainbar.double_clicked() {
                frame.set_window_pos(Pos2::new(0.0, 0.0));
                let monitor = frame.info().window_info.monitor_size.unwrap();
                frame.set_window_size(Vec2::from((monitor.x / 2.0, monitor.y)));
            }

            egui::menu::bar(ui, |ui| {
                ui.menu_button("Doice.", |ui| ui.small_button("Quartered"));

                ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                    let minimize_button = egui::widgets::Button::new("—").fill(Color32::DARK_GRAY);
                    let exit_button = egui::widgets::Button::new("Exit").fill(Color32::DARK_RED);

                    if ui.add(exit_button).clicked() {
                        frame.close()
                    }
                    if ui.add(minimize_button).clicked() {
                        frame.set_minimized(true);
                    }
                })
            })
        });

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
                            .show(ui)
                    })
                })
            });
        });
    }
}
