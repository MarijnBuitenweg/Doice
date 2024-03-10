/// Contains all activities
pub mod activities;
/// A rudimentary application window that can run an activity
#[cfg(feature = "eframe")]
pub mod activity_host;
/// Contains UI setup tailored for ease of use.
/// It is here instead of in doice_gui, because it is not component agnostic like the other ui layouts (and as such depends on the specific ui components).
mod tailored_frames;

use doice_gui::eframe::egui::Context;
use doice_gui::eframe::emath::{Pos2, Vec2};
use doice_gui::eframe::epaint::Color32;
use doice_gui::eframe::Frame;
use egui::panel::TopBottomSide;
use egui::{Id, Layout, Sense, ViewportCommand};
pub use tailored_frames::TailoredUI;

pub fn draw_topbar(ctx: &Context, frame: &mut Frame) {
    egui::TopBottomPanel::new(TopBottomSide::Top, "MAINBAR").show(ctx, |ui| {
        let mainbar = ui.interact(
            ui.clip_rect(),
            Id::from("MAINBARRECT"),
            Sense::click_and_drag(),
        );
        if mainbar.drag_started() {
            ctx.send_viewport_cmd(ViewportCommand::StartDrag);
        }
        if mainbar.double_clicked() {
            ctx.send_viewport_cmd(ViewportCommand::OuterPosition(Pos2::new(0.0, 0.0)));
            let monitor = ctx.input(|i| i.viewport().monitor_size.unwrap());
            ctx.send_viewport_cmd(ViewportCommand::InnerSize(Vec2::from((
                monitor.x / 2.0,
                monitor.y,
            ))));
        }

        egui::menu::bar(ui, |ui| {
            ui.menu_button("Doice.", |ui| ui.small_button("Helo"));

            ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                let minimize_button = egui::widgets::Button::new("—").fill(Color32::DARK_GRAY);
                let exit_button = egui::widgets::Button::new("Exit").fill(Color32::DARK_RED);

                if ui.add(exit_button).clicked() {
                    ctx.send_viewport_cmd(ViewportCommand::Close);
                }
                if ui.add(minimize_button).clicked() {
                    ctx.send_viewport_cmd(ViewportCommand::Minimized(true));
                }
            })
        })
    });
}
