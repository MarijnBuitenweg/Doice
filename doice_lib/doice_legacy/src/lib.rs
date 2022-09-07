use rand::prelude::*;
use std::result::Result;

const _WELCOME_MSG: &str = "Welcome to this simple dice-rolling program.\nFormat:\n(number of dice)d(type of dice)+(optional dicemodifier) +(optional total modifier)";
const _ENTER_MSG: &str = "Enter a diceroll:";
const _EXIT_MSG: &str = "Goodbye!";
const NODICEDEF: &str = "Error: No dice defined.";
const INCORRECT_DICEMOD: &str = "Error: Incorrect dicemod definition.";
const INCORRECT_MOD: &str = "Error: Incorrect modifier definition.";
const _SUMSTRIPE: &str = "_______+";
const _ADVMASK: u8 = 1;
const _DISADVMASK: u8 = 2;

#[derive(Default, Clone)]
pub struct Roller {
    count: u32,
    dtype: u32,
    dmod: i32,
    tmod: i32,
    _flags: u8, //bit0=adv; bit1=disadv;
}

impl Roller {
    pub fn new(cnt: u32, dtype: u32, dmod: i32, tmod: i32, adv: bool, disadv: bool) -> Roller {
        Roller {
            count: cnt,
            dtype,
            dmod,
            tmod,
            _flags: (adv as u8) + 2 * (disadv as u8),
        }
    }

    pub fn from_text(txt: &str) -> Result<Roller, &'static str> {
        interpret_args(txt.split_whitespace().collect())
    }

    pub fn _roll(&self, gen: &mut ThreadRng) -> (i32, Vec<i32>) {
        let mut out = Vec::with_capacity(self.count as usize);
        let mut total = 0;

        for _ in 0..self.count {
            let roll = gen.gen_range(1..(self.dtype + 1));
            let sum = (roll as i32) + self.dmod;
            out.push(sum);
            total += sum;
        }

        (total + self.tmod, out)
    }

    pub fn roll_text(&self, rng: &mut ThreadRng) -> (String, i32) {
        let mut out = String::new();
        let mut total = 0;
        let dmod_str = self.dmod.abs().to_string();

        let d_sign = if self.dmod > 0 { " + " } else { " - " };

        let t_sign = if self.tmod > 0 { " + " } else { " - " };

        if self.count == 1 {
            let roll = rng.gen_range(1..(self.dtype + 1)) as i32;
            out += roll.to_string().as_str();
            if self.dmod != 0 || self.tmod != 0 {
                if self.dmod != 0 {
                    out += d_sign;
                    out += &dmod_str;
                }
                if self.tmod != 0 {
                    out += t_sign;
                    out += self.tmod.abs().to_string().as_str();
                }
                out += " = ";
                out += (roll + self.dmod + self.tmod).to_string().as_str();
            }
            //out.push('\n');
            return (out, roll + self.dmod + self.tmod);
        }

        if self.dmod != 0 {
            for _i in 0..self.count {
                let roll = rng.gen_range(1..(self.dtype + 1)) as i32;
                let sum = roll + self.dmod;
                out += &(roll.to_string() + d_sign + &dmod_str + " = " + &sum.to_string());
                out.push('\n');
                total += sum;
            }
        } else {
            for _i in 0..self.count {
                let roll = rng.gen_range(1..(self.dtype + 1)) as i32;
                out += &roll.to_string();
                out.push('\n');
                total += roll;
            }
        }

        //Add addition line
        out += "________+\n";

        //Add tmod if needed
        if self.tmod != 0 {
            out += &(total.to_string()
                + t_sign
                + self.tmod.abs().to_string().as_str()
                + " = "
                + (total + self.tmod).to_string().as_str());
        } else {
            out += &total.to_string();
        }
        //out.push('\n');

        (out, total + self.tmod)
    }
}

fn interpret_args(args: Vec<&str>) -> Result<Roller, &'static str> {
    //If line is empty, tell user
    if args.is_empty() {
        return Err(NODICEDEF);
    }

    let dice_def: Vec<&str> = args[0].split(|c| c == 'd' || c == 'D').collect();

    if dice_def.len() < 2 {
        return Err(NODICEDEF);
    }

    //Read modifier
    let mut modifier = 0;
    if args.len() > 1 && (args[1].contains('+') || args[1].contains('-')) {
        modifier = match args[1].parse::<i32>() {
            Ok(val) => val,
            Err(_) => return Err(INCORRECT_MOD),
        }
    }

    //Interpret dice type and count
    let dice_cnt = dice_def[0].parse::<u32>().unwrap_or(1);

    let dice_t_def;
    let dice_mod = match dice_def[1].find(&['+', '-'][..]) {
        Some(val) => {
            let mod_def = dice_def[1].split_at(val);
            dice_t_def = mod_def.0;
            match mod_def.1.parse::<i32>() {
                Ok(val) => val,
                Err(_) => return Err(INCORRECT_DICEMOD),
            }
        }
        None => {
            dice_t_def = dice_def[1];
            0
        }
    };

    let dice_type = match dice_t_def.parse::<u32>() {
        Ok(val) => val,
        Err(_) => return Err(NODICEDEF),
    };

    //Ok(Roller::new(dice_cnt, dice_type, dice_mod, modifier))
    Ok(Roller::new(
        dice_cnt, dice_type, dice_mod, modifier, false, false,
    ))
}
