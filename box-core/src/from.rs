use malachite::{Integer, Natural, base::num::arithmetic::traits::UnsignedAbs};

use crate::{BoxValue, BoxVariant, NumBox};

impl From<u32> for BoxValue<NumBox> {
    fn from(value: u32) -> Self {
        let zero = BoxValue::zero();
        if value == 0 {
            return zero.cast();
        }
        zero.wrap::<NumBox>(value)
    }
}

impl From<u64> for BoxValue<NumBox> {
    fn from(value: u64) -> Self {
        let zero = BoxValue::zero();
        if value == 0 {
            return zero.cast();
        }
        zero.wrap::<NumBox>(value)
    }
}

impl From<i32> for BoxValue<NumBox> {
    fn from(value: i32) -> Self {
        let zero = if value >= 0 {
            BoxValue::zero()
        } else {
            BoxValue::anti_zero()
        };

        if value == 0 {
            return zero.cast();
        }

        zero.wrap::<NumBox>(value.unsigned_abs())
    }
}

impl From<i64> for BoxValue<NumBox> {
    fn from(value: i64) -> Self {
        let zero = if value >= 0 {
            BoxValue::zero()
        } else {
            BoxValue::anti_zero()
        };

        if value == 0 {
            return zero.cast();
        }

        zero.wrap::<NumBox>(value.unsigned_abs())
    }
}

impl From<u32> for BoxVariant {
    fn from(value: u32) -> Self {
        let zero = BoxValue::zero();
        if value == 0 {
            return zero.into();
        }
        zero.wrap::<NumBox>(value).into()
    }
}

impl From<u64> for BoxVariant {
    fn from(value: u64) -> Self {
        let zero = BoxValue::zero();
        if value == 0 {
            return zero.into();
        }
        zero.wrap::<NumBox>(value).into()
    }
}

impl From<i32> for BoxVariant {
    fn from(value: i32) -> Self {
        let zero = if value >= 0 {
            BoxValue::zero()
        } else {
            BoxValue::anti_zero()
        };

        if value == 0 {
            return zero.into();
        }

        zero.wrap::<NumBox>(value.unsigned_abs()).into()
    }
}

impl From<i64> for BoxVariant {
    fn from(value: i64) -> Self {
        let zero = if value >= 0 {
            BoxValue::zero()
        } else {
            BoxValue::anti_zero()
        };

        if value == 0 {
            return zero.into();
        }

        zero.wrap::<NumBox>(value.unsigned_abs()).into()
    }
}

impl From<Natural> for BoxValue<NumBox> {
    fn from(value: Natural) -> Self {
        let zero = BoxValue::zero();
        if value == 0 {
            return zero.cast();
        }
        zero.wrap::<NumBox>(value)
    }
}

impl From<Natural> for BoxVariant {
    fn from(value: Natural) -> Self {
        let zero = BoxValue::zero();
        if value == 0 {
            return zero.into();
        }
        zero.wrap::<NumBox>(value).into()
    }
}

impl From<Integer> for BoxValue<NumBox> {
    fn from(value: Integer) -> Self {
        let zero = if value >= 0 {
            BoxValue::zero()
        } else {
            BoxValue::anti_zero()
        };

        if value == 0 {
            return zero.cast();
        }

        zero.wrap::<NumBox>(value.unsigned_abs())
    }
}

impl From<Integer> for BoxVariant {
    fn from(value: Integer) -> Self {
        let zero = if value >= 0 {
            BoxValue::zero()
        } else {
            BoxValue::anti_zero()
        };

        if value == 0 {
            return zero.into();
        }

        zero.wrap::<NumBox>(value.unsigned_abs()).into()
    }
}
