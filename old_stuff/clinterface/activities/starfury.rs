use console::{Key, StyledObject, Term, style};
use dialoguer::Input;
use rand::{Rng, prelude::ThreadRng};

use crate::clinterface::core::AppData;

const POSITIONS: [[usize; 2]; 5] = [
    [1, 2], //Top-left
    [4, 2], //Top-right
    [1, 4], //Bottom-left
    [4, 4], //Bottom-right
    [0, 6], //Back
];

const LINES: [&'static str; 6] = ["-------", "|  |  |", "|><|  |", "|  |><|", " Back", ">Back"];
const SQUARE_SIZE: usize = 10;
const AREA_SIZE: usize = SQUARE_SIZE * SQUARE_SIZE;

//This could probably be improved a lot, for example by using a buffered terminal
pub fn starfury_bombard(_: isize, dat: &mut AppData, term: &mut Term) -> Result<isize, &'static str> {
    if let Err(_) = term.hide_cursor() {
        return Err("Error: Could not hide cursor");
    }

    println!("Select on which of the center squares you will be standing:");
    println!("{}", LINES[0]);
    println!("{}", LINES[2]);
    println!("{}", LINES[0]);
    println!("{}", LINES[1]);
    println!("{}", LINES[0]);
    println!("{}", LINES[4]);
    let mut pos = 0;
    let mut line_type = 2;
    
    if let Err(_) = term.move_cursor_to(POSITIONS[pos][0], POSITIONS[pos][1]) {
        return Err("Error: Cursor could not be moved.");
    }

    loop {
        match term.read_key() {
            Ok(key) => {
                match key {
                    Key::ArrowLeft | Key::Char('a') => {
                        if pos & 1 == 1 {
                            pos -= 1;
                        }
                    },
                    Key::ArrowUp | Key::Char('w') => {
                        if pos > 1 {
                            pos -= 2;
                        }
                    },
                    Key::ArrowRight | Key::Char('d') => {
                        if pos & 1 == 0 && pos < 4{
                            pos += 1;
                        }
                    },
                    Key::ArrowDown | Key::Char('s') => {
                        if pos < 3 {
                            pos += 2;
                        } else if pos == 3 {
                            pos = 4;
                        }
                    },
                    Key::Enter => {
                        break;
                    },
                    _ => {}
                }
            }
            Err(_) => {
                return Err("Error: Keystrokes could not be registered.");
            }
        }
        
        //Reset the current line to its deselected form
        reset_line(line_type, term)?;
        //move the cursor to its new position
        move_cursor(pos, term)?;

        //Determine which kind of selected line form is appropriate
        line_type = match POSITIONS[pos][0] {
            0 => 5,
            1 => 2,
            4 => 3,
            _ => {
                return Err("Error: cursor position misalignment");
            }
        };

        //replace the unselected line with its new selected form
        replace_line(line_type, term)?;

        //move the cursor back
        move_cursor(pos, term)?;
    }

    //If user wants to go back to the menu, send the user back to the menu
    if pos == 4 {
        return Ok(3);
    }
    
    term.clear_screen().unwrap();
    
    //Obtain desired number of meteors
    let mut num: usize = 0;
    if let Err(_) = Input::new()
        .with_prompt("Type the number of meteors you wish to be yote")
        .validate_with(|s: &String| -> Result<(), &str> {
            num = match s.parse() {
                Err(_) | Ok(0) => {
                    return Err("Please enter a valid positive non-zero integer.");
                }
                Ok(val) => val,
            };
            Ok(())
        })
        .interact_text() {
        return Err("Error: Input could not be obtained.");
    }

    //Expect to see more '.unwrap()' from now on, I'm getting lazy
    term.clear_screen().unwrap();

    //Yeet meteors
    yeeteors(pos, num, &mut dat.rng, term)?;

    Ok(3)
}

fn reset_line(line_type: usize, term: &mut Term) -> Result<(), &'static str> {
    let new_line = match line_type {
        2 | 3   => 1,
        5       => 4,
        _       => {
            return Err("Error: Trying to reset a line that cannot be reset.");
        }
    };

    replace_line(new_line, term)
}

fn replace_line(new_line: usize, term: &mut Term) -> Result<(), &'static str> {
    if let Err(_) = term.clear_line() {
        return Err("Error: Could not clear line.");
    }

    if let Err(_) = term.write_line(LINES[new_line]) {
        return Err("Error: Could not write line.");
    }

    Ok(())
}

fn move_cursor(pos: usize, term: &mut Term) -> Result<(), &'static str> {
    if let Err(_) = term.move_cursor_to(POSITIONS[pos][0], POSITIONS[pos][1]) {
        return Err("Error: Cursor could not be moved.");
    }
    Ok(())
}

fn yeeteors(pos: usize, num: usize, rng: &mut ThreadRng, term: &mut Term) -> Result<(), &'static str> {
    if SQUARE_SIZE & 1 == 1 {
        return Err("Error: Invalid AREA_SIZE (it should be even).");
    }

    println!("Below is a map of the affected area, the numbers show how many meteors hit each tile, the tile with the blue background is the tile occupied by the user (if the user has size 'medium').");

    if !term.features().colors_supported() {
        println!("Please note that color is not supported in this terminal, which means that the location of the user cannot be displayed");
    }

    let halfway = SQUARE_SIZE/2 as usize;
    let usr_pos = [halfway - 1 + (pos & 1), halfway - 1 + ((pos & 2) >> 1)];//[halfway - (pos & 1), halfway - 1 + (pos & 2)];
    let usr_i = SQUARE_SIZE*usr_pos[1] + usr_pos[0];

    //Calculates hits for every square in range
    let mut map = [0; AREA_SIZE];
    for _ in 0..num {
        let hit = rng.gen_range(0..AREA_SIZE);
        map[hit] += 1;
    }

    //Converts hits into styled text
    let mut num_text: Vec<StyledObject<String>> = map.iter().map(|hits| {
        let s = style(format!("{:02}", hits)).bold();
        
        //If hit, turn text red
        if *hits == 0 {
            s
        } else {
            s.red()
        }
    }).collect();

    num_text[usr_i] = num_text[usr_i].clone().bold().on_blue();
    let mut listing: String = String::with_capacity(num * 10);

    //print map
    for i in 0..SQUARE_SIZE {
        //print line
        print!("-");
        for _ in 0..SQUARE_SIZE {
            print!("---");
        }
        print!("\n");

        //print line with numbers
        print!("|");
        for j in 0..SQUARE_SIZE {
            //if [j, i] == usr_pos {
            //    print!("{}|", num_text[SQUARE_SIZE*i + j].clone().bold().on_blue());
            //} else {
                print!("{}|", num_text[SQUARE_SIZE*i + j]);
                
                //If there's a hit, write to listing
                if map[SQUARE_SIZE*i + j] != 0 {
                    listing.push_str(
                        &format!(
                            "{} hits at\t{}\t{}\n", 
                            map[SQUARE_SIZE*i + j], 
                            j as isize - usr_pos[0] as isize, 
                            usr_pos[1] as isize - i as isize
                        )
                    );
                }
            //}
        }
        print!("\n");
    }
    //print last line
    print!("-");
    for _ in 0..SQUARE_SIZE {
        print!("---");
    }
    print!("\n");

    println!("Listing with relative position for each hit (horizontal, vertical):");
    println!("{}", listing);

    println!("Press Enter to continue...");
    loop {
        if let Key::Enter = term.read_key().unwrap() {
            break;
        }
    }

    Ok(())
}