#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use doice_lib::activities::*;

fn gui_fullscreen_main() {
    // Set options
    let options = eframe::NativeOptions {
        min_window_size: Some(egui::vec2(320.0, 100.0)),
        transparent: true,
        decorated: false,
        maximized: true,
        ..Default::default()
    };

    // Run app
    eframe::run_native(
        "Doice OS",
        options,
        Box::new(|cc| {
            let mut app = Box::new(DoiceApp::new(cc));
            app.register_activity::<LegacyDiceRoller>();
            app.register_activity::<StarfuryYeeter>();
            app.register_activity::<DiceRoller>();
            app.register_activity::<GlobalAnalyzer>();
            app.register_activity::<SpellBrowser>();
            app.register_activity::<DiceRollPresets>();
            app.register_activity::<Notes>();
            app.register_activity::<CharacterManager>();
            app
        }),
    );
}

fn gui_analyzer_only() {
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

fn gui_main() {
    if std::env::args().any(|s| s.contains("big")) {
        gui_fullscreen_main();
    } else {
        gui_analyzer_only();
    }
}

fn main() {
    gui_main();
}
