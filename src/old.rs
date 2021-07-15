
use rand::prelude::*;
use std::io::stdin;
use std::result::Result;

pub fn run_old() {
    println!("Welcome to the roller of dice.");
    let mut read_txt: String = String::with_capacity(5);
    loop {
        println!("Enter roll:");
        stdin().read_line(&mut read_txt);
        if read_txt.as_str().contains('q') {
            break;
        }
        println!("{}", interpret_and_run(&read_txt).unwrap());
        read_txt.clear();
        println!("");
    }
    println!("Thank you for choosing Doice.");
}

pub fn interpret_and_run(text: &str) -> Result<String, ()> {
    let mut bonus = 0;
    
    let mut parts: Vec<&str> = text.split_whitespace().collect();
    let mut dice_def: Vec<&str> = parts[0].split('d').collect();
    let factor = match dice_def[0].parse::<i32>() {
        Ok(val) => val,
        Err(e) => 1,
    };
    let dice = dice_def[1].parse::<i32>().unwrap();
    if parts.len() > 1 {
        bonus = match parts[1].parse::<i32>() {
            Ok(val) => val,
            Err(e) => 0,
        };
    }

    let mut rng = thread_rng();
    let mut total_dice = 0;
    for i in 0..factor {
        let roll = rng.gen_range(1,dice + 1);
        if factor > 1 { println!("{}", roll); }
        total_dice += roll;
    }
    let final_total = total_dice + bonus;
    if factor > 1 { println!("___+"); }
    if bonus != 0 {
        Ok(total_dice.to_string() + " + " + &bonus.to_string() + " = " + &final_total.to_string())
    } else {
        Ok(total_dice.to_string())
    }
}
