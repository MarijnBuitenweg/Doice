use std::{
    fmt::{Debug, Display},
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use num::{One, ToPrimitive, Zero};
use thiserror::Error;

use crate::numerics::{BasicNum, SignInfo};

/// Generic numeric type, optimized to represent a chance
#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub struct Chance<T>(T);

impl<T> Chance<T> {
    pub fn inner(&self) -> &T {
        &self.0
    }
}

impl<T: BasicNum> Chance<T> {
    pub fn new(val: impl Into<T>) -> Result<Self, ChanceError> {
        let out = Chance(val.into());
        out.validate()?;
        Ok(out)
    }

    /// Checks whether the chance is valid.
    /// You shouldn't need to call this yourselves.
    /// It's public for verification purposes.
    pub fn validate(&self) -> Result<(), ChanceError> {
        if self.0.seems_negative() {
            Err(ChanceError::NegativeChance)
        } else if self.0 > T::one() {
            Err(ChanceError::Overflow)
        } else {
            Ok(())
        }
    }
}

impl<T: BasicNum + Clone> Chance<T> {
    pub fn abs_diff(&self, other: &Self) -> Self {
        if self > other {
            self.clone() - other.clone()
        } else {
            other.clone() - self.clone()
        }
    }
}

impl<T: BasicNum> Chance<T> {
    /// Fallible add.
    pub fn try_add(self, rhs: Self) -> Result<Self, ChanceError> {
        let out = Chance(self.0 + rhs.0);
        out.validate()?;
        Ok(out)
    }

    /// Fallible subtraction.
    pub fn try_sub(self, rhs: Self) -> Result<Self, ChanceError> {
        let out = Chance(self.0 - rhs.0);
        out.validate()?;
        Ok(out)
    }
}

impl<T: BasicNum> Add<Self> for Chance<T> {
    type Output = Self;

    /// # Panics
    /// - If the result is greater than 1
    /// - If the result is under 0
    fn add(self, rhs: Self) -> Self::Output {
        let out = Chance(self.0 + rhs.0);
        out.validate()
            .expect("chance addition should produce a valid chance");
        out
    }
}

impl<T: BasicNum + Clone> Add<&Self> for Chance<T> {
    type Output = Chance<T>;

    /// # Panics
    /// - If the result is greater than 1
    /// - If the result is under 0
    fn add(self, rhs: &Self) -> Self::Output {
        let out = Chance(self.0 + rhs.0.clone());
        out.validate()
            .expect("chance addition should produce a valid chance");
        out
    }
}

impl<T: BasicNum + Clone> Add<Self> for &Chance<T> {
    type Output = Chance<T>;

    /// # Panics
    /// - If the result is greater than 1
    /// - If the result is under 0
    fn add(self, rhs: Self) -> Self::Output {
        let out = Chance(self.0.clone() + rhs.0.clone());
        out.validate()
            .expect("chance addition should produce a valid chance");
        out
    }
}

impl<T: BasicNum + Clone> Add<&Self> for &Chance<T> {
    type Output = Chance<T>;

    /// # Panics
    /// - If the result is greater than 1
    /// - If the result is under 0
    fn add(self, rhs: &Self) -> Self::Output {
        let out = Chance(self.0.clone() + rhs.0.clone());
        out.validate()
            .expect("chance addition should produce a valid chance");
        out
    }
}

impl<T: BasicNum + AddAssign> AddAssign for Chance<T> {
    /// # Panics
    /// - If the result is greater than 1
    /// - If the result is under 0
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.validate()
            .expect("chance addition should produce a valid chance");
    }
}

impl<T: BasicNum> Sub<Self> for Chance<T> {
    type Output = Self;

    /// # Panics
    /// - If the result is greater than 1
    /// - If the result is under 0
    fn sub(self, rhs: Self) -> Self::Output {
        let out = Chance(self.0 - rhs.0);
        out.validate()
            .expect("chance subtraction should produce a valid chance");
        out
    }
}

impl<T: BasicNum + SubAssign> SubAssign for Chance<T> {
    /// # Panics
    /// - If the result is greater than 1
    /// - If the result is under 0
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.validate()
            .expect("chance subtraction should produce a valid chance");
    }
}

impl<T: BasicNum> Mul<Self> for Chance<T> {
    type Output = Self;

    /// Equivalent to mul_unchecked thanks to type constraints.
    fn mul(self, rhs: Self) -> Self::Output {
        Chance(self.0 * rhs.0)
    }
}

impl<T: BasicNum + Clone> Mul<&Self> for Chance<T> {
    type Output = Self;

    /// Equivalent to mul_unchecked thanks to type constraints.
    fn mul(self, rhs: &Self) -> Self::Output {
        Chance(self.0 * rhs.0.clone())
    }
}

impl<T: BasicNum + Clone> Mul<Self> for &Chance<T> {
    type Output = Chance<T>;

    /// Equivalent to mul_unchecked thanks to type constraints.
    fn mul(self, rhs: Self) -> Self::Output {
        Chance(self.0.clone() * rhs.0.clone())
    }
}

impl<T: BasicNum + Clone> Mul<&Self> for &Chance<T> {
    type Output = Chance<T>;

    /// Equivalent to mul_unchecked thanks to type constraints.
    fn mul(self, rhs: &Self) -> Self::Output {
        Chance(self.0.clone() * rhs.0.clone())
    }
}

impl<T: BasicNum + MulAssign> MulAssign for Chance<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}

impl<T: BasicNum> Div<Self> for Chance<T> {
    type Output = Self;

    /// # Panics
    /// - If the result is greater than 1
    /// - If the result is under 0
    fn div(self, rhs: Self) -> Self::Output {
        let out = Chance(self.0 / rhs.0);
        out.validate()
            .expect("chance division should yield a valid chance");
        out
    }
}

impl<T: BasicNum + DivAssign> DivAssign for Chance<T> {
    /// # Panics
    /// - If the result is greater than 1
    /// - If the result is under 0
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
        self.validate()
            .expect("chance division should yield a valid chance");
    }
}

impl<T: ToPrimitive> ToPrimitive for Chance<T> {
    fn to_i64(&self) -> Option<i64> {
        self.0.to_i64()
    }

    fn to_u64(&self) -> Option<u64> {
        self.0.to_u64()
    }

    fn to_f32(&self) -> Option<f32> {
        self.0.to_f32()
    }

    fn to_f64(&self) -> Option<f64> {
        self.0.to_f64()
    }
}

impl<T: ToPrimitive> Chance<T> {
    /// # Panics
    /// - If the contained chance cannot be represented by an f64
    pub fn to_f64_chance(&self) -> Chance<f64> {
        Chance(self.0.to_f64().expect("value must be representable as f64"))
    }
}

impl<T: Default> Default for Chance<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T: BasicNum> One for Chance<T> {
    fn one() -> Self {
        Chance(T::one())
    }
}

impl<T: BasicNum> Zero for Chance<T> {
    fn zero() -> Self {
        Chance(T::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl<T: BasicNum> Sum for Chance<T> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut acc = Self::zero();
        for num in iter {
            acc = acc + num;
        }
        acc
    }
}

pub trait ToChance: Sized {
    fn to_chance(self) -> Result<Chance<Self>, ChanceError>;
}

impl<T: BasicNum> ToChance for T {
    fn to_chance(self) -> Result<Chance<Self>, ChanceError> {
        Chance::new(self)
    }
}

#[derive(Error, Debug)]
pub enum ChanceError {
    NegativeChance,
    Overflow,
}

impl Display for ChanceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ChanceError::NegativeChance => "encountered negative probability",
            ChanceError::Overflow => "encountered probability greater than 1",
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn from_test() {
        let _chance: Chance<_> = 0.3.to_chance().unwrap();
    }
}
