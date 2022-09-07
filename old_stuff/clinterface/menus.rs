//This file contains all the information needed to construct the menus
use super::core::Menu;

pub const MENU_ARRAY: [Menu; 5] = [
    Menu {
        prompt: &concat!("Welcome to Doice ", env!("CARGO_PKG_VERSION"), ", a simple dice rolling program."),
        items: &["Dice console", "Load character", "Combat", "Character Creation", "Miscelaneous", "Exit"],
        mapping: &[-1, -2, 1, 2, 4, isize::MAX],
    },
    Menu {
        prompt: "Choose a combat-related category:",
        items: &["Special", "Back"],
        mapping: &[3, 0],
    },
    Menu {
        prompt: "Here is a set of character creation tools:",
        items: &["Back"],
        mapping: &[0],
    },
    Menu {
        prompt: "Special abilities that may or may not be tied to any specific character:",
        items: &["Starfury bombardment", "Back"],
        mapping: &[-3, 0]
    },
    Menu {
        prompt: "Random stuff that has no-where else to go:",
        items: &["Rainworms optimizer", "Dungeon randomizer", "Back"],
        mapping: &[-4, -5, 0],
    },
];