use malachite::Natural;
use rapidhash::{HashMapExt, RapidHashMap};

use crate::{AnyBox, BoxKind, BoxValue, SetBox};

impl BoxValue<AnyBox> {
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
    pub fn support(&self) -> BoxValue<SetBox> {
        let mut result = BoxValue::<SetBox>::new();
        for mut child in self.clone() {
            child.set_multiplicity(0, 1_u32);
            result.extend(child);
        }
        result
    }

    /// Set union of two boxes
    pub fn union(left: &BoxValue<AnyBox>, right: &BoxValue<AnyBox>) -> BoxValue<AnyBox> {
        let mut unique_children: RapidHashMap<u64, BoxValue<AnyBox>> = RapidHashMap::new();
        for left_child in left.clone() {
            let hash = left_child.hash_content(unique_children.hasher());
            if let Some(other) = unique_children.get_mut(&hash)
                && left.is_eq_content(other)
                && left_child.get_color(0) == other.get_color(0)
            {
                let left_mul = left_child.get_multiplicity(0);
                let other_mul = other.get_multiplicity(0);
                other.set_multiplicity(0, left_mul.max(other_mul));
            } else {
                unique_children.insert(hash, left_child);
            }
        }
        for right_child in right.clone() {
            let hash = right_child.hash_content(unique_children.hasher());
            if let Some(other) = unique_children.get_mut(&hash)
                && left.is_eq_content(other)
                && right_child.get_color(0) == other.get_color(0)
            {
                let left_mult = right_child.get_multiplicity(0);
                let other_mult = other.get_multiplicity(0);
                other.set_multiplicity(0, left_mult.max(other_mult));
            } else {
                unique_children.insert(hash, right_child);
            }
        }

        let mut result = BoxValue::<AnyBox>::new();
        let color = left.get_color(0) + right.get_color(0);
        result.kinds.push(BoxKind::Any);
        result.colors.push(color);
        result.multiplicities.push(Natural::from(1_u32));
        result.lengths.push(1);
        for (_, child) in unique_children.into_iter() {
            result.extend(child);
        }
        result.sort_immediate_children();
        result
    }

    /// Set intersection of two boxes
    pub fn intersection(left: &BoxValue<AnyBox>, right: &BoxValue<AnyBox>) -> BoxValue<AnyBox> {
        let mut left_unique: RapidHashMap<u64, BoxValue<AnyBox>> = RapidHashMap::new();
        for left_child in left.clone() {
            let hash = left_child.hash_content(left_unique.hasher());
            left_unique.insert(hash, left_child);
        }

        let mut right_unique: RapidHashMap<u64, BoxValue<AnyBox>> = RapidHashMap::new();
        for right_child in right.clone() {
            // use the same hasher as for the other left map
            let hash = right_child.hash_content(left_unique.hasher());
            right_unique.insert(hash, right_child);
        }

        let mut result = BoxValue::<AnyBox>::new();
        let color = left.get_color(0) + right.get_color(0);
        result.kinds.push(BoxKind::Any);
        result.colors.push(color);
        result.multiplicities.push(Natural::from(1_u32));
        result.lengths.push(1);

        for (left_hash, mut left_child) in left_unique.into_iter() {
            if let Some(right_child) = right_unique.get_mut(&left_hash)
                && right_child.is_eq_content(&left_child)
                && right_child.get_color(0) == left_child.get_color(0)
            {
                let right_mult = right_child.get_multiplicity(0);
                let left_mult = left_child.get_multiplicity(0);
                left_child.set_multiplicity(0, left_mult.min(right_mult));
                result.extend(left_child);
            }
        }
        result.sort_immediate_children();
        result
    }
}

#[cfg(test)]
mod tests {

    use crate::BoxValue;

    #[test]
    fn test_set_ops() {
        let mut m = BoxValue::empty();
        m.extend_with_mul(BoxValue::from(1), 4_u32);
        m.extend_with_mul(BoxValue::from(2), 2_u32);
        m.extend_with_mul(BoxValue::from(3), 1_u32);

        let mut n = BoxValue::empty();
        n.extend_with_mul(BoxValue::from(1), 7_u32);
        n.extend_with_mul(BoxValue::from(3), 3_u32);
        n.extend(BoxValue::from(4));

        let union = BoxValue::union(&m, &n);

        let mut exp = BoxValue::empty();
        exp.extend_with_mul(BoxValue::from(1), 7_u32);
        exp.extend_with_mul(BoxValue::from(2), 2_u32);
        exp.extend_with_mul(BoxValue::from(3), 3_u32);
        exp.extend_with_mul(BoxValue::from(4), 1_u32);
        exp.sort_immediate_children();

        assert_eq!(union, exp);

        let intersection = BoxValue::intersection(&m, &n);

        let mut exp = BoxValue::empty();
        exp.extend_with_mul(BoxValue::from(1), 4_u32);
        exp.extend_with_mul(BoxValue::from(3), 1_u32);
        exp.sort_immediate_children();

        assert_eq!(intersection, exp);
    }
}
