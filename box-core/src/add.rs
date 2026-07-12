use std::ops::{Add, Sub};

use malachite::{Natural, base::num::arithmetic::traits::SaturatingSub};
use rapidhash::RapidHashMap;

use crate::{
    AnyBox, BoxKind, BoxType, BoxValue, BoxVariant, Color, MultinumBox, NumBox, PolynumBox,
};

/// Trait for the output type of box addition
pub trait BoxAdd<Rhs = Self> {
    type Output: BoxType;
}

impl<T: BoxType> BoxAdd for T {
    type Output = Self;
}

macro_rules! impl_box_add {
    ($lhs:ty, $rhs:ty => $out:ty) => {
        impl BoxAdd<$rhs> for $lhs {
            type Output = $out;
        }
        impl BoxAdd<$lhs> for $rhs {
            type Output = $out;
        }
    };
}

impl_box_add!(NumBox, PolynumBox => PolynumBox);
impl_box_add!(NumBox, MultinumBox => MultinumBox);
impl_box_add!(PolynumBox, MultinumBox => MultinumBox);

impl Add for BoxKind {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (BoxKind::Empty, r) => r,
            (l, BoxKind::Empty) => l,
            (BoxKind::Num, BoxKind::Num) => BoxKind::Num,
            (BoxKind::Num, BoxKind::Polynum) => BoxKind::Polynum,
            (BoxKind::Polynum, BoxKind::Num) => BoxKind::Polynum,
            (BoxKind::Polynum, BoxKind::Polynum) => BoxKind::Polynum,
            (BoxKind::Polynum, BoxKind::Multinum) => BoxKind::Multinum,
            (BoxKind::Multinum, BoxKind::Polynum) => BoxKind::Multinum,
            (BoxKind::Multinum, BoxKind::Multinum) => BoxKind::Multinum,
            (BoxKind::Vexel, BoxKind::Vexel) => BoxKind::Vexel,
            (BoxKind::Maxel, BoxKind::Maxel) => BoxKind::Maxel,
            (_, _) => BoxKind::Any,
        }
    }
}

impl BoxKind {
    pub fn get_kind_from_depth(depth: u32) -> BoxKind {
        match depth {
            0 => BoxKind::Empty,
            1 => BoxKind::Num,
            2 => BoxKind::Polynum,
            3 => BoxKind::Multinum,
            _ => BoxKind::Any,
        }
    }
}

impl<T: BoxType> BoxValue<T> {
    fn add_child_boxes(self, unique_children: &mut RapidHashMap<u64, BoxValue<AnyBox>>) {
        for child in self {
            let child_col = child.get_color(0);
            let child_mul = child.get_multiplicity(0);

            let hash = child.hash_content(unique_children.hasher());

            if let Some(other) = unique_children.get_mut(&hash)
                && child.is_eq_content(other)
            {
                let other_col = other.get_color(0);
                let other_mul = other.get_multiplicity(0);

                if child_col + other_col == Color::Red {
                    if child_mul < other_mul {
                        other.set_multiplicity(0, other_mul.saturating_sub(child_mul));
                    } else {
                        other.set_multiplicity(0, child_mul.saturating_sub(other_mul));
                        other.set_color(0, child_col);
                    }
                } else {
                    other.set_multiplicity(0, other_mul + child_mul);
                }
            } else {
                unique_children.insert(hash, child);
            }
        }
    }
}

impl<L: BoxType + BoxAdd<R>, R: BoxType> Add<BoxValue<R>> for BoxValue<L> {
    type Output = BoxValue<L::Output>;

    fn add(self, rhs: BoxValue<R>) -> Self::Output {
        let lhs_col = self.get_color(0);
        let rhs_col = rhs.get_color(0);

        let lhs_kind = self.get_kind(0);
        let rhs_kind = rhs.get_kind(0);

        let mut result = BoxValue::<L::Output>::new();
        result.kinds.push(BoxKind::Any);
        result.colors.push(lhs_col + rhs_col);
        result.multiplicities.push(Natural::from(1_u32));
        result.lengths.push(1);

        let mut unique_children: RapidHashMap<u64, BoxValue<AnyBox>> = RapidHashMap::default();
        self.add_child_boxes(&mut unique_children);
        rhs.add_child_boxes(&mut unique_children);

        let mut max_depth: u32 = 0;
        for child in unique_children.into_values() {
            let mult = child.get_multiplicity(0);
            if mult == 0 {
                continue;
            }

            let child_len = child.get_length(0);
            max_depth = max_depth.max(child_len);
            result.extend(child);
        }

        let kind = L::Output::KIND;
        let new_kind =
            if kind == BoxKind::Num || kind == BoxKind::Polynum || kind == BoxKind::Multinum {
                BoxKind::get_kind_from_depth(max_depth)
            } else {
                lhs_kind + rhs_kind
            };

        // set kind of box based on final result
        result.kinds[0] = new_kind;

        result.sort_immediate_children();
        result
    }
}

impl<L: BoxType + BoxAdd<R>, R: BoxType> Add<&BoxValue<R>> for &BoxValue<L> {
    type Output = BoxValue<L::Output>;

    fn add(self, rhs: &BoxValue<R>) -> Self::Output {
        self.clone() + rhs.clone()
    }
}

impl<'a, L: BoxType + BoxAdd<R>, R: BoxType> Add<&'a BoxValue<R>> for BoxValue<L> {
    type Output = BoxValue<L::Output>;
    fn add(self, rhs: &'a BoxValue<R>) -> Self::Output {
        &self + rhs
    }
}

impl<L: BoxType + BoxAdd<R>, R: BoxType> Add<BoxValue<R>> for &BoxValue<L> {
    type Output = BoxValue<L::Output>;
    fn add(self, rhs: BoxValue<R>) -> Self::Output {
        self + &rhs
    }
}

impl Add for BoxVariant {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (BoxVariant::Empty(l), mut r) => {
                let l_col = l.get_color(0);
                let r_col = r.get_color(0);
                match l_col + r_col {
                    Color::Black => {
                        r.set_color(0, Color::Black);
                    }
                    Color::Red => {
                        r.set_color(0, Color::Red);
                    }
                }
                r
            }
            (mut l, BoxVariant::Empty(r)) => {
                let l_col = l.get_color(0);
                let r_col = r.get_color(0);
                match l_col + r_col {
                    Color::Black => {
                        l.set_color(0, Color::Black);
                    }
                    Color::Red => {
                        l.set_color(0, Color::Red);
                    }
                }
                l
            }
            (BoxVariant::Num(l), BoxVariant::Num(r)) => BoxVariant::repack_raw(l + r),
            (BoxVariant::Num(l), BoxVariant::Polynum(r)) => BoxVariant::repack_raw(l + r),
            (BoxVariant::Polynum(l), BoxVariant::Num(r)) => BoxVariant::repack_raw(l + r),
            (BoxVariant::Polynum(l), BoxVariant::Polynum(r)) => BoxVariant::repack_raw(l + r),
            (BoxVariant::Num(l), BoxVariant::Multinum(r)) => BoxVariant::repack_raw(l + r),
            (BoxVariant::Multinum(l), BoxVariant::Num(r)) => BoxVariant::repack_raw(l + r),
            (BoxVariant::Polynum(l), BoxVariant::Multinum(r)) => BoxVariant::repack_raw(l + r),
            (BoxVariant::Multinum(l), BoxVariant::Polynum(r)) => BoxVariant::repack_raw(l + r),
            (BoxVariant::Multinum(l), BoxVariant::Multinum(r)) => BoxVariant::repack_raw(l + r),
            (BoxVariant::Vexel(l), BoxVariant::Vexel(r)) => BoxVariant::repack_raw(l + r),
            (BoxVariant::Maxel(l), BoxVariant::Maxel(r)) => BoxVariant::repack_raw(l + r),
            (l, r) => panic!("Type Error: Cannot add {:?} to {:?}", l, r),
        }
    }
}

impl Sub for BoxVariant {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-1) * rhs
    }
}

#[cfg(test)]
mod tests {

    use crate::BoxVariant;

    #[test]
    fn test_add() {
        let left = BoxVariant::from(3);
        let right = BoxVariant::from(5);
        let sum = left + right;
        let exp = BoxVariant::from(8);
        assert_eq!(sum, exp);

        let left = BoxVariant::from(-3);
        let right = BoxVariant::from(5);
        let sum = left + right;
        let exp = BoxVariant::from(2);
        assert_eq!(sum, exp);

        let left = BoxVariant::from(-3);
        let right = BoxVariant::from(3);
        let sum = left + right;
        let exp = BoxVariant::from(0);
        assert_eq!(sum, exp);

        let left = BoxVariant::anti_zero();
        let right = BoxVariant::from(3);
        let sum = left + right;
        let exp = BoxVariant::from(3).into_anti();
        assert_eq!(sum, exp);

        let sum = (BoxVariant::from(1) + BoxVariant::alpha())
            - (BoxVariant::from(1) + BoxVariant::alpha());
        let exp = BoxVariant::zero();
        assert_eq!(sum, exp);

        let sum = (BoxVariant::from(1) + BoxVariant::alpha()) - BoxVariant::alpha();
        let exp = BoxVariant::from(1);
        assert_eq!(sum, exp);
    }
}
