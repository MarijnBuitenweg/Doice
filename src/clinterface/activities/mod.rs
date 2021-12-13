use crate::dice_rolls::Roller;

//This file contains all the activities
use super::core::Activity;

mod dice_console;
mod character_loader;
mod starfury;
mod rollers;
pub mod rainworms;

pub const ACTIVITY_ARRAY: [Activity; 4] = [Activity {
    name: "Dice Console",
    controller: dice_console::dice_console,
},
Activity {
    name: "Character Loader",
    controller: character_loader::character_loader,
},
Activity {
    name: "Character Loader",
    controller: starfury::starfury_bombard,
},
Activity {
    name: "Rainworms Optimizer",
    controller: rainworms::rainworms_optimizer,
}
];