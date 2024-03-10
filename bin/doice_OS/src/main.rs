#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use doice_lib::{
    activities::{egui::IconData, *},
    TailoredUI,
};

use clap::Parser;
use doice_lib::activity_host::ActivityHost;

const LOGO: &[u8] = include_bytes!("../../../design/Logo.png");

#[derive(Parser)]
#[command(name = "Doice")]
#[command(version)]
#[command(about = "Rolls nice dice. Once, twice, or thrice")]
#[command(long_about = None)]
struct Cli {
    #[arg(short, long)]
    big: bool,
}

fn gui_fullscreen_main() {
    // Set options
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_min_inner_size((320.0, 100.0))
            .with_icon(IconData {
                rgba: LOGO.to_vec(),
                width: 256,
                height: 256,
            })
            .with_transparent(true)
            .with_decorations(false)
            .with_fullscreen(false),
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
        viewport: egui::ViewportBuilder::default()
            .with_min_inner_size((320.0, 100.0))
            .with_inner_size((600.0, 670.0))
            .with_icon(IconData {
                rgba: LOGO.to_vec(),
                width: 256,
                height: 256,
            })
            .with_transparent(false)
            .with_decorations(true)
            .with_resizable(false)
            .with_fullscreen(false),
        ..Default::default()
    };

    eframe::run_native(
        "Doice Analyzer",
        options,
        Box::new(|cc| Box::new(ActivityHost::new::<DiceRoller>(cc))),
    );
}

fn gui_main() {
    let cli = Cli::parse();
    if cli.big {
        gui_fullscreen_main();
    } else {
        gui_analyzer_only();
    }
}

fn tailor_test_main() -> eframe::Result<()> {
    // Set options
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_min_inner_size((320.0, 100.0))
            .with_icon(IconData {
                rgba: LOGO.to_vec(),
                width: 256,
                height: 256,
            })
            .with_transparent(true)
            .with_decorations(false)
            .with_fullscreen(false),
        ..Default::default()
    };

    // Run app
    eframe::run_native(
        "Doice.",
        options,
        Box::new(|cc| Box::new(TailoredUI::new(cc))),
    )
}

fn main() -> eframe::Result<()> {
    tailor_test_main()
    //gui_main();
}
