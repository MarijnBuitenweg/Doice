use crate::{dice_roller::{DiceAdapter, DiceOption}, DiceRoller};

mod keep_highest;
mod reroll_eq;

/// Parses and initializes a diceadapter
/// The only thing an adapter usually needs to specify itself it the IDENT
trait DiceAdapterGen: DiceAdapter {
    const IDENT: &'static str;
    fn gen(src: &str, _: &DiceRoller) -> Option<DiceOption>
    where
        Self: From<usize> + 'static,
    {
        Self::fetch_arg(src)
            .map(|n| n.into())
            .map(|adapt: Self| adapt.into())
    }

    fn fetch_arg(src: &str) -> Option<usize> {
        if src.starts_with(Self::IDENT) {
            let arg_len = src.chars().skip(2).take_while(|c| c.is_numeric()).count();
            if arg_len == 0 {
                return None;
            }
            let count = src[2..(3 + arg_len)].parse().ok()?;
            Some(count)
        } else {
            None
        }
    }
}

type DiceAdapterGenFn = fn(&str, &DiceRoller) -> Option<DiceOption>;
pub const DICE_ADAPTER_GENERATORS: [DiceAdapterGenFn; 2] =
    [keep_highest::KeepHighest::gen, reroll_eq::RerollEq::gen];

pub fn generate_dice_adapters(src: &str, dice: &DiceRoller) -> Option<DiceOption> {
    DICE_ADAPTER_GENERATORS
        .iter()
        .find_map(|gen| (gen)(src, dice))
}
