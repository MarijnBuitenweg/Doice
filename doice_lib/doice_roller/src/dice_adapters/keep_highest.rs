use crate::{
    dice_roller::{DiceAdapter, DiceRoller},
    Value,
};

use super::DiceAdapterGen;

#[derive(Clone, Default, Debug)]
pub struct KeepHighest {
    count: usize,
}

impl DiceAdapter for KeepHighest {
    fn run(
        &self,
        _input: Vec<Value>,
        _out_txt: &mut crate::Layouter,
        _d_info: &DiceRoller,
    ) -> Vec<crate::Value> {
        todo!()
    }

    fn dist(&self, _input: crate::ProbDist) -> crate::ProbDist {
        todo!()
    }
}

impl From<usize> for KeepHighest {
    fn from(count: usize) -> Self {
        KeepHighest { count }
    }
}

impl DiceAdapterGen for KeepHighest {
    const IDENT: &'static str = "kh";
}
