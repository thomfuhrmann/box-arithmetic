use std::hash::BuildHasher;

use crate::{AnyBox, BoxValue, BoxVariant, MaxelBox, PixelBox};
use rapidhash::RapidHashSet;

impl BoxValue<MaxelBox> {
    /// Return the domain of a function
    pub fn domain(&self) -> Vec<BoxValue<AnyBox>> {
        let mut result = Vec::new();
        for child in self.clone() {
            let x = child.cast::<PixelBox>().x();
            result.push(x);
        }
        result
    }

    /// Return the range of a function
    pub fn range(&self) -> Vec<BoxValue<AnyBox>> {
        let mut result = Vec::new();
        for child in self.clone() {
            let y = child.cast::<PixelBox>().y();
            result.push(y);
        }
        result
    }

    /// Test if the box is a function
    pub fn is_function(&self) -> bool {
        let mut unique_children = RapidHashSet::default();
        let mut num_children = 0;
        for child in self.clone() {
            num_children += 1;
            let x = child.cast::<PixelBox>().x();
            let hash = unique_children.hasher().hash_one(x);
            unique_children.insert(hash);
        }
        unique_children.len() == num_children
    }

    /// Test if the function is bijective
    pub fn is_bijective(&self) -> bool {
        self.domain().len() == self.range().len()
    }
}

impl BoxVariant {
    pub fn is_function(&self) -> bool {
        match self {
            BoxVariant::Maxel(m) => m.is_function(),
            _ => false,
        }
    }

    pub fn is_bijective(&self) -> bool {
        self.domain().len() == self.range().len()
    }

    pub fn domain(&self) -> Vec<BoxValue<AnyBox>> {
        match self {
            BoxVariant::Maxel(m) => m.domain(),
            _ => Vec::new(),
        }
    }

    pub fn range(&self) -> Vec<BoxValue<AnyBox>> {
        match self {
            BoxVariant::Maxel(m) => m.range(),
            _ => Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{AnyBox, BoxValue, maxel};

    #[test]
    fn test_fn_1() {
        let poly = BoxValue::from(6) + BoxValue::alpha() * BoxValue::alpha();
        let f_box = maxel![[[0, 3], [BoxValue::alpha(), 1], [3, 1], [2, poly], [4, 5]]];
        assert!(f_box.is_function());

        let dom = vec![
            BoxValue::from(0).cast::<AnyBox>(),
            BoxValue::from(3).cast::<AnyBox>(),
            BoxValue::from(4).cast::<AnyBox>(),
            BoxValue::from(2).cast::<AnyBox>(),
            BoxValue::alpha().cast::<AnyBox>(),
        ];
        assert_eq!(f_box.domain(), dom);

        let poly = BoxValue::from(6) + BoxValue::alpha() * BoxValue::alpha();
        let range = vec![
            BoxValue::from(3).cast::<AnyBox>(),
            BoxValue::from(1).cast::<AnyBox>(),
            BoxValue::from(5).cast::<AnyBox>(),
            poly.cast::<AnyBox>(),
            BoxValue::from(1).cast::<AnyBox>(),
        ];
        assert_eq!(f_box.range(), range);

        let g_box = maxel![[[0, 0], [3, 8], [1, 8]]];
        let prod = f_box * g_box;
        let exp = maxel![[[0, 8], [BoxValue::alpha(), 8], [3, 8]]];
        assert_eq!(prod, exp);
    }
}
