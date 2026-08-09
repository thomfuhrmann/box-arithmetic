use malachite::Natural;
use serde::{Deserialize, Serialize};

use crate::{BoxType, BoxValue, BoxVariant, Color};

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct TreeNode {
    pub color: Color,
    #[serde(with = "serde_natural_decimal")]
    pub multiplicity: Natural,
    pub children: Vec<TreeNode>,
}

pub mod serde_natural_decimal {
    use malachite::Natural;
    use serde::{Deserialize, Deserializer, Serializer};
    use std::str::FromStr;

    pub fn serialize<S>(val: &Natural, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&val.to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Natural, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Natural::from_str(&s)
            .map_err(|_| serde::de::Error::custom("failed to parse Natural from decimal string"))
    }
}

impl TreeNode {
    pub fn new(color: Color, multiplicity: usize, children: Vec<TreeNode>) -> Self {
        Self {
            color,
            multiplicity: Natural::from(multiplicity),
            children,
        }
    }

    pub fn leaf(color: Color, multiplicity: usize) -> Self {
        Self::new(color, multiplicity, vec![])
    }
}

impl<T: BoxType> From<BoxValue<T>> for TreeNode {
    fn from(value: BoxValue<T>) -> Self {
        let col = value.get_color(0);
        let mul = value.get_multiplicity(0);
        let mut children = Vec::with_capacity(value.get_length(0) as usize);
        for child in value {
            children.push(child.into());
        }

        TreeNode {
            color: col,
            multiplicity: mul,
            children,
        }
    }
}

impl From<BoxVariant> for TreeNode {
    fn from(value: BoxVariant) -> Self {
        let col = value.get_color(0);
        let mul = value.get_multiplicity(0);
        let mut children = Vec::with_capacity(value.get_length(0) as usize);
        for child in value {
            children.push(child.into());
        }

        TreeNode {
            color: col,
            multiplicity: mul,
            children,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_into_tree() {
        let a = BoxValue::from(2) + BoxValue::alpha();
        let exp = TreeNode::new(
            Color::Black,
            1,
            vec![
                TreeNode::leaf(Color::Black, 2),
                TreeNode::new(Color::Black, 1, vec![TreeNode::leaf(Color::Black, 1)]),
            ],
        );

        assert_eq!(TreeNode::from(a), exp);

        let a = BoxVariant::from(2) + BoxVariant::alpha();
        let exp = TreeNode::new(
            Color::Black,
            1,
            vec![
                TreeNode::leaf(Color::Black, 2),
                TreeNode::new(Color::Black, 1, vec![TreeNode::leaf(Color::Black, 1)]),
            ],
        );

        assert_eq!(TreeNode::from(a), exp);
    }
}
