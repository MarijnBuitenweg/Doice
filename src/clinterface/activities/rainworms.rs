use crate::clinterface::core::AppData;

use console::Term;
use dialoguer::Input;
use rand::{prelude::ThreadRng, Rng};

const DICE_COUNT: usize = 8;

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
    fn score(&self) -> u8;
}

impl Score for Piece {
    fn score(&self) -> u8 {
        (*self - 17) >> 2
    }
}

impl Score for Vec<Piece> {
    fn score(&self) -> u8 {
        self.iter().fold(0, |cnt, &p| cnt + p.score())
    }
}

pub struct Rainworms {
    board: [Piece; 16],
    player_hands: Vec<Vec<Piece>>,
    inv: [u8; DICE_COUNT],
    inv_count: u8,
    dice: [u8; DICE_COUNT],
}

impl Rainworms {
    pub fn new(player_count: u8) -> Self {
        let mut temp_board = [0; 16];
        for i in 0..16 {
            temp_board[i] = (i + 21) as u8;
        }

        Rainworms { 
            board: temp_board,
            player_hands: vec![Vec::new(); player_count as usize], 
            inv: [0; DICE_COUNT],
            inv_count: 0,
            dice: [0; DICE_COUNT],
        }
    }

    pub fn roll(&mut self, rng: &mut ThreadRng) -> &[u8] { 
        for i in 0..(DICE_COUNT - self.inv_count as usize) {
            self.dice[i] = rng.gen_range(1, 7);
        }
        &self.dice[0..(DICE_COUNT - self.inv_count as usize)]
    }

    pub fn dice(&self) -> &[u8] {
        &self.dice[0..(DICE_COUNT - self.inv_count as usize)]
    }

    pub fn set_aside(&mut self, num: u8) {
        for i in 0..(DICE_COUNT - self.inv_count as usize) {
            if self.dice[i] == num {
                self.add_to_inv(num);
            }
        }
        self.dice = [0; DICE_COUNT];
    }

    fn add_to_inv(&mut self, num: u8) {
        self.inv[self.inv_count as usize] = num;
        self.inv_count += 1;
    }

    pub fn hands(&self) -> &[Vec<Piece>] {
        &self.player_hands
    }

    pub fn inventory(&self) -> &[u8] {
        &self.inv[0..self.inv_count as usize]
    }

    pub fn inv_score(&self) -> (u8, bool) {
        (self.inv.iter().sum(), self.inv.iter().any(|&n| n == 6))
    }

    pub fn give_piece(&mut self, piece: Piece, player: u8) {
        for i in 0..self.board.len() {
            if self.board[i] == piece {
                self.board[i] = 0;
                self.player_hands[player as usize].push(piece);
                break;
            }
        }
    }

    pub fn end_turn(&mut self) {
        let (score, valid) = self.inv_score();
        self.inv = [0; DICE_COUNT];

        if valid && score >= 21 {
            for i in 1..self.player_hands.len() {
                for j in 0..self.player_hands[i].len() {
                    if self.player_hands[i][j] == score {
                        self.player_hands[i].remove(j);
                        self.player_hands[0].push(score);
                        return;
                    }
                }
            }

            for i in (score as usize - 21)..0 {
                if self.board[i] != 0 {
                    self.player_hands[0].push(self.board[i]);
                    self.board[i] = 0;
                }
            }
        } else if let Some(piece) = self.player_hands[0].pop() {
            self.board[piece as usize - 21] = piece;
        }
    }

}