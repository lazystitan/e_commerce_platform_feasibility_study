use std::ops::{Add, Deref, Sub, Div, Mul};

#[derive(Debug, Copy, Clone)]
struct UniqueNumber<TP, VAL>
    where VAL: Add<Output=VAL> + Sub<Output=VAL> + Div<Output=VAL> + Mul<Output=VAL>,
          TP: Copy + Default
{
    __type: TP,
    __value: VAL,
}

// type FundMathOps = dyn Add + Sub;

impl<TP, VAL> UniqueNumber<TP, VAL>
    where VAL: Add<Output=VAL> + Sub<Output=VAL> + Div<Output=VAL> + Mul<Output=VAL>,
          TP: Copy + Default
{
    pub fn new(value: VAL) -> Self {
        Self {
            __type: Default::default(),
            __value: value,
        }
    }
}

impl<TP, VAL> Deref for UniqueNumber<TP, VAL>
    where VAL: Add<Output=VAL> + Sub<Output=VAL> + Div<Output=VAL> + Mul<Output=VAL>,
          TP: Copy + Default
{
    type Target = VAL;

    fn deref(&self) -> &Self::Target {
        &self.__value
    }
}

impl<TP, VAL> Add for UniqueNumber<TP, VAL>
    where VAL: Add<Output=VAL> + Sub<Output=VAL> + Div<Output=VAL> + Mul<Output=VAL>,
          TP: Copy + Default
{
    type Output = UniqueNumber<TP, VAL>;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            __type: self.__type,
            __value: self.__value + rhs.__value,
        }
    }
}

impl<TP, VAL> Sub for UniqueNumber<TP, VAL>
    where VAL: Add<Output=VAL> + Sub<Output=VAL> + Div<Output=VAL> + Mul<Output=VAL>,
          TP: Copy + Default
{
    type Output = UniqueNumber<TP, VAL>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            __type: self.__type,
            __value: self.__value - rhs.__value,
        }
    }
}

impl<TP, VAL> Mul for UniqueNumber<TP, VAL>
    where VAL: Add<Output=VAL> + Sub<Output=VAL> + Div<Output=VAL> + Mul<Output=VAL>,
          TP: Copy + Default
{
    type Output = UniqueNumber<TP, VAL>;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            __type: self.__type,
            __value: self.__value / rhs.__value,
        }
    }
}


#[derive(Copy, Clone, Debug, Default)]
struct WeightGramType;
type WeightGram = UniqueNumber<WeightGramType, u32>;

impl Mul<u32> for WeightGram {
    type Output = WeightGram;

    fn mul(self, rhs: u32) -> Self::Output {
        Self {
            __type: self.__type,
            __value: self.__value  * rhs
        }
    }
}

impl Mul<WeightGram> for u32 {
    type Output = WeightGram;

    fn mul(self, rhs: WeightGram) -> Self::Output {
        WeightGram::new(*rhs * self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cal_test() {
        let w1 = WeightGram::new(12);
        let w2 = WeightGram::new(42);
        assert_eq!(*(w1 + w2), *WeightGram::new(54));

        let w12sum = w1 + w2;
        assert_eq!(*w12sum, *WeightGram::new(54));

        let w3 = WeightGram::new(4);
        assert_eq!(*(w3*3), *WeightGram::new(12));
        assert_eq!(*(3*w3), *WeightGram::new(12));
    }

    #[test]
    fn cal_basic_diff_types_test() {
        let n1 = 34 as u32;
        let n2 = 56 as i32;
        // assert_eq!(n1 + n2, 90);
        assert_eq!(n1 + n2 as u32, 90);
    }
}