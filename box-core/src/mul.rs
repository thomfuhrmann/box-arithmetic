use std::ops::Mul;

use malachite::{Natural, base::num::arithmetic::traits::SaturatingSub};
use rapidhash::RapidHashMap;

use crate::{AnyBox, BoxType, BoxValue, BoxVariant, Color, MultinumBox, NumBox, PolynumBox};

/// Trait for the output type of box multiplication
pub trait BoxMul<Rhs = Self> {
    type Output: BoxType;
}

impl<T: BoxType> BoxMul for T {
    type Output = Self;
}

macro_rules! impl_box_mul {
    ($lhs:ty, $rhs:ty => $out:ty) => {
        impl BoxMul<$rhs> for $lhs {
            type Output = $out;
        }
        impl BoxMul<$lhs> for $rhs {
            type Output = $out;
        }
    };
}

impl_box_mul!(NumBox, PolynumBox => PolynumBox);
impl_box_mul!(NumBox, MultinumBox => MultinumBox);
impl_box_mul!(PolynumBox, MultinumBox => MultinumBox);

impl<L: BoxType + BoxMul<R>, R: BoxType> Mul<BoxValue<R>> for BoxValue<L> {
    type Output = BoxValue<L::Output>;

    /// Multiply two boxes
    fn mul(self, rhs: BoxValue<R>) -> Self::Output {
        let mut result = BoxValue::new();
        let mut unique_children: RapidHashMap<u64, BoxValue<AnyBox>> = RapidHashMap::default();

        let lhs_col = self.get_color(0);
        let rhs_col = rhs.get_color(0);

        let lhs_kind = self.get_kind(0);
        let rhs_kind = rhs.get_kind(0);

        for left_child in self {
            for right_child in rhs.clone() {
                let left_mul = left_child.get_multiplicity(0);
                let right_mul = right_child.get_multiplicity(0);
                let mul = left_mul * right_mul;

                let mut box_sum = left_child.clone() + right_child;

                let col = box_sum.get_color(0);
                let struct_hash = box_sum.hash_content(unique_children.hasher());

                if let Some(other) = unique_children.get_mut(&struct_hash)
                    && box_sum.is_eq_content(other)
                {
                    let other_col = other.get_color(0);
                    let other_mul = other.get_multiplicity(0);
                    if col + other_col == Color::Red {
                        if mul < other_mul {
                            other.set_multiplicity(0, other_mul.saturating_sub(mul));
                        } else {
                            other.set_multiplicity(0, mul.saturating_sub(other_mul));
                            other.set_color(0, col);
                        }
                    } else {
                        other.set_multiplicity(0, other_mul + mul);
                    }
                } else {
                    box_sum.set_multiplicity(0, mul);
                    unique_children.insert(struct_hash, box_sum);
                }
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

impl<L: BoxType + BoxMul<R>, R: BoxType> Mul<&BoxValue<R>> for &BoxValue<L> {
    type Output = BoxValue<L::Output>;

    fn mul(self, rhs: &BoxValue<R>) -> Self::Output {
        self.clone() * rhs.clone()
    }
}

impl<'a, L: BoxType + BoxMul<R>, R: BoxType> Mul<&'a BoxValue<R>> for BoxValue<L> {
    type Output = BoxValue<L::Output>;
    fn mul(self, rhs: &'a BoxValue<R>) -> Self::Output {
        &self * rhs
    }
}

impl<L: BoxType + BoxMul<R>, R: BoxType> Mul<BoxValue<R>> for &BoxValue<L> {
    type Output = BoxValue<L::Output>;
    fn mul(self, rhs: BoxValue<R>) -> Self::Output {
        self * &rhs
    }
}

impl<T: BoxType + BoxMul<T>> Mul<BoxValue<T>> for u32 {
    type Output = BoxValue<T::Output>;

    #[inline]
    fn mul(self, rhs: BoxValue<T>) -> Self::Output {
        BoxValue::from(self).cast::<T>() * rhs
    }
}

impl<T: BoxType + BoxMul<T>> Mul<u32> for BoxValue<T> {
    type Output = BoxValue<T::Output>;

    #[inline]
    fn mul(self, rhs: u32) -> Self::Output {
        self * BoxValue::from(rhs).cast::<T>()
    }
}

impl<T: BoxType + BoxMul<T>> Mul<BoxValue<T>> for u64 {
    type Output = BoxValue<T::Output>;

    #[inline]
    fn mul(self, rhs: BoxValue<T>) -> Self::Output {
        BoxValue::from(self).cast::<T>() * rhs
    }
}

impl<T: BoxType + BoxMul<T>> Mul<u64> for BoxValue<T> {
    type Output = BoxValue<T::Output>;

    #[inline]
    fn mul(self, rhs: u64) -> Self::Output {
        self * BoxValue::from(rhs).cast::<T>()
    }
}

impl<T: BoxType + BoxMul<T>> Mul<BoxValue<T>> for i32 {
    type Output = BoxValue<T::Output>;

    #[inline]
    fn mul(self, rhs: BoxValue<T>) -> Self::Output {
        BoxValue::from(self).cast::<T>() * rhs
    }
}

impl<T: BoxType + BoxMul<T>> Mul<i32> for BoxValue<T> {
    type Output = BoxValue<T::Output>;

    #[inline]
    fn mul(self, rhs: i32) -> Self::Output {
        self * BoxValue::from(rhs).cast::<T>()
    }
}

impl<T: BoxType + BoxMul<T>> Mul<BoxValue<T>> for i64 {
    type Output = BoxValue<T::Output>;

    #[inline]
    fn mul(self, rhs: BoxValue<T>) -> Self::Output {
        BoxValue::from(self).cast::<T>() * rhs
    }
}

impl<T: BoxType + BoxMul<T>> Mul<i64> for BoxValue<T> {
    type Output = BoxValue<T::Output>;

    #[inline]
    fn mul(self, rhs: i64) -> Self::Output {
        self * BoxValue::from(rhs).cast::<T>()
    }
}

impl Mul for BoxVariant {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (BoxVariant::Empty(l), r) => {
                let l_col = l.get_color(0);
                let r_col = r.get_color(0);
                match l_col * r_col {
                    Color::Black => BoxValue::zero().into(),
                    Color::Red => BoxValue::anti_zero().into(),
                }
            }
            (l, BoxVariant::Empty(r)) => {
                let l_col = l.get_color(0);
                let r_col = r.get_color(0);
                match l_col * r_col {
                    Color::Black => BoxValue::zero().into(),
                    Color::Red => BoxValue::anti_zero().into(),
                }
            }
            (BoxVariant::Num(l), BoxVariant::Num(r)) => BoxVariant::repack_raw(l * r),
            (BoxVariant::Num(l), BoxVariant::Polynum(r)) => BoxVariant::repack_raw(l * r),
            (BoxVariant::Polynum(l), BoxVariant::Num(r)) => BoxVariant::repack_raw(l * r),
            (BoxVariant::Polynum(l), BoxVariant::Polynum(r)) => BoxVariant::repack_raw(l * r),
            (BoxVariant::Num(l), BoxVariant::Multinum(r)) => BoxVariant::repack_raw(l * r),
            (BoxVariant::Multinum(l), BoxVariant::Num(r)) => BoxVariant::repack_raw(l * r),
            (BoxVariant::Polynum(l), BoxVariant::Multinum(r)) => BoxVariant::repack_raw(l * r),
            (BoxVariant::Multinum(l), BoxVariant::Polynum(r)) => BoxVariant::repack_raw(l * r),
            (BoxVariant::Multinum(l), BoxVariant::Multinum(r)) => BoxVariant::repack_raw(l * r),
            (BoxVariant::Maxel(l), BoxVariant::Vexel(r)) => {
                BoxVariant::repack_raw(BoxValue::mul_max_vex(l, r))
            }
            (BoxVariant::Maxel(l), BoxVariant::Maxel(r)) => {
                BoxVariant::repack_raw(BoxValue::mul_max(l, r))
            }
            (l, r) => panic!("Type Error: Cannot multiply {:?} with {:?}", l, r),
        }
    }
}

impl Mul<BoxVariant> for u32 {
    type Output = BoxVariant;

    #[inline]
    fn mul(self, rhs: BoxVariant) -> Self::Output {
        BoxVariant::from(self) * rhs
    }
}

impl Mul<u32> for BoxVariant {
    type Output = BoxVariant;

    #[inline]
    fn mul(self, rhs: u32) -> Self::Output {
        self * BoxVariant::from(rhs)
    }
}

impl Mul<BoxVariant> for u64 {
    type Output = BoxVariant;

    #[inline]
    fn mul(self, rhs: BoxVariant) -> Self::Output {
        BoxVariant::from(self) * rhs
    }
}

impl Mul<u64> for BoxVariant {
    type Output = BoxVariant;

    #[inline]
    fn mul(self, rhs: u64) -> Self::Output {
        self * BoxVariant::from(rhs)
    }
}

impl Mul<BoxVariant> for i32 {
    type Output = BoxVariant;

    #[inline]
    fn mul(self, rhs: BoxVariant) -> Self::Output {
        BoxVariant::from(self) * rhs
    }
}

impl Mul<i32> for BoxVariant {
    type Output = BoxVariant;

    #[inline]
    fn mul(self, rhs: i32) -> Self::Output {
        self * BoxVariant::from(rhs)
    }
}

impl Mul<BoxVariant> for i64 {
    type Output = BoxVariant;

    #[inline]
    fn mul(self, rhs: BoxVariant) -> Self::Output {
        BoxVariant::from(self) * rhs
    }
}

impl Mul<i64> for BoxVariant {
    type Output = BoxVariant;

    #[inline]
    fn mul(self, rhs: i64) -> Self::Output {
        self * BoxVariant::from(rhs)
    }
}

#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn test_mul() {
        let prod = BoxVariant::from(2) * BoxVariant::from(3);
        let expected = BoxVariant::from(6);
        assert_eq!(prod, expected);

        let expected = BoxVariant::from(3) * BoxVariant::from(2);
        assert_eq!(prod, expected);

        let prod = BoxVariant::from(2) * BoxVariant::from(-3);
        let expected = BoxVariant::from(-6);
        assert_eq!(prod, expected);

        let prod = 2 * BoxVariant::from(-3);
        let expected = BoxVariant::from(-6);
        assert_eq!(prod, expected);

        let prod = BoxVariant::alpha() * BoxVariant::alpha();
        let expected = BoxVariant::from(2).wrap::<PolynumBox>(1_u32);
        assert_eq!(prod, expected);

        let s1 = BoxVariant::one() + BoxVariant::alpha();
        let minus_alpha = (-1) * BoxVariant::alpha();
        let s2 = BoxVariant::one() + minus_alpha;
        let prod = s1 * s2;
        let expected = BoxVariant::from(1) + (-1) * BoxVariant::alpha() * BoxVariant::alpha();
        assert_eq!(prod, expected);
    }
}
