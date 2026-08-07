use malachite::Natural;
use rapidhash::RapidHashSet;

use crate::{AnyBox, BoxContentKey, BoxKind, BoxOrder, BoxValue, BoxVariant, Color, SetBox};

impl BoxValue<SetBox> {
    /// Construct an empty set with a given color
    pub fn empty_set(col: Color) -> Self {
        BoxValue {
            kinds: vec![BoxKind::Set],
            colors: vec![col],
            multiplicities: vec![Natural::from(1_u32)],
            lengths: vec![1],
            _marker: std::marker::PhantomData,
        }
    }
}

impl From<Vec<BoxValue<AnyBox>>> for BoxValue<SetBox> {
    fn from(value: Vec<BoxValue<AnyBox>>) -> Self {
        let mut result = BoxValue::new();
        result.kinds.push(BoxKind::Set);
        result.colors.push(Color::Black);
        result.multiplicities.push(malachite::Natural::from(1_u32));
        result.lengths.push(1);
        for any_box in value {
            result.extend(any_box);
        }
        result
    }
}

impl BoxVariant {
    pub fn union(self, rhs: Self) -> Self {
        match (self, rhs) {
            (BoxVariant::Set(l), BoxVariant::Set(r)) => BoxVariant::Set(BoxValue::union(l, r)),
            (l, r) => panic!("Type Error: Cannot compute union of {:?} and {:?}", l, r),
        }
    }

    pub fn intersection(self, rhs: Self) -> Self {
        match (self, rhs) {
            (BoxVariant::Set(l), BoxVariant::Set(r)) => {
                BoxVariant::Set(BoxValue::intersection(l, r))
            }
            (l, r) => panic!(
                "Type Error: Cannot compute intersection of {:?} and {:?}",
                l, r
            ),
        }
    }
}

impl BoxValue<SetBox> {
    /// A set is a box with all its elements having multiplicity one
    pub fn is_set(&self) -> bool {
        for child in self {
            let mult = child.multiplicities[0].clone();
            if mult != 1 {
                return false;
            }
        }
        true
    }

    /// Creates the supporting set of a box consisting of all its elements but with multiplicity one
    pub fn support(self) -> Self {
        let mut result = BoxValue::<SetBox>::new();
        for mut child in self {
            child.set_multiplicity(0, 1_u32);
            result.extend(child);
        }
        result
    }

    /// Set union of two boxes
    pub fn union(left: Self, right: Self) -> Self {
        let mut unique_children: RapidHashSet<BoxContentKey> = RapidHashSet::default();
        let color = left.get_color(0) + right.get_color(0);

        for left_child in left {
            let key = BoxContentKey(left_child);

            if let Some(mut existing) = unique_children.take(&key) {
                let left_mul = key.0.get_multiplicity(0);
                let existing_mul = existing.0.get_multiplicity(0);
                existing.0.set_multiplicity(0, left_mul.max(existing_mul));

                unique_children.insert(existing);
            } else {
                unique_children.insert(key);
            }
        }

        for right_child in right {
            let key = BoxContentKey(right_child);

            if let Some(mut existing) = unique_children.take(&key) {
                let left_mul = key.0.get_multiplicity(0);
                let existing_mul = existing.0.get_multiplicity(0);
                existing.0.set_multiplicity(0, left_mul.max(existing_mul));

                unique_children.insert(existing);
            } else {
                unique_children.insert(key);
            }
        }

        let mut result = BoxValue::empty_set(color);
        for key in unique_children {
            let child = key.0;
            result.extend(child);
        }
        result.sort_immediate_children(BoxOrder::Lex);
        result
    }

    /// Set intersection of two boxes
    pub fn intersection(left: Self, right: Self) -> Self {
        let mut left_unique: RapidHashSet<BoxContentKey> = RapidHashSet::default();
        let color = left.get_color(0) + right.get_color(0);

        for left_child in left {
            left_unique.insert(BoxContentKey(left_child));
        }

        let mut right_unique: RapidHashSet<BoxContentKey> = RapidHashSet::default();
        for right_child in right {
            // use the same hasher as for the other left map
            right_unique.insert(BoxContentKey(right_child));
        }

        let mut result = BoxValue::empty_set(color);

        for key in left_unique {
            if let Some(BoxContentKey(right_child)) = right_unique.take(&key) {
                let mut left_child = key.0;
                if left_child.get_color(0) == right_child.get_color(0) {
                    let right_mult = right_child.get_multiplicity(0);
                    let left_mult = left_child.get_multiplicity(0);
                    left_child.set_multiplicity(0, left_mult.min(right_mult));
                    result.extend(left_child);
                }
            }
        }
        result.sort_immediate_children(BoxOrder::Lex);
        result
    }
}

#[cfg(test)]
mod tests {

    use crate::BoxValue;

    #[test]
    fn test_set_ops() {
        let mut m = BoxValue::empty_set(crate::Color::Black);
        m.extend_with_mul(BoxValue::from(1), 4_u32);
        m.extend_with_mul(BoxValue::from(2), 2_u32);
        m.extend_with_mul(BoxValue::from(3), 1_u32);

        let mut n = BoxValue::empty_set(crate::Color::Black);
        n.extend_with_mul(BoxValue::from(1), 7_u32);
        n.extend_with_mul(BoxValue::from(3), 3_u32);
        n.extend(BoxValue::from(4));

        let union = BoxValue::union(m.clone(), n.clone());

        let mut exp = BoxValue::empty_set(crate::Color::Black);
        exp.extend_with_mul(BoxValue::from(1), 7_u32);
        exp.extend_with_mul(BoxValue::from(2), 2_u32);
        exp.extend_with_mul(BoxValue::from(3), 3_u32);
        exp.extend_with_mul(BoxValue::from(4), 1_u32);
        exp.sort_immediate_children(crate::BoxOrder::Lex);

        assert_eq!(union, exp.cast());

        let intersection = BoxValue::intersection(m, n);

        let mut exp = BoxValue::empty_set(crate::Color::Black);
        exp.extend_with_mul(BoxValue::from(1), 4_u32);
        exp.extend_with_mul(BoxValue::from(3), 1_u32);
        exp.sort_immediate_children(crate::BoxOrder::Lex);

        assert_eq!(intersection, exp);
    }
}
