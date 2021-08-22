use std::ops::{Deref, Add, Sub};
use proc_macro::TokenStream;
extern crate proc_macro;

#[derive(Debug)]
struct Weight(u32);

impl Deref for Weight {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Add for Weight {
    type Output = Weight;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Weight {
    type Output = Weight;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Add;

    #[test]
    fn cal_test() {
        let a = Weight(12);
        let b = Weight(44);
        let res = a + b;
        assert_eq!(res.0, Weight(56).0);
    }
}