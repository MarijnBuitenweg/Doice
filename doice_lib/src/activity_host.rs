use std::rc::Rc;
use doice_gui::{Activity, AppData, DCtx, eframe};

use eframe::egui;
use instant::Duration;
use crate::draw_topbar;

/// Back again
pub struct ActivityHost {
    act: Box<dyn Activity>,
    data: Rc<AppData>,
    focused: bool,
}

impl ActivityHost {
    // Create a new activityhost hosting the provided activity
    pub fn new<T: Activity + Clone + Default + 'static>(cc: &eframe::CreationContext<'_>) -> Self {
        let mut act = Box::<T>::default();
        let ctx = cc.egui_ctx.clone();
        act.init(ctx);
        ActivityHost {
            act,
            data: Rc::new(AppData::new(cc)),
            focused: false,
        }
    }

    fn context(&self) -> DCtx {
        DCtx::new(Some(0), false, 0, Rc::clone(&self.data))
    }
}

impl eframe::App for ActivityHost {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if !self.focused {
            self.act.focus();
            self.focused = true;
        }
        ctx.set_visuals(egui::Visuals::dark());
        draw_topbar(ctx, frame);
        egui::CentralPanel::default().show(ctx, |ui| self.act.update(ui, &mut self.context()));
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