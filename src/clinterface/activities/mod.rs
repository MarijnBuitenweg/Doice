//This file contains all the activities
use super::core::Activity;

mod dice_console;

pub const ACTIVITY_ARRAY: [Activity; 1] = [Activity {
    name: "Dice Console",
    controller: dice_console::dice_console,
}];