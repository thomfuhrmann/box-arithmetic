use std::ops::Div;

use malachite::Natural;
use rapidhash::RapidHashMap;

use crate::{
    AnyBox, BoxKind, BoxType, BoxValue, BoxVariant, Color, MultinumBox, NumBox, PolynumBox,
};

/// Trait for the output type of box division
pub trait BoxDiv<Rhs = Self> {
    type Output: BoxType;
}

impl<T: BoxType> BoxDiv for T {
    type Output = Self;
}

macro_rules! impl_box_div {
    ($lhs:ty, $rhs:ty => $out:ty) => {
        impl BoxDiv<$rhs> for $lhs {
            type Output = $out;
        }
        impl BoxDiv<$lhs> for $rhs {
            type Output = $out;
        }
    };
}

impl_box_div!(NumBox, PolynumBox => PolynumBox);
impl_box_div!(NumBox, MultinumBox => MultinumBox);
impl_box_div!(PolynumBox, MultinumBox => MultinumBox);

impl<L: BoxType + BoxDiv<R>, R: BoxType> Div<BoxValue<R>> for BoxValue<L> {
    type Output = BoxValue<L::Output>;

    /// Divide two boxes
    fn div(mut self, rhs: BoxValue<R>) -> Self::Output {
        let mut result = BoxValue::new();
        let mut unique_children: RapidHashMap<u64, BoxValue<AnyBox>> = RapidHashMap::default();

        let lhs_col = self.get_color(0);
        let rhs_col = rhs.get_color(0);

        let lhs_kind = self.get_kind(0);
        let rhs_kind = rhs.get_kind(0);

        let mut rhs_iter = rhs.into_iter();
        let Some(divisor) = rhs_iter.next() else {
            panic!("No divisor value: {:?}", rhs_iter);
        };

        while self.get_length(0) > 1 {
            let mut lhs_iter = self.clone().into_iter();
            if let Some(dividend) = lhs_iter.next() {
                let divisor_mul = divisor.get_multiplicity(0);
                let mut dividend_mul = dividend.get_multiplicity(0);

                let dividend_kind = dividend.get_kind(0);
                let divisor_kind = divisor.get_kind(0);

                // case 1: both children are empty
                if dividend_kind == BoxKind::Empty && divisor_kind == BoxKind::Empty {
                    let factor = BoxValue::from(divisor_mul.clone());

                    let mut mul = Natural::from(0_u32);
                    while dividend_mul >= divisor_mul {
                        dividend_mul -= divisor_mul.clone();
                        mul += Natural::from(1_u32);
                    }

                    if dividend_mul == 0 {
                        let mut val = BoxValue::zero();
                        val.set_multiplicity(0, mul.clone());
                        let struct_hash = val.hash_content(unique_children.hasher());
                        unique_children.insert(struct_hash, val.cast());
                    } else {
                        return BoxValue::zero().cast();
                    }

                    // subtract first divisor child box
                    self = self - mul.clone() * factor.cast::<L>();

                    // subtract other divisor child boxes
                    for divisor in rhs_iter.clone() {
                        let wrapped = divisor.wrap::<L>(1_u32);
                        self = self - wrapped;
                    }
                    continue;
                }

                // case 2: dividend child is number and divisor child is empty
                if dividend_kind == BoxKind::Num && divisor_kind == BoxKind::Empty {
                    let mut factor = BoxValue::from(divisor_mul.clone()).cast::<PolynumBox>();

                    let mut exp = dividend.get_multiplicity(1);
                    while exp > 1 {
                        factor = factor * BoxValue::alpha();
                        exp -= Natural::from(1_u32);
                    }

                    let mut mul = Natural::from(0_u32);
                    while dividend_mul >= divisor_mul {
                        dividend_mul -= divisor_mul.clone();
                        mul += Natural::from(1_u32);
                    }

                    if dividend_mul == 0 {
                        let mut val = BoxValue::zero();
                        val.set_multiplicity(0, mul.clone());
                        let struct_hash = val.hash_content(unique_children.hasher());
                        unique_children.insert(struct_hash, val.cast());
                    } else {
                        return BoxValue::zero().cast();
                    }

                    // subtract first divisor child box
                    self = self - mul.clone() * factor.cast::<L>();

                    // subtract other divisor child boxes
                    for divisor in rhs_iter.clone() {
                        let wrapped = divisor.wrap::<L>(mul.clone());
                        self = self - wrapped;
                    }
                    continue;
                }

                // case 3: both child operands have at least depth 1
                let mut factor = BoxValue::from(divisor_mul.clone()).cast::<PolynumBox>();

                let mut exp = dividend.get_multiplicity(1);
                while exp > 1 {
                    factor = factor * BoxValue::alpha();
                    exp -= Natural::from(1_u32);
                }

                let mut mul = Natural::from(0_u32);
                while dividend_mul >= divisor_mul {
                    dividend_mul -= divisor_mul.clone();
                    mul += Natural::from(1_u32);
                }

                if dividend_mul == 0 {
                    let mut val = BoxValue::zero();
                    val.set_multiplicity(0, mul.clone());
                    let struct_hash = val.hash_content(unique_children.hasher());
                    unique_children.insert(struct_hash, val.cast());
                } else {
                    return BoxValue::zero().cast();
                }

                self = self - mul * factor.cast::<L>();
            }
        }

        result.kinds.push(lhs_kind + rhs_kind);
        result.colors.push(lhs_col + rhs_col);
        result.multiplicities.push(Natural::from(1_u32));
        result.lengths.push(1);

        for raw_box in unique_children.into_values() {
            let mul = raw_box.get_multiplicity(0);
            if mul == 0 {
                continue;
            }

            result.extend(raw_box);
        }

        result.sort_immediate_children();
        result
    }
}

impl Div for BoxVariant {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (BoxVariant::Empty(l), r) => {
                let l_col = l.get_color(0);
                let r_col = r.get_color(0);
                match l_col * r_col {
                    Color::Black => BoxValue::zero().into(),
                    Color::Red => BoxValue::anti_zero().into(),
                }
            }
            (_, BoxVariant::Empty(_)) => panic!("Division by zero is not allowed"),
            (BoxVariant::Any(l), BoxVariant::Any(r)) => BoxVariant::repack_raw(l / r),
            (BoxVariant::Num(l), BoxVariant::Num(r)) => BoxVariant::repack_raw(l / r),
            (BoxVariant::Num(l), BoxVariant::Polynum(r)) => BoxVariant::repack_raw(l / r),
            (BoxVariant::Polynum(l), BoxVariant::Num(r)) => BoxVariant::repack_raw(l / r),
            (BoxVariant::Polynum(l), BoxVariant::Polynum(r)) => BoxVariant::repack_raw(l / r),
            (BoxVariant::Num(l), BoxVariant::Multinum(r)) => BoxVariant::repack_raw(l / r),
            (BoxVariant::Multinum(l), BoxVariant::Num(r)) => BoxVariant::repack_raw(l / r),
            (BoxVariant::Polynum(l), BoxVariant::Multinum(r)) => BoxVariant::repack_raw(l / r),
            (BoxVariant::Multinum(l), BoxVariant::Polynum(r)) => BoxVariant::repack_raw(l / r),
            (BoxVariant::Multinum(l), BoxVariant::Multinum(r)) => BoxVariant::repack_raw(l / r),
            (l, r) => panic!("Type Error: Cannot divide {:?} with {:?}", l, r),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_div() {
        let dividend = BoxValue::from(6);
        let divisor = BoxValue::from(3);
        let quot = dividend / divisor;
        let exp = BoxValue::from(2);
        assert_eq!(quot, exp);

        let dividend = (-1) * BoxVariant::alpha() - BoxVariant::alpha() * BoxVariant::alpha()
            + BoxVariant::from(6);
        let divisor = BoxVariant::from(3) + BoxVariant::alpha();

        // let quot = dividend / divisor;
    }
}
