use std::io;
use crate::dice_rolls;
use console::Term;
use rand;
use super::super::core::AppData;

pub fn dice_console(_: isize, _: &mut AppData, _: &mut Term) -> Result<isize, &'static str> {
    println!("Welcome to the dice console!\nDice format: (dice count)d(dice type)+(optional dice modifier) +(optional total modifier)");

    let mut buf = String::new();
    loop {
        buf.clear();

        //Print prompt
        println!("Enter a diceroll:");

        //Obtain user input
        if let Err(_) = io::stdin().read_line(&mut buf) {
            return Err("IO error: user input could not be read.");
        }

        //Checks if user wants to exit
        if buf.contains('q') {
            break;
        }


        let mut rng = rand::thread_rng();

        let roller = match dice_rolls::Roller::from_text(buf.as_str()) {
            Ok(val) => val,
            Err(s) => {
                println!("{}", s);
                continue;
            }
        };

        let out = roller.roll_text(&mut rng);

        println!("{}", out);
    }

    Ok(0)
}