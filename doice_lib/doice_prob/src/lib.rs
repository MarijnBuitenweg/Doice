pub mod chance;
pub mod distributions;
pub mod numerics;
pub mod stoch_error;

pub mod prelude {
    pub use crate::chance::*;
    pub use crate::numerics::*;
    pub use num::traits::*;
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
