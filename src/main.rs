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

fn run_new() {
    let term = Term::stdout();
    let opts = vec!["Meep", "Moop"];
    let chosen = Select::new()
        .items(&opts)
        .interact()
        .unwrap();
    term.write_line(&chosen.to_string()).unwrap();
}

fn main() {
    run_new();
}
