//This file contains all the activities
use super::core::Activity;

mod dice_console;
mod character_loader;
mod starfury;

pub const ACTIVITY_ARRAY: [Activity; 3] = [Activity {
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
}];