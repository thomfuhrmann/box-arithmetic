use malachite::Natural;
use strum::EnumDiscriminants;

use std::{
    cmp::Ordering::{self},
    hash::{Hash, Hasher},
    marker::PhantomData,
    ops::{Add, Mul, Range},
};

pub mod add;
pub mod derivative;
pub mod display;
pub mod div;
pub mod from;
pub mod function;
pub mod iter;
pub mod maxel;
pub mod mul;
pub mod parser;
pub mod set;
pub mod store;
pub mod sub;

/// Trait for types of boxes
pub trait BoxType: Sized + Clone + PartialEq + Eq + std::fmt::Debug {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ListBox;
impl BoxType for ListBox {
    const KIND: BoxKind = BoxKind::List;
}

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
    List(BoxValue<ListBox>),
}

#[macro_export]
macro_rules! dispatch {
    (&$self:ident => $($op:tt)*) => {
        match $self {
            BoxVariant::Any(inner) => inner.$($op)*,
            BoxVariant::Empty(inner) => inner.$($op)*,
            BoxVariant::Num(inner) => inner.$($op)*,
            BoxVariant::Polynum(inner) => inner.$($op)*,
            BoxVariant::Multinum(inner) => inner.$($op)*,
            BoxVariant::Unixel(inner) => inner.$($op)*,
            BoxVariant::Vexel(inner) => inner.$($op)*,
            BoxVariant::Pixel(inner) => inner.$($op)*,
            BoxVariant::Maxel(inner) => inner.$($op)*,
            BoxVariant::Set(inner) => inner.$($op)*,
            BoxVariant::List(inner) => inner.$($op)*,
        }
    };

    (&mut $self:ident => $($op:tt)*) => {
        match $self {
            BoxVariant::Any(inner) => inner.$($op)*,
            BoxVariant::Empty(inner) => inner.$($op)*,
            BoxVariant::Num(inner) => inner.$($op)*,
            BoxVariant::Polynum(inner) => inner.$($op)*,
            BoxVariant::Multinum(inner) => inner.$($op)*,
            BoxVariant::Unixel(inner) => inner.$($op)*,
            BoxVariant::Vexel(inner) => inner.$($op)*,
            BoxVariant::Pixel(inner) => inner.$($op)*,
            BoxVariant::Maxel(inner) => inner.$($op)*,
            BoxVariant::Set(inner) => inner.$($op)*,
            BoxVariant::List(inner) => inner.$($op)*,
        }
    };

    ($self:ident => $($op:tt)*) => {
        match $self {
            BoxVariant::Any(inner) => inner.$($op)*,
            BoxVariant::Empty(inner) => inner.$($op)*,
            BoxVariant::Num(inner) => inner.$($op)*,
            BoxVariant::Polynum(inner) => inner.$($op)*,
            BoxVariant::Multinum(inner) => inner.$($op)*,
            BoxVariant::Unixel(inner) => inner.$($op)*,
            BoxVariant::Vexel(inner) => inner.$($op)*,
            BoxVariant::Pixel(inner) => inner.$($op)*,
            BoxVariant::Maxel(inner) => inner.$($op)*,
            BoxVariant::Set(inner) => inner.$($op)*,
            BoxVariant::List(inner) => inner.$($op)*,
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
    pub fn set_multiplicity(&mut self, idx: usize, mul: impl Into<Natural>) {
        dispatch!(self => multiplicities[idx] = mul.into());
    }

    #[inline]
    pub fn set_length(&mut self, idx: usize, len: u32) {
        dispatch!(self => lengths[idx] = len);
    }

    /// Returns the underlying box as any box
    #[inline]
    pub fn into_any_raw(self) -> BoxValue<AnyBox> {
        dispatch!(self => cast::<AnyBox>())
    }

    /// Convert into any box
    #[inline]
    pub fn into_any(self) -> BoxVariant {
        dispatch!(self => cast::<AnyBox>()).into()
    }

    /// Check if is is an anti-box
    #[inline]
    pub fn is_anti(&self) -> bool {
        dispatch!(self => is_anti())
    }

    /// Create a zero
    pub fn zero() -> Self {
        BoxValue::zero().into()
    }

    /// Create an anti-zero
    pub fn anti_zero() -> Self {
        BoxValue::anti_zero().into()
    }

    /// Create a one
    pub fn one() -> Self {
        BoxValue::one().into()
    }

    /// Create an anti-one
    pub fn anti_one() -> Self {
        BoxValue::anti_one().into()
    }

    /// Create an alpha
    pub fn alpha() -> Self {
        BoxValue::alpha().into()
    }

    /// Create an anti-alpha
    pub fn anti_alpha() -> Self {
        BoxValue::anti_alpha().into()
    }

    /// Creat a beta
    pub fn beta(idx: impl Into<Natural>) -> Self {
        BoxValue::beta(idx).into()
    }

    /// Create an anti-beta
    pub fn anti_beta(idx: impl Into<Natural>) -> Self {
        BoxValue::anti_beta(idx).into()
    }

    /// Wrap the box in an empty box
    pub fn wrap<U: BoxType + IntoVariant>(self, mul: impl Into<Natural>) -> Self {
        dispatch!(self => wrap::<U>(mul)).into()
    }

    /// Convert a box into its anti-box
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
            BoxKind::Any => BoxVariant::Any(raw.cast()),
            BoxKind::Empty => BoxVariant::Empty(raw.cast()),
            BoxKind::Num => BoxVariant::Num(raw.cast()),
            BoxKind::Polynum => BoxVariant::Polynum(raw.cast()),
            BoxKind::Multinum => BoxVariant::Multinum(raw.cast()),
            BoxKind::Unixel => BoxVariant::Unixel(raw.cast()),
            BoxKind::Vexel => BoxVariant::Vexel(raw.cast()),
            BoxKind::Pixel => BoxVariant::Pixel(raw.cast()),
            BoxKind::Maxel => BoxVariant::Maxel(raw.cast()),
            BoxKind::Set => BoxVariant::Set(raw.cast()),
            BoxKind::List => BoxVariant::List(raw.cast()),
        }
    }

    #[inline]
    pub fn hash_content<H: Hasher>(&self, hasher: H) -> u64 {
        dispatch!(self => hash_content(hasher))
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
            (BoxVariant::List(l), BoxVariant::List(r)) => l.is_eq_content(r),
            (_, _) => false,
        }
    }

    #[inline]
    pub fn sort_immediate_children(&mut self) {
        dispatch!(self => sort_immediate_children());
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

impl IntoVariant for ListBox {
    fn into_variant(v: BoxValue<Self>) -> BoxVariant {
        BoxVariant::List(v)
    }
}

impl<T: IntoVariant> From<BoxValue<T>> for BoxVariant {
    fn from(value: BoxValue<T>) -> Self {
        T::into_variant(value)
    }
}

#[derive(Debug, Eq, Clone)]
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

impl<T: BoxType> Hash for BoxValue<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.kinds.hash(state);
        self.colors.hash(state);
        self.multiplicities.hash(state);
        self.lengths.hash(state);
    }
}

impl<T: BoxType, U: BoxType> PartialEq<BoxValue<U>> for BoxValue<T> {
    fn eq(&self, other: &BoxValue<U>) -> bool {
        self.kinds == other.kinds
            && self.colors == other.colors
            && self.multiplicities == other.multiplicities
            && self.lengths == other.lengths
    }
}

// PartialOrd is necessary here to allow two different type parameters for LHS and RHS respectively
impl<T: BoxType, U: BoxType> PartialOrd<BoxValue<U>> for BoxValue<T> {
    fn partial_cmp(&self, other: &BoxValue<U>) -> Option<Ordering> {
        let len_a = self.get_length(0) as usize;
        let range_a = 1..len_a;
        let len_b = other.get_length(0) as usize;
        let range_b = 1..len_b;
        Some(self.cmp_ranges(other, range_a, range_b))
    }
}

impl<T: BoxType> BoxValue<T> {
    /// Initializes an empty raw box
    pub fn new() -> Self {
        Self {
            kinds: Vec::new(),
            colors: Vec::new(),
            multiplicities: Vec::new(),
            lengths: Vec::new(),
            _marker: PhantomData,
        }
    }

    /// Constructs a box from the given vectors
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

    /// Returns the kind of box
    pub fn kind(&self) -> BoxKind {
        T::KIND
    }

    /// Tests if the box is an anti-box
    pub fn is_anti(&self) -> bool {
        self.get_color(0) == Color::Red
    }

    /// Casts this box to another box type
    pub fn cast<U: BoxType>(self) -> BoxValue<U> {
        BoxValue::<U>::new_with(self.kinds, self.colors, self.multiplicities, self.lengths)
    }

    /// Hashes the content of the box ignoring its outer multiplicity and color
    fn hash_content<H: Hasher>(&self, mut hasher: H) -> u64 {
        self.kinds.hash(&mut hasher);
        self.colors.get(1..).unwrap_or(&[]).hash(&mut hasher);
        self.multiplicities
            .get(1..)
            .unwrap_or(&[])
            .hash(&mut hasher);
        self.lengths.hash(&mut hasher);

        hasher.finish()
    }

    /// Compares the content of the two boxes ignoring their outer multiplicities and colors
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

    /// Reusable helper function for box comparison
    fn cmp_ranges<U: BoxType>(
        &self,
        other: &BoxValue<U>,
        range_a: Range<usize>,
        range_b: Range<usize>,
    ) -> Ordering {
        // values at the end carry the most weight
        // red comes before black
        self.multiplicities[range_a.clone()]
            .iter()
            .cmp(other.multiplicities[range_b.clone()].iter())
            .then(self.colors[range_a].cmp(&other.colors[range_b]))
    }

    /// Sorts immediate child boxes
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

            self.cmp_ranges(self, range_a, range_b)
        });

        // load staging buffers
        let content_len = end_idx - start_idx;
        let mut sorted_kinds = Vec::with_capacity(content_len);
        let mut sorted_colors = Vec::with_capacity(content_len);
        let mut sorted_lens = Vec::with_capacity(content_len);
        let mut sorted_mults = Vec::with_capacity(content_len);

        for &(start, len) in child_ranges.iter() {
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

    /// Extends the box with another box
    pub fn extend(&mut self, value: BoxValue<impl BoxType>) {
        if let Some(len) = self.lengths.get_mut(0) {
            *len += value.get_length(0);
        }
        self.kinds.extend(value.kinds);
        self.colors.extend(value.colors);
        self.multiplicities.extend(value.multiplicities);
        self.lengths.extend(value.lengths);
    }

    /// Extends the box with another box and outer multiplicity
    pub fn extend_with_mul(&mut self, mut value: BoxValue<impl BoxType>, mul: impl Into<Natural>) {
        value.set_multiplicity(0, mul);
        self.extend(value);
    }

    /// Gets the first child by copying its elements into a new box
    pub fn first_child(&self) -> BoxValue<AnyBox> {
        let child_len = self.get_length(1) as usize;
        let range = 1..1 + child_len;

        let kinds = self.kinds[range.clone()].to_vec();
        let colors = self.colors[range.clone()].to_vec();
        let lengths = self.lengths[range.clone()].to_vec();
        let mults = self.multiplicities[range].to_vec();

        BoxValue::new_with(kinds, colors, mults, lengths)
    }

    /// Returns the last child by copying its elements into a new box
    pub fn last_child(&self) -> BoxValue<AnyBox> {
        let box_len = self.lengths[0] as usize;

        let start_idx = 1;
        let end_idx = box_len;

        let mut curr = start_idx;
        let mut range = 0..0;
        while curr < end_idx {
            let len = self.lengths[curr] as usize;
            if curr + len == box_len {
                range = curr..curr + len;
                break;
            }
            curr += len;
        }

        let kinds = self.kinds[range.clone()].to_vec();
        let colors = self.colors[range.clone()].to_vec();
        let lengths = self.lengths[range.clone()].to_vec();
        let mults = self.multiplicities[range].to_vec();

        BoxValue::new_with(kinds, colors, mults, lengths)
    }

    /// Returns the k-th kind if it exists
    ///
    /// # Panics
    /// Panics if the index is out of bounds.
    pub fn get_kind(&self, index: usize) -> BoxKind {
        self.kinds[index]
    }

    /// Returns the k-th color if it exists
    ///
    /// # Panics
    /// Panics if the index is out of bounds.
    pub fn get_color(&self, index: usize) -> Color {
        self.colors[index]
    }

    /// Returns the k-th multiplicity
    ///
    /// # Panics
    /// Panics if the index is out of bounds.
    pub fn get_multiplicity(&self, index: usize) -> Natural {
        self.multiplicities[index].clone()
    }

    /// Returns the k-th length
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

    /// Push a single row
    pub fn push(&mut self, kind: BoxKind, color: Color, multiplicity: impl Into<Natural>) {
        self.kinds.push(kind);
        self.colors.push(color);
        self.multiplicities.push(multiplicity.into());
        self.lengths.push(1);
        self.set_length(0, self.get_length(0) + 1);
    }

    /// Wrap a box in another box and apply the given multiplicity
    pub fn wrap<U: BoxType>(mut self, mul: impl Into<Natural>) -> BoxValue<U> {
        let prev_mul = self.get_multiplicity(0);
        self.set_multiplicity(0, mul.into() * prev_mul);

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
            colors: vec![Color::Red, Color::Black, Color::Black],
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
        let n = n.into();
        if n > 0 {
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
                    n,
                ],
                lengths: vec![4, 3, 2, 1],
                _marker: std::marker::PhantomData,
            }
        } else {
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
    }

    /// Construct the variable anti-beta
    pub fn anti_beta(n: impl Into<Natural>) -> Self {
        let n = n.into();
        if n > 0 {
            BoxValue {
                kinds: vec![
                    BoxKind::Multinum,
                    BoxKind::Polynum,
                    BoxKind::Num,
                    BoxKind::Empty,
                ],
                colors: vec![Color::Red, Color::Black, Color::Black, Color::Black],
                multiplicities: vec![
                    Natural::from(1_u32),
                    Natural::from(1_u32),
                    Natural::from(1_u32),
                    n,
                ],
                lengths: vec![4, 3, 2, 1],
                _marker: std::marker::PhantomData,
            }
        } else {
            BoxValue {
                kinds: vec![BoxKind::Polynum, BoxKind::Num, BoxKind::Empty],
                colors: vec![Color::Red, Color::Black, Color::Black],
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
}

#[derive(Clone)]
struct BoxContentKey(BoxValue<AnyBox>);

impl PartialEq for BoxContentKey {
    fn eq(&self, other: &Self) -> bool {
        self.0.is_eq_content(&other.0)
    }
}

impl Eq for BoxContentKey {}

impl Hash for BoxContentKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash_content(state);
    }
}

/// Color of a box
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Color {
    Red,
    Black,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_last_child() {
        let a = BoxValue::one() + BoxValue::alpha();
        let last = a.last_child();
        assert_eq!(last, BoxValue::zero());
    }

    #[test]
    fn test_ord() {
        let a = BoxValue::from(0_u32);
        let b = BoxValue::from(1_u32);
        assert!(a < b);

        let a = BoxValue::one();
        let b = BoxValue::alpha();
        assert!(a < b);

        let a = BoxValue::alpha();
        let b = 2 * BoxValue::alpha();
        assert!(a < b);

        let a = BoxValue::alpha();
        let b = BoxValue::one() + BoxValue::alpha();
        assert!(a < b);

        let a = BoxValue::alpha();
        let b = BoxValue::alpha() * BoxValue::alpha();
        assert!(a < b);

        let a = BoxValue::alpha();
        let b = BoxValue::beta(1_u32);
        assert!(a < b);

        let a = BoxValue::beta(1_u32);
        let b = BoxValue::beta(2_u32);
        assert!(a < b);

        let a = BoxValue::beta(2_u32);
        let b = BoxValue::beta(2_u32) * BoxValue::beta(2_u32);
        assert!(a < b);
    }
}
