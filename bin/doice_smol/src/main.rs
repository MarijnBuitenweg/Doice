#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use clap::Parser;
use eframe::NativeOptions;
use doice_lib::activities::*;
use doice_lib::activities::eframe::IconData;
use doice_lib::activities::egui::vec2;
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
    let options = NativeOptions {
        min_window_size: Some(vec2(320.0, 400.0)),
        icon_data: Some(IconData::try_from_png_bytes(LOGO).unwrap()),
        transparent: true,
        decorated: false,
        fullscreen: false,
        initial_window_size: Some(vec2(420.0, 516.0)),

        ..Default::default()
    };

    eframe::run_native(
        "Doice Analyzer",
        options,
        Box::new(|cc| Box::new(ActivityHost::new::<DiceRoller>(cc))),
    ).expect("Application errored out.");
}
