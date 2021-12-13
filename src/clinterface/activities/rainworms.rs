use crate::clinterface::core::AppData;

use console::Term;
use dialoguer::Input;

pub fn rainworms_optimizer(_: isize, dat: &mut AppData, term: &mut Term) -> Result<isize, &'static str> {
    //If no game instantiated, intantiate one
    if let None = dat.rainwormsgame {
        dat.rainwormsgame = Some(init_game(term)?);
    }
    Ok(0)
}

fn init_game(term: &mut Term) -> Result<Rainworms, &'static str> {
    let playercount: u8 = Input::new()
        .with_prompt("How many players?")
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
    Ok(Rainworms::new(playercount))
}

//bij regenwormen liggen er stenen van 21 tot en met 36 op tafel
//Speler gooit acht dobbelstenen en zet elke keer dat ie ze gooit een cijfer apart
//Elk cijfer kan maar eenmaal apart worden gezet
//Er moet minstens 1 zes/regenworm apart worden gezet om in aanmerking te komen voor een steen
//De laatste steen die een andere speler heeft gekregen kan worden gestolen als iemand anders precies de juiste score behaalt
type Piece = u8;

trait Score {
    fn score(self) -> Self;
}

impl Score for Piece {
    fn score(self) -> Self {
        (self - 17) >> 2
    }
}

pub struct Rainworms {
    board: [Piece; 16],
    player_hands: Vec<Vec<Piece>>,
}

impl Rainworms {
    pub fn new(player_count: u8) -> Self {
        Rainworms { 
            board: [0; 16],
            player_hands: vec![Vec::new(); player_count as usize], 
        }
    }
}