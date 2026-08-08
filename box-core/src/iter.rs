use malachite::Natural;
use rapidhash::RapidHashSet;

use crate::{AnyBox, BoxContentKey, BoxKind, BoxType, BoxValue, BoxVariant, Color};

#[derive(Debug, Clone)]
pub struct BoxValueIter<T: BoxType> {
    raw: BoxValue<T>,
}

impl<T: BoxType> BoxValueIter<T> {
    pub fn new(value: BoxValue<T>) -> Self {
        let kinds: Vec<_> = value.kinds.into_iter().skip(1).collect();
        let colors: Vec<_> = value.colors.into_iter().skip(1).collect();
        let multiplicities: Vec<_> = value.multiplicities.into_iter().skip(1).collect();
        let lengths: Vec<_> = value.lengths.into_iter().skip(1).collect();

        BoxValueIter {
            raw: BoxValue::new_with(kinds, colors, multiplicities, lengths),
        }
    }
}

impl<T: BoxType> Iterator for BoxValueIter<T> {
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
    type IntoIter = BoxValueIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        BoxValueIter::new(self)
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

impl IntoIterator for BoxVariant {
    type Item = BoxVariant;
    type IntoIter = BoxVariantIter;

    fn into_iter(self) -> Self::IntoIter {
        let raw_any = self.into_any_raw();

        BoxVariantIter {
            inner: BoxValueIter::new(raw_any),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BoxVariantIter {
    inner: BoxValueIter<AnyBox>,
}

impl Iterator for BoxVariantIter {
    type Item = BoxVariant;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(BoxVariant::repack_raw)
    }
}

#[derive(Debug, Clone, Copy, Hash)]
pub struct BoxValueRef<'a> {
    pub(crate) kinds: &'a [BoxKind],
    pub(crate) colors: &'a [Color],
    pub(crate) multiplicities: &'a [Natural],
    pub(crate) lengths: &'a [u32],
}

impl<'a, T: BoxType> IntoIterator for &'a BoxValue<T> {
    type Item = BoxValueRef<'a>;
    type IntoIter = BoxValueRef<'a>;

    fn into_iter(self) -> Self::IntoIter {
        BoxValueRef {
            kinds: &self.kinds[1..],
            colors: &self.colors[1..],
            multiplicities: &self.multiplicities[1..],
            lengths: &self.lengths[1..],
        }
    }
}

impl<'a> Iterator for BoxValueRef<'a> {
    type Item = BoxValueRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.lengths.is_empty() {
            return None;
        }

        let current_len = self.lengths[0] as usize;

        let item = BoxValueRef {
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
