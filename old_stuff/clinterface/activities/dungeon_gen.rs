use crate::clinterface::core::AppData;

use console::Term;
use dialoguer::{Input, Select, Confirm};
use rand::Rng;

//(connec)S(tower)(R#(roomcount), Hall)

pub struct DungeonData {
    room_types: Vec<String>,
    connec_types: Vec<Connec>,
    tower: usize
}

impl DungeonData {
    pub fn new() -> DungeonData {
        DungeonData {
            room_types: Vec::new(),
            connec_types: Vec::new(),
            tower: 0,
        }
    }
}

#[derive(Clone)]
enum Connec {
    Ext(String),
    Int(String),
}

pub fn gen_dung(_: isize, dat: &mut AppData, term: &mut Term) -> Result<isize, &'static str> {
    if dat.dungeon_dat.room_types.len() == 0 {
        loop {
            let toipe: String = Input::new()
                .with_prompt("Enter a room type (leave empty to finisch specifying types)")
                .default(String::from(""))
                .interact()
                .or(Err("Could not read input."))?;

            if toipe.trim().len() == 0 {
                break;
            } else {
                dat.dungeon_dat.room_types.push(toipe);
            }
        }
    }
    if dat.dungeon_dat.connec_types.len() == 0 {
        loop {
            let connec_name: String = Input::new()
                .with_prompt("Enter name of a connection type (bridge, corridor, whatever...)")
                .default(String::from(""))
                .interact()
                .or(Err("Could not read input."))?;

            if connec_name.trim().len() == 0 {
                break;
            } 

            if Confirm::new()
                .with_prompt("Is it going to another tower?")
                .interact()
                .or(Err("Could not obtain selection."))? {
                    dat.dungeon_dat.connec_types.push(Connec::Ext(connec_name));
            } else {
                    dat.dungeon_dat.connec_types.push(Connec::Int(connec_name));
            }
        }
    }

    if dat.dungeon_dat.tower == 0 {
        dat.dungeon_dat.tower = Input::new()
            .with_prompt("What tower to start in?")
            .validate_with(|s: &String| -> Result<(), &str> {
                if s.trim().chars().all(|c| c.is_numeric()) {
                    Ok(())
                } else {
                    Err("Invalid number.")
                }
            }).interact()
            .or(Err("Could not read input."))?
            .parse()
            .or(Err("Could not parse number."))?;
    }

    loop {
        let opts = ["generate", "reset"];
        let chosen = Select::new()
            .with_prompt("what to do?")
            .items(&opts)
            .interact()
            .or(Err("Could not select option."))?;
        term.clear_screen().unwrap();

        if chosen == 0 {
            let connec = &dat.dungeon_dat.connec_types[dat.rng.gen_range(0..dat.dungeon_dat.connec_types.len())];
            let target;

            match connec {
                 Connec::Ext(s) => {
                     let mut new_tower = dat.rng.gen_range(1..5);
                     if new_tower >= dat.dungeon_dat.tower {
                         new_tower += 1;
                     }
                     dat.dungeon_dat.tower = new_tower;
                     target = s;
                     println!("Moving to tower {} via {}.", new_tower, target);
                 },
                 Connec::Int(s) => {
                     target = s;
                     println!("Moving to new room via {}.", target);
                 }
            }
            let room_type = &dat.dungeon_dat.room_types[dat.rng.gen_range(0..dat.dungeon_dat.room_types.len())];
            println!("Arriving in room of type: {}.", room_type);
        } else if chosen == 1 {
            dat.dungeon_dat.room_types.clear();
            dat.dungeon_dat.connec_types.clear();
            dat.dungeon_dat.tower = 0;
            return Ok(-5);
        }
    }

    Ok(0)
}