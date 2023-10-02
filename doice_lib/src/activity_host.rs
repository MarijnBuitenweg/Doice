use std::panic::{AssertUnwindSafe, catch_unwind, UnwindSafe};
use std::rc::Rc;
use std::sync::Arc;
use std::task::Context;
use dyn_clone::DynClone;
use doice_gui::{Activity, AppData, DCtx, eframe};

use eframe::egui;
use egui::{Frame, Key};
use instant::Duration;
use crate::draw_topbar;

trait CloneActivity: Activity + DynClone {}
impl<T: Activity + DynClone> CloneActivity for T{}

/// Back again
pub struct ActivityHost {
    act: Option<Box<dyn CloneActivity>>,
    backup_act: Box<dyn CloneActivity>,
    data: Rc<AppData>,
    focused: bool,
    errored_out: bool,
}

impl ActivityHost {
    // Create a new activityhost hosting the provided activity
    pub fn new<T: CloneActivity + Clone + Default + 'static>(cc: &eframe::CreationContext<'_>) -> Self {
        let mut act: Box<(dyn CloneActivity + 'static)> = Box::<T>::default();
        let backup_act = dyn_clone::clone_box(&*act);
        let ctx = cc.egui_ctx.clone();
        act.init(ctx);
        ActivityHost {
            act: Some(act),
            backup_act,
            data: Rc::new(AppData::new(cc)),
            focused: false,
            errored_out: false,
        }
    }

    fn context(&self) -> DCtx {
        DCtx::new(Some(0), false, 0, Rc::clone(&self.data))
    }
    fn update_logic(&mut self,  ctx: &egui::Context, frame: &mut eframe::Frame) {
        if !self.focused {
            self.act.as_mut().unwrap().focus();
            self.focused = true;
        }
        let mut act = self.act.take().unwrap();
        let mut dctx = self.context();
        let nu_ctx = ctx.clone();
        if cfg!(not(debug_assertions)) {
            match catch_unwind(AssertUnwindSafe(|| {
                egui::CentralPanel::default().show(&nu_ctx, |ui| {
                    act.update(ui, &mut dctx);
                });
                act
            })) {
                Ok(act) => { self.act = Some(act); }
                Err(_) => { self.errored_out = true; }
            }
        } else {
            egui::CentralPanel::default().show(&nu_ctx, |ui| {
                act.update(ui, &mut dctx);
            });
            self.act = Some(act);
        }
    }

    fn show_error_screen(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.heading("Something went wrong.\nPlease press F to restart.");
                if ui.input(|i| i.key_pressed(Key::F)) {
                    self.act = Some(dyn_clone::clone_box(&*self.backup_act));
                    self.errored_out = false;
                }
            })
        });
    }
}

impl eframe::App for ActivityHost {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::dark());
        draw_topbar(ctx, frame);

        if self.errored_out {
            self.show_error_screen(ctx, frame);
            return;
        }

        self.update_logic(ctx, frame);
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}

    fn on_exit(&mut self, _gl: std::option::Option<&eframe::glow::Context>) {}

    fn auto_save_interval(&self) -> Duration {
        Duration::from_secs(30)
    }

    fn max_size_points(&self) -> egui::Vec2 {
        egui::Vec2::INFINITY
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        // NOTE: a bright gray makes the shadows of the windows look weird.
        // We use a bit of transparency so that if the user switches on the
        // `transparent()` option they get immediate results.
        egui::Color32::from_rgba_unmultiplied(12, 12, 12, 180).to_normalized_gamma_f32()

        // _visuals.window_fill() would also be a natural choice
    }

    fn persist_native_window(&self) -> bool {
        true
    }

    fn persist_egui_memory(&self) -> bool {
        true
    }

    fn warm_up_enabled(&self) -> bool {
        false
    }
}
