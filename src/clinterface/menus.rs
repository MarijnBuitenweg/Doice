//This file contains all the information needed to construct the menus
use super::core::Menu;

pub const MENU_ARRAY: [Menu; 1] = [
    Menu {
        prompt: "Welcome to Doice, a simple dice rolling program.",
        items: &["Dice console", "Load character", "Exit"],
        mapping: &[-1, -2, isize::MAX],
    },
];