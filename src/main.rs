//use rand::prelude::*;
//use std::io::stdin;
//use std::result::Result;
use dialoguer::Select;
use console::Term;

mod old;
mod dice_rolls;
mod clinterface;
mod dnd_character;
mod interpret;

fn bsc_test() {
    let term = Term::stdout();
    let opts = vec!["Meep", "Moop"];
    let chosen = Select::new()
        .items(&opts)
        .interact()
        .unwrap();
    term.write_line(&chosen.to_string()).unwrap();
}

use clinterface::core::UI;
fn main() {
    let mut ui = UI::new();
    ui.run().unwrap();
}
