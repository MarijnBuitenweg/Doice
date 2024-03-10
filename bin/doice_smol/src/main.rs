#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::egui::IconData;
use clap::Parser;
use doice_lib::activities::*;
use doice_lib::activity_host::ActivityHost;

const LOGO: &[u8] = include_bytes!("../../../design/Logo2.png");

#[derive(Parser)]
#[command(name = "Doice")]
#[command(version)]
#[command(about = "Rolls nice dice. Once, twice, or thrice")]
#[command(long_about = None)]
struct Cli {}

fn main() {
    let _cli = Cli::parse();

    // Set options
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_min_inner_size((320.0, 400.0))
            .with_icon(IconData {
                rgba: LOGO.to_vec(),
                width: 256,
                height: 256,
            })
            .with_transparent(true)
            .with_decorations(false)
            .with_fullscreen(false)
            .with_inner_size((420.0, 516.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Doice Analyzer",
        options,
        Box::new(|cc| Box::new(ActivityHost::new::<DiceRoller>(cc))),
    )
    .expect("Application errored out.");
}
