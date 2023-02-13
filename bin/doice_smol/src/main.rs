#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use doice_lib::activities::*;

fn main() {
    // Set options
    let options = eframe::NativeOptions {
        min_window_size: Some(egui::vec2(320.0, 100.0)),
        transparent: false,
        decorated: true,
        maximized: false,
        resizable: false,
        initial_window_size: Some(egui::vec2(600.0, 670.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Doice Analyzer",
        options,
        Box::new(|cc| Box::new(ActivityHost::new::<DiceRoller>(cc))),
    );
}
