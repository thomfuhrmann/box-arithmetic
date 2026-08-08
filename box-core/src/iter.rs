use malachite::Natural;
use rapidhash::RapidHashSet;

use crate::{AnyBox, BoxContentKey, BoxKind, BoxType, BoxValue, BoxVariant, Color};

#[derive(Debug, Clone)]
pub struct IntoIter<T: BoxType> {
    raw: BoxValue<T>,
}

impl<T: BoxType> IntoIter<T> {
    pub fn new(value: BoxValue<T>) -> Self {
        let kinds: Vec<_> = value.kinds.into_iter().skip(1).collect();
        let colors: Vec<_> = value.colors.into_iter().skip(1).collect();
        let multiplicities: Vec<_> = value.multiplicities.into_iter().skip(1).collect();
        let lengths: Vec<_> = value.lengths.into_iter().skip(1).collect();

        IntoIter {
            raw: BoxValue::new_with(kinds, colors, multiplicities, lengths),
        }
    }
}

impl<T: BoxType> Iterator for IntoIter<T> {
    type Item = BoxValue<AnyBox>;

    fn next(&mut self) -> Option<Self::Item> {
        let child_len = match self.raw.lengths.first() {
            Some(&len) => len as usize,
            None => return None,
        };

        let kinds: Vec<_> = self.raw.kinds.drain(0..child_len).collect();
        let colors: Vec<_> = self.raw.colors.drain(0..child_len).collect();
        let multiplicities: Vec<_> = self.raw.multiplicities.drain(0..child_len).collect();
        let lengths: Vec<_> = self.raw.lengths.drain(0..child_len).collect();

        let child_value = BoxValue::<AnyBox>::new_with(kinds, colors, multiplicities, lengths);
        Some(child_value)
    }
}

impl<T: BoxType> IntoIterator for BoxValue<T> {
    type Item = BoxValue<AnyBox>;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self)
    }
}

impl<U: BoxType> FromIterator<BoxValue<AnyBox>> for BoxValue<U> {
    fn from_iter<T: IntoIterator<Item = BoxValue<AnyBox>>>(iter: T) -> Self {
        let mut result = BoxValue::new();
        result.kinds.push(BoxKind::Any);
        result.colors.push(Color::Black);
        result.multiplicities.push(1_u32.into());
        result.lengths.push(1);

        let mut unique_children: RapidHashSet<BoxContentKey> = RapidHashSet::default();
        for item in iter {
            unique_children.insert(BoxContentKey(item));
        }

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

impl<T: BoxType> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.raw.lengths.is_empty() {
            return None;
        }

        let len = self.raw.lengths.len();
        let mut curr_idx = 1;
        let mut at = 0;
        while curr_idx < len {
            let child_len = self.raw.lengths[curr_idx] as usize;
            curr_idx += child_len;
            if curr_idx == len {
                at = curr_idx - child_len;
                break;
            }
        }

        let kinds: Vec<_> = self.raw.kinds.split_off(at);
        let colors: Vec<_> = self.raw.colors.split_off(at);
        let multiplicities: Vec<_> = self.raw.multiplicities.split_off(at);
        let lengths: Vec<_> = self.raw.lengths.split_off(at);

        let child_value = BoxValue::<AnyBox>::new_with(kinds, colors, multiplicities, lengths);
        Some(child_value)
    }
}

impl IntoIterator for BoxVariant {
    type Item = BoxVariant;
    type IntoIter = BoxVariantIter;

    fn into_iter(self) -> Self::IntoIter {
        let raw_any = self.into_any_raw();

        BoxVariantIter {
            inner: IntoIter::new(raw_any),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BoxVariantIter {
    inner: IntoIter<AnyBox>,
}

impl Iterator for BoxVariantIter {
    type Item = BoxVariant;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(BoxVariant::repack_raw)
    }
}

#[derive(Debug, Clone, Copy, Hash)]
pub struct Iter<'a> {
    pub(crate) kinds: &'a [BoxKind],
    pub(crate) colors: &'a [Color],
    pub(crate) multiplicities: &'a [Natural],
    pub(crate) lengths: &'a [u32],
}

impl<'a, T: BoxType> IntoIterator for &'a BoxValue<T> {
    type Item = Iter<'a>;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            kinds: &self.kinds[1..],
            colors: &self.colors[1..],
            multiplicities: &self.multiplicities[1..],
            lengths: &self.lengths[1..],
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = Iter<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.lengths.is_empty() {
            return None;
        }

        let current_len = self.lengths[0] as usize;

        let item = Iter {
            kinds: &self.kinds[..current_len],
            colors: &self.colors[..current_len],
            multiplicities: &self.multiplicities[..current_len],
            lengths: &self.lengths[..current_len],
        };

        self.kinds = &self.kinds[current_len..];
        self.colors = &self.colors[current_len..];
        self.multiplicities = &self.multiplicities[current_len..];
        self.lengths = &self.lengths[current_len..];

        Some(item)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_ended() {
        let a = BoxValue::one() + BoxValue::alpha() + BoxValue::alpha() * BoxValue::alpha();
        let mut iter = a.into_iter();
        while let Some(val) = iter.next_back() {
            println!("{val}");
        }
    }
}
