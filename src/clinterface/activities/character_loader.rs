use console::Term;
use dialoguer::{Input};
use std::{fs};
use crate::clinterface::{
    core::AppData,
    msg::display_msg
};

//Data format for a character uses TOML headers (eg: [header]), uses ':' for value assignment, does need anything to delimit a string

pub fn character_loader(_: isize, _dat: &mut AppData, term: &mut Term) -> Result<isize, &'static str> {
    println!("The character loader currently does not do anything. Using 'q' as the filename will bring you back to the menu.");
    let mut _src = String::new();
    loop {
        //Obtain filename from user and handle errors
        let f_name = match Input::<String>::new()
            .with_prompt("Enter the filename of the file you want to load or drop the file onto the prompt")
            .interact() {
                Ok(s) => s,
                Err(_) => {
                    return Err("IO error: User input could not be read");
                }
        };

        if f_name == "q" || f_name == "Q" {
            return Ok(0);
        }
        
        _src = match fs::read_to_string(f_name) {
            Ok(s) => s,
            Err(_) => {
                display_msg("File could not be found", term);
                continue;
            }
        };
        break;
    }

    display_msg("File loaded successfully!", term);
    Ok(0)
}