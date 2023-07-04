use std::ops::MulAssign;

use num::{Bounded, FromPrimitive, Num};

pub trait BasicNum: Num + Bounded + PartialOrd + FromPrimitive + MulAssign {}
impl<T: Num + Bounded + PartialOrd + FromPrimitive + MulAssign> BasicNum for T {}

pub trait SignInfo {
    fn seems_signed() -> bool;
    fn seems_negative(&self) -> bool;
    fn seems_positive(&self) -> bool;
}

impl<T: BasicNum> SignInfo for T {
    fn seems_signed() -> bool {
        T::zero() > T::min_value()
    }

    fn seems_negative(&self) -> bool {
        *self < T::zero()
    }

    fn seems_positive(&self) -> bool {
        *self > T::zero()
    }
}
