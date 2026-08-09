use malachite::base::num::arithmetic::traits::SaturatingSub;
use rapidhash::RapidHashSet;

use crate::{BoxContentKey, BoxType, BoxValue, BoxVariant, Color, mul::BoxMul};

/// Caret operation is the next operation after multiplication, one level up the hierarchy
/// It is defined as multiplication of child boxes
pub trait Caret<Rhs = Self> {
    type Output;
    fn caret(self, rhs: Rhs) -> Self::Output;
}

impl<L: BoxType + BoxMul<R>, R: BoxType> Caret<BoxValue<R>> for BoxValue<L> {
    type Output = BoxValue<L::Output>;

    /// Caret operation for two boxes
    fn caret(self, rhs: BoxValue<R>) -> Self::Output {
        let mut result = BoxValue::new();
        let mut unique_children: RapidHashSet<BoxContentKey> = RapidHashSet::default();

        let lhs_col = self.get_color(0);
        let rhs_col = rhs.get_color(0);

        let lhs_kind = self.get_kind(0);
        let rhs_kind = rhs.get_kind(0);

        let lhs_mul = self.get_multiplicity(0);
        let rhs_mul = rhs.get_multiplicity(0);

        for left_child in self {
            for right_child in rhs.clone() {
                let left_mul = left_child.get_multiplicity(0);
                let right_mul = right_child.get_multiplicity(0);
                let mul = left_mul * right_mul;

                let mut prod = left_child.clone() * right_child;
                prod.set_multiplicity(0, mul.clone());
                let col = prod.get_color(0);
                let key = BoxContentKey(prod);

                if let Some(BoxContentKey(mut other)) = unique_children.take(&key) {
                    let other_col = other.get_color(0);
                    let other_mul = other.get_multiplicity(0);
                    if col + other_col == Color::Red {
                        if mul < other_mul {
                            other.set_multiplicity(0, other_mul.saturating_sub(mul));
                        } else {
                            other.set_multiplicity(1, mul.saturating_sub(other_mul));
                            other.set_color(0, col);
                        }
                    } else {
                        other.set_multiplicity(0, other_mul + mul);
                    }
                    unique_children.insert(BoxContentKey(other));
                } else {
                    unique_children.insert(key);
                }
            }
        }

        result.kinds.push(lhs_kind * rhs_kind);
        result.colors.push(lhs_col * rhs_col);
        result.multiplicities.push(lhs_mul * rhs_mul);
        result.lengths.push(1);

        for key in unique_children {
            let raw_box = key.0;
            if raw_box.get_multiplicity(0) != 0 {
                result.extend(raw_box);
            }
        }

        result.sort_immediate_children();
        result
    }
}

impl Caret for BoxVariant {
    type Output = Self;

    fn caret(self, rhs: Self) -> Self::Output {
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
            (BoxVariant::Any(l), BoxVariant::Any(r)) => BoxVariant::repack_raw(l.caret(r)),
            (BoxVariant::Num(l), BoxVariant::Num(r)) => BoxVariant::repack_raw(l.caret(r)),
            (BoxVariant::Num(l), BoxVariant::Polynum(r)) => BoxVariant::repack_raw(l.caret(r)),
            (BoxVariant::Polynum(l), BoxVariant::Num(r)) => BoxVariant::repack_raw(l.caret(r)),
            (BoxVariant::Polynum(l), BoxVariant::Polynum(r)) => BoxVariant::repack_raw(l.caret(r)),
            (BoxVariant::Num(l), BoxVariant::Multinum(r)) => BoxVariant::repack_raw(l.caret(r)),
            (BoxVariant::Multinum(l), BoxVariant::Num(r)) => BoxVariant::repack_raw(l.caret(r)),
            (BoxVariant::Polynum(l), BoxVariant::Multinum(r)) => BoxVariant::repack_raw(l.caret(r)),
            (BoxVariant::Multinum(l), BoxVariant::Polynum(r)) => BoxVariant::repack_raw(l.caret(r)),
            (BoxVariant::Multinum(l), BoxVariant::Multinum(r)) => {
                BoxVariant::repack_raw(l.caret(r))
            }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_caret() {
        let a = BoxValue::from(2);
        let b = BoxValue::from(3);
        let exp = BoxValue::from(6);
        assert_eq!(a.caret(b), exp);

        let a = BoxValue::alpha().pow(2);
        let b = BoxValue::alpha().pow(3);
        let exp = BoxValue::alpha().pow(6);
        assert_eq!(a.caret(b), exp);

        let a = 5_u32 * BoxValue::alpha().pow(2);
        let b = 4_u32 * BoxValue::alpha().pow(3);
        let exp = 20_u32 * BoxValue::alpha().pow(6);
        assert_eq!(a.caret(b), exp);

        let a = BoxValue::alpha() + BoxValue::alpha().pow(2) + BoxValue::alpha().pow(3);
        let b = BoxValue::alpha().pow(2) + BoxValue::alpha().pow(4);
        let exp = BoxValue::alpha().pow(2)
            + 2 * BoxValue::alpha().pow(4)
            + BoxValue::alpha().pow(6)
            + BoxValue::alpha().pow(8)
            + BoxValue::alpha().pow(12);
        assert_eq!(a.caret(b), exp);

        let a = BoxValue::beta(2_u32);
        let b = BoxValue::beta(4_u32);
        let exp = BoxValue::beta(6_u32);
        assert_eq!(a.caret(b), exp);
    }
}
