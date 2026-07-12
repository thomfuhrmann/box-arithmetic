//! Maxel is an extension of matrices into the world of boxes

use crate::{AnyBox, BoxKind, BoxType, BoxValue, Color, MaxelBox, PixelBox, UnixelBox, VexelBox};
use malachite::Natural;
use rapidhash::RapidHashMap;

impl BoxValue<UnixelBox> {
    /// Create a unixel out of a box
    pub fn unixel<T: BoxType>(value: BoxValue<T>) -> Self {
        let len = value.get_length(0);

        let mut result = BoxValue::<UnixelBox>::new();
        result.kinds.push(BoxKind::Unixel);
        result.colors.push(Color::Black);
        result.multiplicities.push(Natural::from(1_u32));
        result.lengths.push(1 + len);

        result.kinds.extend(value.kinds);
        result.colors.extend(value.colors);
        result.multiplicities.extend(value.multiplicities);
        result.lengths.extend(value.lengths);

        result
    }

    /// Return the box inside the unixel
    pub fn x(&self) -> BoxValue<AnyBox> {
        let x_len = self.get_length(1) as usize;
        BoxValue::new_with(
            self.kinds[1..1 + x_len].to_vec(),
            self.colors[1..1 + x_len].to_vec(),
            self.multiplicities[1..1 + x_len].to_vec(),
            self.lengths[1..1 + x_len].to_vec(),
        )
    }
}

impl From<Vec<BoxValue<UnixelBox>>> for BoxValue<VexelBox> {
    /// Create a vexel from unixels
    fn from(value: Vec<BoxValue<UnixelBox>>) -> Self {
        let mut result = BoxValue::new();
        result.kinds.push(BoxKind::Vexel);
        result.colors.push(Color::Black);
        result.multiplicities.push(malachite::Natural::from(1_u32));
        result.lengths.push(1);
        for pix in value {
            result.extend(pix);
        }
        result
    }
}

impl BoxValue<PixelBox> {
    /// Create a pixel from two boxes
    pub fn pixel<X: BoxType, Y: BoxType>(x: BoxValue<X>, y: BoxValue<Y>) -> Self {
        let x_len = x.get_length(0);
        let y_len = y.get_length(0);

        let mut result = BoxValue::<PixelBox>::new();
        result.kinds.push(BoxKind::Pixel);
        result.colors.push(Color::Black);
        result.multiplicities.push(Natural::from(1_u32));
        result.lengths.push(1 + x_len + y_len);

        result.kinds.extend(x.kinds);
        result.colors.extend(x.colors);
        result.multiplicities.extend(x.multiplicities);
        result.lengths.extend(x.lengths);

        result.kinds.extend(y.kinds);
        result.colors.extend(y.colors);
        result.multiplicities.extend(y.multiplicities);
        result.lengths.extend(y.lengths);

        result
    }

    /// Return the first child of a pixel
    pub fn x(&self) -> BoxValue<AnyBox> {
        let x_len = self.get_length(1) as usize;
        BoxValue::new_with(
            self.kinds[1..1 + x_len].to_vec(),
            self.colors[1..1 + x_len].to_vec(),
            self.multiplicities[1..1 + x_len].to_vec(),
            self.lengths[1..1 + x_len].to_vec(),
        )
    }

    /// Return the second child of a pixel
    pub fn y(&self) -> BoxValue<AnyBox> {
        let x_len = self.get_length(1) as usize;
        let y_idx = 1 + x_len;
        let y_len = self.get_length(y_idx) as usize;
        BoxValue::new_with(
            self.kinds[y_idx..y_idx + y_len].to_vec(),
            self.colors[y_idx..y_idx + y_len].to_vec(),
            self.multiplicities[y_idx..y_idx + y_len].to_vec(),
            self.lengths[y_idx..y_idx + y_len].to_vec(),
        )
    }

    /// Multiplies two pixels
    pub fn mul_pix(left: Self, right: Self) -> Option<Self> {
        let left_y = left.y();
        let right_x = right.x();

        if left_y == right_x {
            let left_x = left.x();
            let right_y = right.y();
            return Some(Self::pixel(left_x, right_y));
        }

        None
    }

    /// Multiply a pixel with a unixel
    fn mul_pix_unix(self, unix: BoxValue<UnixelBox>) -> Option<BoxValue<UnixelBox>> {
        let pix_y = self.y();
        let unix_x = unix.x();

        if pix_y == unix_x {
            return Some(BoxValue::<UnixelBox>::unixel(self.x()));
        }

        None
    }
}

impl From<Vec<BoxValue<PixelBox>>> for BoxValue<MaxelBox> {
    fn from(value: Vec<BoxValue<PixelBox>>) -> Self {
        let mut result = BoxValue::<MaxelBox>::new();
        result.kinds.push(BoxKind::Maxel);
        result.colors.push(Color::Black);
        result.multiplicities.push(malachite::Natural::from(1_u32));
        result.lengths.push(1);
        for pix in value {
            result.extend(pix);
        }
        result
    }
}

impl BoxValue<MaxelBox> {
    /// Multiply two maxels
    pub fn mul_max(left: Self, right: Self) -> Self {
        let mut unique_children: RapidHashMap<u64, BoxValue<PixelBox>> =
            rapidhash::RapidHashMap::default();

        let mut result = BoxValue::<MaxelBox>::new();
        result.kinds.push(BoxKind::Maxel);
        result.colors.push(Color::Black);
        result.multiplicities.push(Natural::from(1_u32));
        result.lengths.push(1);

        for left_pix in left {
            let col = left_pix.get_color(0);
            let lhs_mul = left_pix.get_multiplicity(0);
            for right_pix in right.clone() {
                let rhs_mul = right_pix.get_multiplicity(0);
                if let Some(mut pixel) = BoxValue::mul_pix(
                    left_pix.clone().cast::<PixelBox>(),
                    right_pix.cast::<PixelBox>(),
                ) {
                    let struct_hash = pixel.hash_content(unique_children.hasher());
                    if let Some(other) = unique_children.get_mut(&struct_hash)
                        && pixel.is_eq_content(other)
                    {
                        let other_col = other.get_color(0);
                        let other_mul = other.get_multiplicity(0);
                        other.set_color(0, col * other_col);
                        other.set_multiplicity(0, other_mul + lhs_mul.clone());
                    } else {
                        pixel.set_multiplicity(0, lhs_mul.clone() * rhs_mul);
                        unique_children.insert(struct_hash, pixel);
                    }
                }
            }
        }
        for pixel in unique_children.into_values() {
            let mul = pixel.get_multiplicity(0);
            if mul == 0 {
                continue;
            }

            result.extend(pixel);
        }
        result.sort_immediate_children();
        result
    }

    /// Multiply a maxel with a vexel
    pub fn mul_max_vex(self, vex: BoxValue<VexelBox>) -> BoxValue<VexelBox> {
        let mut unique_children: RapidHashMap<u64, BoxValue<UnixelBox>> =
            rapidhash::RapidHashMap::default();

        let mut result = BoxValue::<VexelBox>::new();
        result.kinds.push(BoxKind::Vexel);
        result.colors.push(Color::Black);
        result.multiplicities.push(Natural::from(1_u32));
        result.lengths.push(1);

        for left_pix in self {
            let col = left_pix.get_color(0);
            let lhs_mul = left_pix.get_multiplicity(0);
            for right_unix in vex.clone() {
                let rhs_mul = right_unix.get_multiplicity(0);
                if let Some(mut unixel) = left_pix
                    .clone()
                    .cast::<PixelBox>()
                    .mul_pix_unix(right_unix.cast::<UnixelBox>())
                {
                    let struct_hash = unixel.hash_content(unique_children.hasher());
                    if let Some(other) = unique_children.get_mut(&struct_hash)
                        && unixel.is_eq_content(other)
                    {
                        let other_col = other.get_color(0);
                        let other_mul = other.get_multiplicity(0);
                        other.set_color(0, col * other_col);
                        other.set_multiplicity(0, other_mul + lhs_mul.clone());
                    } else {
                        unixel.set_multiplicity(0, lhs_mul.clone() * rhs_mul);
                        unique_children.insert(struct_hash, unixel);
                    }
                }
            }
        }
        for unixel in unique_children.into_values() {
            let mul = unixel.get_multiplicity(0);
            if mul == 0 {
                continue;
            }

            result.extend(unixel);
        }
        result.sort_immediate_children();
        result
    }
}

#[macro_export]
macro_rules! pixel {
    ($x:expr, $y:expr) => {{ $crate::BoxValue::<$crate::maxel::PixelBox>::pixel($x.into(), $y.into()) }};
}

#[macro_export]
macro_rules! vexel {
    ([$($x:expr),* $(,)?]) => {
       {
            use malachite::base::num::arithmetic::traits::SaturatingSub;
            let mut result = $crate::BoxValue::<$crate::VexelBox>::new();
            result.kinds.push($crate::BoxKind::Vexel);
            result.colors.push($crate::Color::Black);
            result.multiplicities.push(malachite::Natural::from(1_u32));
            result.lengths.push(1);

            let mut unique_children: rapidhash::RapidHashMap<u64, $crate::BoxValue<$crate::UnixelBox>> = rapidhash::RapidHashMap::default();
            $(
                let unix = $crate::BoxValue::<$crate::UnixelBox>::unixel(($x).into());
                let col = unix.get_color(0);
                let mul = unix.get_multiplicity(0);
                let struct_hash = unix.hash_content(unique_children.hasher());
                if let Some(other) = unique_children.get_mut(&struct_hash)
                    && unix.is_eq_content(other)
                {
                    let other_col = other.get_color(0);
                    let other_mul = other.get_multiplicity(0);
                    if col + other_col == $crate::Color::Red {
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
                    unique_children.insert(struct_hash, unix);
                }
            )*

            for unixel in unique_children.into_values() {
                let mul = unixel.get_multiplicity(0);
                if mul == 0 {
                    continue;
                }

                result.extend(unixel);
            }
            result.sort_immediate_children();
            $crate::BoxVariant::from(result)
       }
    };
}

#[macro_export]
macro_rules! maxel {
    ([$([$x:expr, $y:expr]),* $(,)?]) => {
        {
            use malachite::base::num::arithmetic::traits::SaturatingSub;
            let mut result = $crate::BoxValue::<$crate::MaxelBox>::new();
            result.kinds.push($crate::BoxKind::Maxel);
            result.colors.push($crate::Color::Black);
            result.multiplicities.push(malachite::Natural::from(1_u32));
            result.lengths.push(1);

            let mut unique_children: rapidhash::RapidHashMap<u64, $crate::BoxValue<$crate::PixelBox>> = rapidhash::RapidHashMap::default();
            $(
                let pix = $crate::BoxValue::<$crate::PixelBox>::pixel(($x).into(), ($y).into());
                let col = pix.get_color(0);
                let mul = pix.get_multiplicity(0);
                let struct_hash = pix.hash_content(unique_children.hasher());
                if let Some(other) = unique_children.get_mut(&struct_hash)
                    && pix.is_eq_content(other)
                {
                    let other_col = other.get_color(0);
                    let other_mul = other.get_multiplicity(0);
                    if col + other_col == $crate::Color::Red {
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
                    unique_children.insert(struct_hash, pix);
                }
            )*

            for pixel in unique_children.into_values() {
                let mul = pixel.get_multiplicity(0);
                if mul == 0 {
                    continue;
                }

                result.extend(pixel);
            }
            result.sort_immediate_children();
            $crate::BoxVariant::from(result)
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::BoxValue;

    #[test]
    fn test_pixel() {
        let p1 = pixel!(1, 2);
        let p2 = pixel!(2, 3);
        let p3 = BoxValue::mul_pix(p1.clone(), p2);
        let expected = pixel!(1, 3);

        assert_eq!(p3, Some(expected));

        let p4 = pixel!(3, 2);
        let p5 = BoxValue::mul_pix(p1, p4);
        assert!(p5.is_none());
    }

    #[test]
    fn test_maxel() {
        let a = maxel![[[1, 1], [1, 2], [2, 2]]];
        let b = maxel![[[1, 2], [2, 1]]];

        let prod = a * b;
        let expected = maxel![[[1, 1], [1, 2], [2, 1]]];
        assert_eq!(prod, expected);

        let a = maxel![[[1, 1], [1, 1], [1, 2], [2, 2], [1, 4]]];
        let b = maxel![[[1, 2], [2, 1], [4, 1]]];

        let prod = a * b;
        let expected = maxel![[[1, 1], [1, 1], [1, 2], [1, 2], [2, 1]]];
        assert_eq!(prod, expected);

        let m = maxel![[[1, 1], [2, 2], [3, 3]]];
        let v = vexel!([1, 2, 3]);
        let prod = m * v;
        let expected = vexel!([1, 2, 3]);
        assert_eq!(prod, expected);

        let m = maxel![[[1, 1], [2, 2], [3, 3]]];
        let v = vexel!([1, 1, 2, 3]);
        let prod = m * v;
        let expected = vexel!([1, 1, 2, 3]);
        assert_eq!(prod, expected);
    }
}
