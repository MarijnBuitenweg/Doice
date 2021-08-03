use console::Term;
use dialoguer::{Confirm, Select};
use super::menus::MENU_ARRAY;
use super::activities::ACTIVITY_ARRAY;
use crate::dnd_character::DndCharacter;
use super::msg::display_msg;

//All the isize typed vars are indices, where negative values point to activities and positive ones to menus
//Setting the current screen to isize::MAX indicates to the program that the user wants to quit
pub struct Menu {
    pub prompt: &'static str,
    pub items: &'static [&'static str],
    pub mapping: &'static [isize],
}

pub struct Activity  {
    pub name: &'static str,
    pub controller: fn(isize, &mut AppData, &mut Term) -> Result<isize, &'static str>,
}

pub struct AppData {
    pub character: Option<DndCharacter>,
}

pub struct UI {
    menus: &'static [Menu],
    activities: &'static [Activity],
    past_screen: isize,
    current_screen: isize,
    term: Term,
    dat: AppData
}

impl UI {
    pub fn new() -> UI {
        UI {
            menus: &MENU_ARRAY,
            activities: &ACTIVITY_ARRAY,
            past_screen: 0,
            current_screen: 0,
            term: Term::stdout(),
            dat: AppData {character: None},
        }
    }

    pub fn run(&mut self) -> std::io::Result<usize> {
        while self.current_screen != isize::MAX {
            self.term.clear_screen()?;
            if self.current_screen >= 0 {
                self.run_menu()?;
            } else {
                self.run_activity()?;
            }
        }

        Ok(0)
    }

    fn run_menu(&mut self) -> std::io::Result<usize> {
        //obtain user input
        println!("{}", self.menus[self.current_screen as usize].prompt);
        let selection = Select::new()
            //.with_prompt(self.menus[self.current_screen as usize].prompt)
            .items(self.menus[self.current_screen as usize].items)
            .default(0)
            .interact()?;
    
        //Update history
        self.past_screen = self.current_screen;
        //Update future
        self.current_screen = self.menus[self.current_screen as usize].mapping[selection];

        Ok(0)
    }

    fn run_activity(&mut self) -> std::io::Result<usize> {
        //Calc correct activity index
        let i = (self.current_screen.abs() - 1) as usize;
        
        //Let the activity run
        let out = (self.activities[i].controller)(self.past_screen, &mut self.dat, &mut self.term);

        //Handle its output
        match out {
            //If OK: move to the indicated screen
            Ok(val) => {
                self.past_screen = self.current_screen;
                self.current_screen = val;
            },
            //If Err: Show msg and go back to previous screen
            Err(s) => {
                display_msg(s, &mut self.term);
                self.current_screen = self.past_screen;
            }
        }

        Ok(0)
    }
}
