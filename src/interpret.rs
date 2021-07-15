use crate::dnd_character::{*, self};
use crate::dice_rolls::Roller;

const STATNAMES: Vec<&'static str> = vec!["str", "dex", "con", "int", "wis", "cha"];
const SKILLNAMES: Vec<&'static str> = vec!["acr", "anh", "arc", "ath", "dec", "his", "ins", "int", "inv", "med", "nat", "pct", "pfm", "pss", "rel", "soh", "ste", "sur"];
const CLASSNAMES: Vec<&'static str> = dnd_character::CLASSNAMES.iter().map(|s| s.to_lowercase().as_str() ).collect();
const CATNAMES: Vec<&'static str> = vec!["general", "stats", "saves", "skills", "cantrips", "spells"];

#[derive(PartialEq)]
enum Line<'a> {
    Cat(usize),
    Val((&'a str, &'a str)),
}

type Category<'a> = (usize, Vec<Line<'a>>);

pub fn interpret_char(source: &str) -> Result<DndCharacter, &'static str> {
    let txt = source.to_lowercase();
    let mut temp: Vec<&str> = Vec::with_capacity(10);

    let lines: Vec<Line> = txt.lines()
        .filter_map(|l| {
            temp = l.split(':').collect();
            if temp.len() > 2 {
                None
            } else if temp.len() == 2 {
                Some(Line::Val((temp[0], temp[1])))
            } else {
                match SKILLNAMES.binary_search(&temp[0]) {
                    Ok(val) => Some(Line::Cat(val)),
                    Err(_) => None,
                }
            }
        }).collect();
    
    let mut cats = Vec::<Vec<(&str, &str)>>::with_capacity(CATNAMES.len());
    let mut cat = 0;
    for line in lines.into_iter() {
        match line {
            Line::Cat(val) => cat = val,
            Line::Val(val) => cats[cat].push(val),
        }
    }

    let general = interpret_general(cats[0])?;
    

    DndCharacter
}

fn interpret_general(lines: Vec<(&str, &str)>) -> Result<[usize; GENERALNAMES.len()], &'static str> {
    let mut out: [usize; GENERALNAMES.len()];
    for line in lines.iter() {
        if let Some(i) = GENERALNAMES.iter().position(|s| *s == (*line).0) {
            out[i] = match (*line).1.split(|c: char| c.is_whitespace()|| c=='+')
                .filter(|s| s.chars().all(|c| c.is_numeric()))
                .nth(0) {
                Some(s) => s.parse().unwrap(),
                None => return Err("ERR::interpret_general no score was found"),
            };
        }
    }

    Ok(out)
}