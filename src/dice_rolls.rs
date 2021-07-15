use std::result::Result;
use rand::prelude::*;

const WELCOME_MSG: &'static str = "Welcome to this simple dice-rolling program.\nFormat:\n(number of dice)d(type of dice)+(optional dicemodifier) +(optional total modifier)";
const ENTER_MSG: &'static str = "Enter a diceroll:";
const EXIT_MSG: &'static str = "Goodbye!";
const NODICEDEF: &'static str = "Error: No dice defined.";
const INCORRECT_DICEMOD: &'static str = "Error: Incorrect dicemod definition.";
const INCORRECT_MOD: &'static str = "Error: Incorrect modifier definition.";
const SUMSTRIPE: &'static str = "_______+";
const ADVMASK: u8 = 1;
const DISADVMASK: u8 = 2;

pub struct Roller {
    count: u32,
    dtype: u32,
    dmod: u32,
    tmod: u32,
    flags: u8, //bit0=adv; bit1=disadv;  
}

impl Roller {
    pub fn new(cnt: u32, dtype: u32, dmod: u32, tmod: u32, adv: bool, disadv: bool) -> Roller {
        Roller{
            count: cnt,
            dtype: dtype,
            dmod: dmod,
            tmod: tmod,
            flags: (adv as u8) + 2*(disadv as u8),
        }
    }

    pub fn from_text(txt: &str) -> Result<Roller, &'static str> {
        interpret_args(txt.split_whitespace().collect())
    }

    pub fn roll(&self, gen: &mut ThreadRng) -> (u32, Vec<u32>) {
        let mut out = Vec::with_capacity(self.count as usize);
        let mut total = 0;

        for i in 0..self.count {
            let roll = gen.gen_range(1,self.dtype + 1);
            let sum = roll + self.dmod;
            out.push(sum);
            total += sum;
        }
        
        (total + self.tmod, out)
    }

    pub fn roll_text(&self, gen: &mut ThreadRng) -> String {
        let mut out = String::new();
        let mut total = 0;
        let dmod_str = self.dmod.to_string();
        if self.count == 1 {
            let roll = gen.gen_range(1,self.dtype + 1);
            if self.dmod > 0 {
                return roll.to_string() + " + " + &dmod_str + " = " + &(roll + self.dmod).to_string();
            } else {
                return roll.to_string();
            }
        }

        if self.tmod > 0 {
            for i in 0..self.count {
                let roll = gen.gen_range(1,self.dtype + 1);
                let sum = roll + self.dmod;
                out += &(roll.to_string() + " + " + &dmod_str + " = " + &sum.to_string());
                out.push('\n');
                total += sum;
            }
        } else {
            for i in 0..self.count {
                let roll = gen.gen_range(1,self.dtype + 1);
                out += &roll.to_string();
                out.push('\n');
                total += roll;
            }
        }

        if self.tmod > 0 {

        }

        out
    }
}

fn interpret_args(args: Vec<&str>) -> Result<Roller, &'static str> {
    let mut dice_def: Vec<&str> = args[0].split(
        |c| c == 'd' || c == 'D'
    ).collect();

    if dice_def.len() < 2 {
        return Err(NODICEDEF);
    }

    //Read modifier
    let mut modifier = 0;
    if args.len() > 1 {
        if args[1].contains('+') {
            let (_, temp) = args[1].split_at(1);
            modifier = match temp.parse::<u32>() {
                Ok(val) => val,
                Err(_) => return Err(INCORRECT_MOD)
            }
        }
    }

    //Interpret dice type and count
    let dice_cnt = dice_def[0].parse::<u32>().unwrap_or(1);
    let mod_def: Vec<&str> = dice_def[1].split('+').collect();
    dice_def[1] = mod_def[0];
    let dice_type = match dice_def[1].parse::<u32>() {
        Ok(val) => val,
        Err(e) => return Err(NODICEDEF)
    };

    let mut dice_mod = 0;
    if mod_def.len() > 1 {
        dice_mod = match mod_def[1].parse::<u32>() {
            Ok(val) => val,
            Err(e) => return Err(INCORRECT_DICEMOD)
        };
    }

    Ok(Roller::new(dice_cnt, dice_type, dice_mod, modifier))
    
}
