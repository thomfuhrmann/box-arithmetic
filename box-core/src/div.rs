use std::ops::Div;

use malachite::{Natural, base::num::arithmetic::traits::SaturatingSub};
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

// TODO: return quotient and remainder
impl<L: BoxType + BoxDiv<R>, R: BoxType> Div<BoxValue<R>> for BoxValue<L> {
    type Output = BoxValue<L::Output>;

    /// Divide two boxes
    fn div(mut self, rhs: BoxValue<R>) -> Self::Output {
        if rhs.get_length(0) < 2 {
            panic!("Division by zero or invalid divisor: {:?}", rhs);
        }

        let lhs_col = self.get_color(0);
        let rhs_col = rhs.get_color(0);
        let lhs_kind = self.get_kind(0);
        let rhs_kind = rhs.get_kind(0);

        let mut result = BoxValue::new();
        let mut unique_children: RapidHashMap<u64, BoxValue<AnyBox>> = RapidHashMap::default();
        // let mut unique_children_rem: RapidHashMap<u64, BoxValue<AnyBox>> = RapidHashMap::default();

        let first_divisor_child = rhs.first_child();
        let first_divisor_child_mul = first_divisor_child.get_multiplicity(0);

        let rhs_cast = rhs.cast::<L>();

        while self.get_length(0) > 1 {
            // safe since length of self > 1
            let dividend = self.first_child();

            let dividend_mul = dividend.get_multiplicity(0);
            if dividend_mul < first_divisor_child_mul {
                // Unmatched term -> drop / pass to remainder
                self = self - dividend.wrap::<L>(1_u32);
                // TODO: add to remainder
                continue;
            }

            // balance first level
            let mul = dividend_mul / first_divisor_child_mul.clone();

            let mut factor = dividend.clone();
            factor.set_multiplicity(0, mul);
            factor.set_color(0, dividend.get_color(0));

            // balance second level
            let mut has_match = true;
            for divisor_child in first_divisor_child.clone().into_iter() {
                has_match = false;

                let divisor_child_mul = divisor_child.get_multiplicity(0);
                let divisor_child_hash = divisor_child.hash_content(unique_children.hasher());

                let mut factor_kind = factor.get_kind(0);
                let factor_col = factor.get_color(0);
                let factor_mul = factor.get_multiplicity(0);
                // TODO: mutable iterator -> impl iter_mut for BoxValue
                factor = factor
                    .clone()
                    .into_iter()
                    .map(|mut fact| {
                        if divisor_child_hash == fact.hash_content(unique_children.hasher())
                            && divisor_child.is_eq_content(&fact)
                        {
                            let dividend_child_mul = fact.get_multiplicity(0);
                            let mut mul = dividend_child_mul.clone() / divisor_child_mul.clone();
                            // reduce by one because exponent is additive not multiplicative
                            mul = mul.saturating_sub(Natural::from(1_u32));
                            fact.set_multiplicity(0, mul);

                            has_match = true;
                            fact
                        } else {
                            fact
                        }
                    })
                    .collect();

                // reset outer values after collect
                if factor.get_length(0) == 1 {
                    factor_kind = BoxKind::Empty;
                }
                factor.set_kind(0, factor_kind);
                factor.set_color(0, factor_col);
                factor.set_multiplicity(0, factor_mul);

                if !has_match {
                    break;
                }
            }

            if !has_match {
                // could not find a matching child - remove dividend and continue
                self = self - dividend.clone().wrap::<L>(1_u32);
                // TODO: add to remainder
                continue;
            }

            let struct_hash = factor.hash_content(unique_children.hasher());
            unique_children.insert(struct_hash, factor.clone().cast());

            // subtract
            self = self - factor.wrap::<L>(1_u32) * rhs_cast.clone();
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

        let dividend =
            BoxVariant::from(6) - BoxVariant::alpha() - BoxVariant::alpha() * BoxVariant::alpha();
        let divisor = BoxVariant::from(3) + BoxVariant::alpha();
        let quot = dividend / divisor;
        let exp = BoxVariant::from(2) - BoxVariant::alpha();
        assert_eq!(quot, exp);

        let dividend = 2 * BoxVariant::alpha() * BoxVariant::alpha();
        let divisor = BoxVariant::alpha();
        let quot = dividend / divisor;
        let exp = 2 * BoxVariant::alpha();
        assert_eq!(quot, exp);
    }

    #[test]
    fn test_div_rem() {
        let dividend =
            BoxVariant::from(6) - BoxVariant::alpha() - BoxVariant::alpha() * BoxVariant::alpha();
        let divisor = BoxVariant::alpha();
        let quot = dividend / divisor;
        let exp = -BoxVariant::from(1) - BoxVariant::alpha();
        assert_eq!(quot, exp);
    }
}
