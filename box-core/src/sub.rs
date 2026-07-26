use std::ops::Sub;

use crate::{BoxType, BoxValue, BoxVariant};

impl<T: BoxType> Sub for BoxValue<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-1) * rhs
    }
}

impl Sub for BoxVariant {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-1) * rhs
    }
}
