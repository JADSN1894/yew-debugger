use std::ops::{AddAssign, SubAssign};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct CounterModel(i64);

impl CounterModel {
    pub fn take(self) -> i64 {
        self.0
    }
}

impl From<i64> for CounterModel {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl AddAssign for CounterModel {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl SubAssign for CounterModel {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}
