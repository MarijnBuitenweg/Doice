/// Contains all activities
pub mod activities;
/// Contains UI setup tailored for ease of use.
/// It is here instead of in doice_gui, because it is not component agnostic like the other ui layouts (and as such depends on the specific ui components).
mod tailored_frames;
/// A rudimentary application window that can run an activity
#[cfg(feature = "eframe")]
pub mod activity_host;

use egui::{Id, Layout, Sense};
use egui::panel::TopBottomSide;
use doice_gui::eframe::emath::{Pos2, Vec2};
use doice_gui::eframe::epaint::Color32;
use doice_gui::eframe::Frame;
use doice_gui::eframe::egui::Context;
pub use tailored_frames::TailoredUI;

pub fn draw_topbar(ctx: &Context, frame: &mut Frame) {
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
            ui.menu_button("Doice.", |ui| ui.small_button("Helo"));

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
}
