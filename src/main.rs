#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use clap::Parser;

#[derive(Parser)]
#[command(name = "Doice")]
#[command(version)]
#[command(about = "Rolls nice dice. Once, twice, or thrice")]
#[command(long_about = None)]
struct Cli {}

fn main() {
    let _cli = Cli::parse();
}
