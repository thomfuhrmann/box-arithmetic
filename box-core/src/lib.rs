use malachite::Natural;
use strum::EnumDiscriminants;

use std::{
    cmp::Ordering::Equal,
    // fmt::{self, Display, Formatter},
    hash::{BuildHasher, Hash, Hasher},
    marker::PhantomData,
    ops::{Add, Mul},
};

use rapidhash::fast::RandomState;

pub mod add;
pub mod derivative;
pub mod display;
pub mod from;
pub mod function;
pub mod maxel;
pub mod mul;
pub mod parser;
pub mod set;
pub mod store;

/// Kind of boxes that can exist in a store
#[derive(Debug, Clone, Hash, PartialEq, Eq, EnumDiscriminants)]
#[strum_discriminants(name(BoxKind))]
#[strum_discriminants(derive(Hash, PartialOrd, Ord))]
pub enum BoxVariant {
    Any(BoxValue<AnyBox>),
    Empty(BoxValue<EmptyBox>),
    Num(BoxValue<NumBox>),
    Polynum(BoxValue<PolynumBox>),
    Multinum(BoxValue<MultinumBox>),
    Unixel(BoxValue<UnixelBox>),
    Vexel(BoxValue<VexelBox>),
    Pixel(BoxValue<PixelBox>),
    Maxel(BoxValue<MaxelBox>),
    Set(BoxValue<SetBox>),
}

#[macro_export]
macro_rules! dispatch {
    (&$self:ident => $($field:tt)*) => {
        match $self {
            BoxVariant::Any(inner) => inner.$($field)*,
            BoxVariant::Empty(inner) => inner.$($field)*,
            BoxVariant::Num(inner) => inner.$($field)*,
            BoxVariant::Polynum(inner) => inner.$($field)*,
            BoxVariant::Multinum(inner) => inner.$($field)*,
            BoxVariant::Unixel(inner) => inner.$($field)*,
            BoxVariant::Vexel(inner) => inner.$($field)*,
            BoxVariant::Pixel(inner) => inner.$($field)*,
            BoxVariant::Maxel(inner) => inner.$($field)*,
            BoxVariant::Set(inner) => inner.$($field)*,
        }
    };

    (&mut $self:ident => $($field:tt)*) => {
        match $self {
            BoxVariant::Any(inner) => inner.$($field)*,
            BoxVariant::Empty(inner) => inner.$($field)*,
            BoxVariant::Num(inner) => inner.$($field)*,
            BoxVariant::Polynum(inner) => inner.$($field)*,
            BoxVariant::Multinum(inner) => inner.$($field)*,
            BoxVariant::Unixel(inner) => inner.$($field)*,
            BoxVariant::Vexel(inner) => inner.$($field)*,
            BoxVariant::Pixel(inner) => inner.$($field)*,
            BoxVariant::Maxel(inner) => inner.$($field)*,
            BoxVariant::Set(inner) => inner.$($field)*,
        }
    };

    ($self:ident => $($field:tt)*) => {
        match $self {
            BoxVariant::Any(inner) => inner.$($field)*,
            BoxVariant::Empty(inner) => inner.$($field)*,
            BoxVariant::Num(inner) => inner.$($field)*,
            BoxVariant::Polynum(inner) => inner.$($field)*,
            BoxVariant::Multinum(inner) => inner.$($field)*,
            BoxVariant::Unixel(inner) => inner.$($field)*,
            BoxVariant::Vexel(inner) => inner.$($field)*,
            BoxVariant::Pixel(inner) => inner.$($field)*,
            BoxVariant::Maxel(inner) => inner.$($field)*,
            BoxVariant::Set(inner) => inner.$($field)*,
        }
    };
}

impl BoxVariant {
    #[inline]
    pub fn get_kind(&self, idx: usize) -> BoxKind {
        dispatch!(self => kinds[idx])
    }

    #[inline]
    pub fn get_color(&self, idx: usize) -> Color {
        dispatch!(self => colors[idx])
    }

    #[inline]
    pub fn get_multiplicity(&self, idx: usize) -> Natural {
        dispatch!(self => multiplicities[idx].clone())
    }

    #[inline]
    pub fn get_length(&self, idx: usize) -> u32 {
        dispatch!(self => lengths[idx])
    }

    #[inline]
    pub fn set_kind(&mut self, idx: usize, kind: BoxKind) {
        dispatch!(self => kinds[idx] = kind);
    }

    #[inline]
    pub fn set_color(&mut self, idx: usize, col: Color) {
        dispatch!(self => colors[idx] = col);
    }

    #[inline]
    pub fn set_multiplicity(&mut self, idx: usize, mul: Natural) {
        dispatch!(self => multiplicities[idx] = mul);
    }

    #[inline]
    pub fn set_length(&mut self, idx: usize, len: u32) {
        dispatch!(self => lengths[idx] = len);
    }

    #[inline]
    pub fn into_any_raw(self) -> BoxValue<AnyBox> {
        dispatch!(self => cast::<AnyBox>())
    }

    #[inline]
    pub fn into_any(self) -> BoxVariant {
        dispatch!(self => cast::<AnyBox>()).into()
    }

    #[inline]
    pub fn is_anti(&self) -> bool {
        dispatch!(self => is_anti())
    }

    pub fn zero() -> Self {
        BoxValue::zero().into()
    }

    pub fn anti_zero() -> Self {
        BoxValue::anti_zero().into()
    }

    pub fn one() -> Self {
        BoxValue::one().into()
    }

    pub fn anti_one() -> Self {
        BoxValue::anti_one().into()
    }

    pub fn alpha() -> Self {
        BoxValue::alpha().into()
    }

    pub fn anti_alpha() -> Self {
        BoxValue::anti_alpha().into()
    }

    pub fn wrap<U: BoxType + IntoVariant>(self, mul: impl Into<Natural>) -> Self {
        dispatch!(self => wrap::<U>(mul)).into()
    }

    pub fn into_anti(mut self) -> Self {
        let col = self.get_color(0);
        match col {
            Color::Black => self.set_color(0, Color::Red),
            Color::Red => self.set_color(0, Color::Black),
        }
        self
    }

    /// Repack the box based on its runtime type
    pub fn repack_raw<T: BoxType>(raw: BoxValue<T>) -> Self {
        match raw.kinds[0] {
            BoxKind::Any => BoxVariant::Any(raw.cast::<AnyBox>()),
            BoxKind::Empty => BoxVariant::Empty(raw.cast::<EmptyBox>()),
            BoxKind::Num => BoxVariant::Num(raw.cast::<NumBox>()),
            BoxKind::Polynum => BoxVariant::Polynum(raw.cast::<PolynumBox>()),
            BoxKind::Multinum => BoxVariant::Multinum(raw.cast::<MultinumBox>()),
            BoxKind::Unixel => BoxVariant::Unixel(raw.cast::<UnixelBox>()),
            BoxKind::Vexel => BoxVariant::Vexel(raw.cast::<VexelBox>()),
            BoxKind::Pixel => BoxVariant::Pixel(raw.cast::<PixelBox>()),
            BoxKind::Maxel => BoxVariant::Maxel(raw.cast::<MaxelBox>()),
            BoxKind::Set => BoxVariant::Set(raw.cast::<SetBox>()),
        }
    }

    #[inline]
    pub fn hash_content(&self, random_state: &RandomState) -> u64 {
        dispatch!(self => hash_content(random_state))
    }

    #[inline]
    pub fn is_eq_content(&self, other: &Self) -> bool {
        match (self, other) {
            (BoxVariant::Any(l), BoxVariant::Any(r)) => l.is_eq_content(r),
            (BoxVariant::Empty(l), BoxVariant::Empty(r)) => l.is_eq_content(r),
            (BoxVariant::Num(l), BoxVariant::Num(r)) => l.is_eq_content(r),
            (BoxVariant::Polynum(l), BoxVariant::Polynum(r)) => l.is_eq_content(r),
            (BoxVariant::Multinum(l), BoxVariant::Multinum(r)) => l.is_eq_content(r),
            (BoxVariant::Vexel(l), BoxVariant::Vexel(r)) => l.is_eq_content(r),
            (BoxVariant::Maxel(l), BoxVariant::Maxel(r)) => l.is_eq_content(r),
            (BoxVariant::Set(l), BoxVariant::Set(r)) => l.is_eq_content(r),
            (_, _) => false,
        }
    }
}

/// Static conversion into [`BoxVariant`]
pub trait IntoVariant: BoxType {
    fn into_variant(value: BoxValue<Self>) -> BoxVariant;
}

impl IntoVariant for AnyBox {
    fn into_variant(v: BoxValue<Self>) -> BoxVariant {
        BoxVariant::Any(v)
    }
}

impl IntoVariant for EmptyBox {
    fn into_variant(v: BoxValue<Self>) -> BoxVariant {
        BoxVariant::Empty(v)
    }
}

impl IntoVariant for NumBox {
    fn into_variant(v: BoxValue<Self>) -> BoxVariant {
        BoxVariant::Num(v)
    }
}

impl IntoVariant for PolynumBox {
    fn into_variant(v: BoxValue<Self>) -> BoxVariant {
        BoxVariant::Polynum(v)
    }
}

impl IntoVariant for MultinumBox {
    fn into_variant(v: BoxValue<Self>) -> BoxVariant {
        BoxVariant::Multinum(v)
    }
}

impl IntoVariant for UnixelBox {
    fn into_variant(v: BoxValue<Self>) -> BoxVariant {
        BoxVariant::Unixel(v)
    }
}

impl IntoVariant for VexelBox {
    fn into_variant(v: BoxValue<Self>) -> BoxVariant {
        BoxVariant::Vexel(v)
    }
}

impl IntoVariant for PixelBox {
    fn into_variant(v: BoxValue<Self>) -> BoxVariant {
        BoxVariant::Pixel(v)
    }
}

impl IntoVariant for MaxelBox {
    fn into_variant(v: BoxValue<Self>) -> BoxVariant {
        BoxVariant::Maxel(v)
    }
}

impl IntoVariant for SetBox {
    fn into_variant(v: BoxValue<Self>) -> BoxVariant {
        BoxVariant::Set(v)
    }
}

impl<T: IntoVariant> From<BoxValue<T>> for BoxVariant {
    fn from(value: BoxValue<T>) -> Self {
        T::into_variant(value)
    }
}

/// Traits for types of boxes
pub trait BoxType: Sized + Clone {
    const KIND: BoxKind;
}

/// Implementations of the [`BoxType`] trait
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnyBox;
impl BoxType for AnyBox {
    const KIND: BoxKind = BoxKind::Any;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EmptyBox;
impl BoxType for EmptyBox {
    const KIND: BoxKind = BoxKind::Empty;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NumBox;
impl BoxType for NumBox {
    const KIND: BoxKind = BoxKind::Num;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PolynumBox;
impl BoxType for PolynumBox {
    const KIND: BoxKind = BoxKind::Polynum;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MultinumBox;
impl BoxType for MultinumBox {
    const KIND: BoxKind = BoxKind::Multinum;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PixelBox;
impl BoxType for PixelBox {
    const KIND: BoxKind = BoxKind::Pixel;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MaxelBox;
impl BoxType for MaxelBox {
    const KIND: BoxKind = BoxKind::Maxel;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnixelBox;
impl BoxType for UnixelBox {
    const KIND: BoxKind = BoxKind::Unixel;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VexelBox;
impl BoxType for VexelBox {
    const KIND: BoxKind = BoxKind::Vexel;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetBox;
impl BoxType for SetBox {
    const KIND: BoxKind = BoxKind::Set;
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BoxValue<T: BoxType> {
    pub(crate) kinds: Vec<BoxKind>,
    pub(crate) colors: Vec<Color>,
    pub(crate) multiplicities: Vec<Natural>,
    pub(crate) lengths: Vec<u32>,
    _marker: PhantomData<T>,
}

impl<T: BoxType> Default for BoxValue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Vec<BoxValue<AnyBox>>> for BoxValue<AnyBox> {
    fn from(value: Vec<BoxValue<AnyBox>>) -> Self {
        let mut result = BoxValue::new();
        result.kinds.push(BoxKind::Any);
        result.colors.push(Color::Black);
        result.multiplicities.push(malachite::Natural::from(1_u32));
        result.lengths.push(1);
        for any_box in value {
            result.extend(any_box);
        }
        result
    }
}

impl<T: BoxType> Hash for BoxValue<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.kinds.hash(state);
        self.colors.hash(state);
        self.multiplicities.hash(state);
        self.lengths.hash(state);
    }
}

impl<T: BoxType> BoxValue<T> {
    /// Initialize an empty raw box
    pub fn new() -> Self {
        Self {
            kinds: Vec::new(),
            colors: Vec::new(),
            multiplicities: Vec::new(),
            lengths: Vec::new(),
            _marker: PhantomData,
        }
    }

    /// Construct a box from the given vectors
    pub fn new_with(
        kinds: Vec<BoxKind>,
        colors: Vec<Color>,
        multiplicities: Vec<Natural>,
        lengths: Vec<u32>,
    ) -> Self {
        Self {
            kinds,
            colors,
            multiplicities,
            lengths,
            _marker: PhantomData,
        }
    }

    /// Return the kind of box
    pub fn kind(&self) -> BoxKind {
        T::KIND
    }

    /// Test if the box is an anti-box
    pub fn is_anti(&self) -> bool {
        self.get_color(0) == Color::Red
    }

    /// Cast this box to another box type
    pub fn cast<U: BoxType>(self) -> BoxValue<U> {
        BoxValue::<U>::new_with(self.kinds, self.colors, self.multiplicities, self.lengths)
    }

    /// Hash the content of the box
    fn hash_content(&self, random_state: &RandomState) -> u64 {
        let mut hasher = random_state.build_hasher();

        self.kinds.hash(&mut hasher);
        self.colors.get(1..).unwrap_or(&[]).hash(&mut hasher);
        self.multiplicities
            .get(1..)
            .unwrap_or(&[])
            .hash(&mut hasher);
        self.lengths.hash(&mut hasher);

        hasher.finish()
    }

    /// Compare the content of the two boxes for equality
    pub fn is_eq_content(&self, other: &Self) -> bool {
        let left_len = self.get_length(0) as usize;
        let right_len = other.get_length(0) as usize;

        if left_len != right_len {
            return false;
        }

        self.kinds == other.kinds
            && self.colors[1..] == other.colors[1..]
            && self.multiplicities[1..] == other.multiplicities[1..]
            && self.lengths[1..] == other.lengths[1..]
    }

    /// Sort the immediate child boxes of this box
    pub fn sort_immediate_children(&mut self) {
        if self.lengths.is_empty() {
            return;
        }

        let box_len = self.lengths[0] as usize;
        if box_len <= 1 {
            return;
        }

        let start_idx = 1;
        let end_idx = box_len;

        // collect offset ranges of immediate children
        let mut child_ranges = Vec::new();
        let mut curr = start_idx;
        while curr < end_idx {
            let len = self.lengths[curr] as usize;
            child_ranges.push((curr, len));
            curr += len;
        }

        if child_ranges.len() <= 1 {
            return;
        }

        // sort ranges
        child_ranges.sort_by(|&(start_a, len_a), &(start_b, len_b)| {
            let range_a = start_a..(start_a + len_a);
            let range_b = start_b..(start_b + len_b);

            let kinds_cmp = self.kinds[range_a.clone()].cmp(&self.kinds[range_b.clone()]);
            if kinds_cmp != Equal {
                return kinds_cmp;
            }

            let col_cmp = self.colors[range_a.clone()].cmp(&self.colors[range_b.clone()]);
            if col_cmp != Equal {
                return col_cmp;
            }

            let len_cmp = self.lengths[range_a.clone()].cmp(&self.lengths[range_b.clone()]);
            if len_cmp != Equal {
                return len_cmp;
            }

            self.multiplicities[range_a].cmp(&self.multiplicities[range_b])
        });

        // load staging buffers
        let content_len = end_idx - start_idx;
        let mut sorted_kinds = Vec::with_capacity(content_len);
        let mut sorted_colors = Vec::with_capacity(content_len);
        let mut sorted_lens = Vec::with_capacity(content_len);
        let mut sorted_mults = Vec::with_capacity(content_len);

        for &(start, len) in &child_ranges {
            let range = start..(start + len);
            sorted_kinds.extend_from_slice(&self.kinds[range.clone()]);
            sorted_colors.extend_from_slice(&self.colors[range.clone()]);
            sorted_lens.extend_from_slice(&self.lengths[range.clone()]);

            for idx in range {
                let item = std::mem::take(&mut self.multiplicities[idx]);
                sorted_mults.push(item);
            }
        }

        // load target buffers
        let target_range = start_idx..end_idx;
        self.kinds[target_range.clone()].copy_from_slice(&sorted_kinds);
        self.colors[target_range.clone()].copy_from_slice(&sorted_colors);
        self.lengths[target_range.clone()].copy_from_slice(&sorted_lens);

        for (dest_idx, src_natural) in target_range.zip(sorted_mults) {
            self.multiplicities[dest_idx] = src_natural;
        }
    }

    /// Extend the box with another box
    pub fn extend(&mut self, value: BoxValue<impl BoxType>) {
        if let Some(len) = self.lengths.get_mut(0) {
            *len += value.get_length(0);
        }
        self.kinds.extend(value.kinds);
        self.colors.extend(value.colors);
        self.multiplicities.extend(value.multiplicities);
        self.lengths.extend(value.lengths);
    }

    /// Extend the box with another box and multiplicity
    pub fn extend_with_mul(&mut self, mut value: BoxValue<impl BoxType>, mul: impl Into<Natural>) {
        value.set_multiplicity(0, mul);
        self.extend(value);
    }

    /// Return the k-th kind if it exists
    ///
    /// # Panics
    /// Panics if the index is out of bounds.
    pub fn get_kind(&self, index: usize) -> BoxKind {
        self.kinds[index]
    }

    /// Return the k-th color if it exists
    ///
    /// # Panics
    /// Panics if the index is out of bounds.
    pub fn get_color(&self, index: usize) -> Color {
        self.colors[index]
    }

    /// Return the k-th multiplicity
    ///
    /// # Panics
    /// Panics if the index is out of bounds.
    pub fn get_multiplicity(&self, index: usize) -> Natural {
        self.multiplicities[index].clone()
    }

    /// Return the k-th length
    ///
    /// # Panics
    /// Panics if the index is out of bounds.
    pub fn get_length(&self, index: usize) -> u32 {
        self.lengths[index]
    }

    /// Set the k-th kind
    ///
    /// # Panics
    /// Panics if the index is out of bounds.
    pub fn set_kind(&mut self, index: usize, kind: BoxKind) {
        self.kinds[index] = kind;
    }

    /// Set the k-th color
    ///
    /// # Panics
    /// Panics if the index is out of bounds.
    pub fn set_color(&mut self, index: usize, col: Color) {
        self.colors[index] = col;
    }

    /// Set the k-th multiplicity
    ///
    /// # Panics
    /// Panics if the index is out of bounds.
    pub fn set_multiplicity(&mut self, index: usize, mul: impl Into<Natural>) {
        self.multiplicities[index] = mul.into();
    }

    /// Set the k-th length
    ///
    /// # Panics
    /// Panics if the index is out of bounds.
    pub fn set_length(&mut self, index: usize, len: u32) {
        self.lengths[index] = len;
    }

    /// Remove the k-th row (without adjusting the lengths)
    pub fn remove(&mut self, index: usize) {
        self.kinds.remove(index);
        self.colors.remove(index);
        self.multiplicities.remove(index);
        self.lengths.remove(index);
    }

    /// Wrap a box in another box
    pub fn wrap<U: BoxType>(mut self, mul: impl Into<Natural>) -> BoxValue<U> {
        self.set_multiplicity(0, mul);

        let mut result = BoxValue::<U>::new();
        result.kinds.push(U::KIND);
        result.colors.push(Color::Black);
        result.multiplicities.push(Natural::from(1_u32));
        result.lengths.push(1);

        result.extend(self);

        result
    }

    /// Invert the color of the box
    pub fn into_anti(mut self) -> Self {
        let col = self.get_color(0);
        if col == Color::Black {
            self.set_color(0, Color::Red);
        } else {
            self.set_color(0, Color::Black);
        }
        self
    }
}

impl BoxValue<AnyBox> {
    /// Construct an empty box
    pub fn empty() -> Self {
        BoxValue {
            kinds: vec![BoxKind::Any],
            colors: vec![Color::Black],
            multiplicities: vec![Natural::from(1_u32)],
            lengths: vec![1],
            _marker: std::marker::PhantomData,
        }
    }

    /// Construct an empty red box
    pub fn anti_empty() -> Self {
        BoxValue {
            kinds: vec![BoxKind::Any],
            colors: vec![Color::Red],
            multiplicities: vec![Natural::from(1_u32)],
            lengths: vec![1],
            _marker: std::marker::PhantomData,
        }
    }
}

impl BoxValue<EmptyBox> {
    /// Construct an empty black box
    pub fn zero() -> Self {
        BoxValue {
            kinds: vec![BoxKind::Empty],
            colors: vec![Color::Black],
            multiplicities: vec![Natural::from(1_u32)],
            lengths: vec![1],
            _marker: std::marker::PhantomData,
        }
    }

    /// Construct an empty red box
    pub fn anti_zero() -> Self {
        BoxValue {
            kinds: vec![BoxKind::Empty],
            colors: vec![Color::Red],
            multiplicities: vec![Natural::from(1_u32)],
            lengths: vec![1],
            _marker: std::marker::PhantomData,
        }
    }
}

impl BoxValue<NumBox> {
    /// Construct the box representing the number one
    pub fn one() -> Self {
        BoxValue {
            kinds: vec![BoxKind::Num, BoxKind::Empty],
            colors: vec![Color::Black, Color::Black],
            multiplicities: vec![Natural::from(1_u32), Natural::from(1_u32)],
            lengths: vec![2, 1],
            _marker: std::marker::PhantomData,
        }
    }

    /// Construct the anti-box representing the number one
    pub fn anti_one() -> Self {
        BoxValue {
            kinds: vec![BoxKind::Num, BoxKind::Empty],
            colors: vec![Color::Red, Color::Black],
            multiplicities: vec![Natural::from(1_u32), Natural::from(1_u32)],
            lengths: vec![2, 1],
            _marker: std::marker::PhantomData,
        }
    }
}

impl BoxValue<PolynumBox> {
    /// Construct the variable alpha
    pub fn alpha() -> Self {
        BoxValue {
            kinds: vec![BoxKind::Polynum, BoxKind::Num, BoxKind::Empty],
            colors: vec![Color::Black, Color::Black, Color::Black],
            multiplicities: vec![
                Natural::from(1_u32),
                Natural::from(1_u32),
                Natural::from(1_u32),
            ],
            lengths: vec![3, 2, 1],
            _marker: std::marker::PhantomData,
        }
    }

    /// Construct the variable anti-alpha
    pub fn anti_alpha() -> Self {
        BoxValue {
            kinds: vec![BoxKind::Polynum, BoxKind::Num, BoxKind::Empty],
            colors: vec![Color::Black, Color::Red, Color::Black],
            multiplicities: vec![
                Natural::from(1_u32),
                Natural::from(1_u32),
                Natural::from(1_u32),
            ],
            lengths: vec![3, 2, 1],
            _marker: std::marker::PhantomData,
        }
    }
}

impl BoxValue<MultinumBox> {
    /// Construct the variable beta
    pub fn beta(n: impl Into<Natural>) -> Self {
        BoxValue {
            kinds: vec![
                BoxKind::Multinum,
                BoxKind::Polynum,
                BoxKind::Num,
                BoxKind::Empty,
            ],
            colors: vec![Color::Black, Color::Black, Color::Black, Color::Black],
            multiplicities: vec![
                Natural::from(1_u32),
                Natural::from(1_u32),
                Natural::from(1_u32),
                n.into(),
            ],
            lengths: vec![4, 3, 2, 1],
            _marker: std::marker::PhantomData,
        }
    }

    /// Construct the variable anti-beta
    pub fn anti_beta(n: impl Into<Natural>) -> Self {
        BoxValue {
            kinds: vec![
                BoxKind::Multinum,
                BoxKind::Polynum,
                BoxKind::Num,
                BoxKind::Empty,
            ],
            colors: vec![Color::Black, Color::Red, Color::Black, Color::Black],
            multiplicities: vec![
                Natural::from(1_u32),
                Natural::from(1_u32),
                Natural::from(1_u32),
                n.into(),
            ],
            lengths: vec![4, 3, 2, 1],
            _marker: std::marker::PhantomData,
        }
    }
}

#[derive(Debug)]
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

/// Color of a box
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Color {
    Black,
    Red,
}

impl Color {
    pub fn invert(self) -> Self {
        match self {
            Color::Black => Color::Red,
            Color::Red => Color::Black,
        }
    }
}

impl Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        match (self, rhs) {
            (Color::Black, Color::Black) => Color::Black,
            (Color::Black, Color::Red) => Color::Red,
            (Color::Red, Color::Black) => Color::Red,
            (Color::Red, Color::Red) => Color::Black,
        }
    }
}

impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        match (self, rhs) {
            (Color::Black, Color::Black) => Color::Black,
            (Color::Black, Color::Red) => Color::Red,
            (Color::Red, Color::Black) => Color::Red,
            (Color::Red, Color::Red) => Color::Black,
        }
    }
}
