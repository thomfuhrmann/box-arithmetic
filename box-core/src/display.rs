use crate::{BoxKind, BoxType, BoxValue, BoxVariant, BoxVariantIter, Color, store::BoxStore};
use colored::Colorize;
use malachite::Natural;
use std::{
    fmt::{Display, Formatter},
    hash::BuildHasher,
};

/// Display for [`BoxValue`] for debugging purposes
impl<T: BoxType> std::fmt::Display for BoxValue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Rows: {} | Kinds: {:?} | Colors: {:?} | Mults: {:?}",
            self.lengths.first().unwrap_or(&0),
            self.kinds,
            self.colors,
            self.multiplicities
        )
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub enum ColorMode {
    #[default]
    Markup,
    Terminal,
}

#[derive(Debug, Default, Clone, Copy)]
pub enum OutputFormat {
    #[default]
    Mixed,
    Boxed,
}

trait ColorizeToken {
    fn colorize_token(&self, mode: ColorMode, is_anti: bool) -> String;
}

impl ColorizeToken for str {
    fn colorize_token(&self, mode: ColorMode, is_anti: bool) -> String {
        match (mode, is_anti) {
            (ColorMode::Markup, true) => format!("<red>{}</red>", self),
            (ColorMode::Terminal, true) => self.red().to_string(),
            _ => self.to_string(),
        }
    }
}

fn open_bracket(kind: BoxKind) -> &'static str {
    match kind {
        BoxKind::Unixel | BoxKind::Pixel | BoxKind::List => "⌈",
        BoxKind::Set => "{",
        _ => "⌊",
    }
}

fn close_bracket(kind: BoxKind) -> &'static str {
    match kind {
        BoxKind::Unixel | BoxKind::Pixel | BoxKind::List => "⌉",
        BoxKind::Set => "}",
        _ => "⌋",
    }
}

/// Helper function to display multiplicities as subscripts
fn to_subscript(num: Natural) -> String {
    num.to_string()
        .chars()
        .map(|c| match c {
            '0' => '₀',
            '1' => '₁',
            '2' => '₂',
            '3' => '₃',
            '4' => '₄',
            '5' => '₅',
            '6' => '₆',
            '7' => '₇',
            '8' => '₈',
            '9' => '₉',
            _ => c,
        })
        .collect()
}

#[derive(Debug, Clone)]
pub struct BoxDisplay<'a> {
    pub value: BoxVariant,
    pub mode: ColorMode,
    pub format: OutputFormat,
    pub store: &'a BoxStore,
}

impl<'a> BoxDisplay<'a> {
    pub fn new(
        store: &'a BoxStore,
        value: BoxVariant,
        mode: ColorMode,
        format: OutputFormat,
    ) -> Self {
        Self {
            store,
            value,
            mode,
            format,
        }
    }

    pub fn set_mode(&mut self, mode: ColorMode) {
        self.mode = mode;
    }

    pub fn set_format(&mut self, format: OutputFormat) {
        self.format = format;
    }

    pub fn set_store(&mut self, store: &'a BoxStore) {
        self.store = store;
    }
}

impl<'a> BoxDisplay<'a> {
    pub fn from_variant(value: BoxVariant, store: &'a BoxStore) -> Self {
        BoxDisplay::new(store, value, ColorMode::default(), OutputFormat::default())
    }
}

#[derive(Debug)]
pub struct BoxDisplayIter<'a> {
    inner: BoxVariantIter,
    mode: ColorMode,
    format: OutputFormat,
    store: &'a BoxStore,
}

impl<'a> Iterator for BoxDisplayIter<'a> {
    type Item = BoxDisplay<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.inner.next();

        if let Some(value) = value {
            return Some(BoxDisplay {
                value,
                mode: self.mode,
                format: self.format,
                store: self.store,
            });
        }

        None
    }
}

impl<'a> IntoIterator for BoxDisplay<'a> {
    type Item = BoxDisplay<'a>;
    type IntoIter = BoxDisplayIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        BoxDisplayIter {
            inner: self.value.into_iter(),
            mode: self.mode,
            format: self.format,
            store: self.store,
        }
    }
}

fn box_display(box_display: &BoxDisplay, f: &mut Formatter<'_>) -> std::fmt::Result {
    let kind = box_display.value.get_kind(0);
    let is_anti = box_display.value.is_anti();
    let mode = box_display.mode;

    let open = open_bracket(kind).colorize_token(mode, is_anti);
    let close = close_bracket(kind).colorize_token(mode, is_anti);

    write!(f, "{}", open)?;

    let mut first = true;
    for child in box_display.clone() {
        if !first {
            write!(f, ",")?;
        }
        first = false;

        let len = child.value.get_length(0);
        let mult = child.value.get_multiplicity(0);
        if len > 1 {
            if f.alternate() {
                if mult > 1 {
                    write!(f, "{}", to_subscript(mult))?;
                }

                child.fmt(f)?;
            } else if let Ok(count) = usize::try_from(&mult) {
                for i in 0..count {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    child.fmt(f)?;
                }
            }
        } else {
            let symbol = "□".colorize_token(mode, child.value.is_anti());

            if f.alternate() {
                if mult > 1 {
                    write!(f, "{}", to_subscript(mult))?;
                }

                write!(f, "{}", symbol)?;
            } else if let Ok(count) = usize::try_from(&mult) {
                for i in 0..count {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    write!(f, "{}", symbol)?;
                }
            }
        }
    }

    write!(f, "{}", close)
}

fn write_alpha(
    f: &mut Formatter<'_>,
    child: BoxDisplay,
    name: String,
    first: bool,
) -> std::fmt::Result {
    let col = child.value.get_color(0);
    let mul = child.value.get_multiplicity(0);

    let op = if first {
        if col == Color::Red { "-" } else { "" }
    } else {
        if col == Color::Red { " - " } else { " + " }
    };

    let exp = child.value.get_multiplicity(1);
    if mul > 1 {
        write!(f, "{op}{mul}*{name}")?;
    } else {
        write!(f, "{op}{name}")?;
    };
    if exp > 1 {
        write!(f, "^{exp}")?;
    }

    Ok(())
}

fn write_num(f: &mut Formatter<'_>, child: BoxDisplay, first: bool) -> std::fmt::Result {
    let mul = child.value.get_multiplicity(0);
    let col = child.value.get_color(0);
    let op = if first {
        if col == Color::Red { "-" } else { "" }
    } else {
        if col == Color::Red { " - " } else { " + " }
    };
    write!(f, "{op}{mul}")
}

fn write_beta(
    f: &mut Formatter<'_>,
    child: BoxDisplay,
    name: String,
    first: bool,
) -> std::fmt::Result {
    let col = child.value.get_color(0);
    let mul = child.value.get_multiplicity(0);
    let op = if first {
        if col == Color::Red { "-" } else { "" }
    } else {
        if col == Color::Red { " - " } else { " + " }
    };
    let exp = child.value.get_multiplicity(1);
    let idx = child.value.get_multiplicity(2);
    if mul > 1 {
        write!(f, "{op}{mul}*{name}")?;
    } else {
        write!(f, "{op}{name}")?;
    };
    write!(f, "{}", to_subscript(idx))?;
    if exp > 1 {
        write!(f, "^{exp}")?;
    }

    Ok(())
}

fn mixed_display(box_display: &BoxDisplay, f: &mut Formatter<'_>) -> std::fmt::Result {
    let kind = box_display.value.get_kind(0);
    let is_anti = box_display.value.is_anti();
    let mode = box_display.mode;

    if kind == BoxKind::Empty {
        let zero = "0".colorize_token(mode, box_display.value.is_anti());
        return write!(f, "{}", zero);
    } else if kind == BoxKind::Num {
        let mult = box_display.value.get_multiplicity(1);
        let num = mult
            .to_string()
            .colorize_token(mode, box_display.value.is_anti());
        let empty_col = box_display.value.get_color(1);
        if empty_col == Color::Red {
            write!(f, "-")?;
        }
        return write!(f, "{}", num);
    } else if kind == BoxKind::Polynum {
        let alpha = BoxVariant::alpha();
        let hash = box_display.store.boxes.hasher().hash_one(alpha);
        let name = box_display.store.fetch_name(hash);
        let mut first = true;
        for mut child in box_display.clone() {
            let kind = child.value.get_kind(0);
            if kind == BoxKind::Num {
                if let Some(name) = name.clone() {
                    write_alpha(f, child, name.clone(), first)?;
                } else {
                    child.set_format(OutputFormat::Boxed);
                    write!(f, "{child:#}")?;
                }
            } else {
                write_num(f, child, first)?;
            }
            first = false;
        }
        return Ok(());
    } else if kind == BoxKind::Multinum {
        let beta = BoxVariant::beta(1_u32);
        let hash = box_display.store.boxes.hasher().hash_one(beta);
        let name_beta = box_display.store.fetch_name(hash);

        let alpha = BoxVariant::alpha();
        let hash = box_display.store.boxes.hasher().hash_one(alpha);
        let name_alpha = box_display.store.fetch_name(hash);
        let mut first = true;
        for mut child in box_display.clone() {
            let kind = child.value.get_kind(0);
            if kind == BoxKind::Polynum {
                if let Some(name_beta) = name_beta.clone() {
                    write_beta(f, child, name_beta.clone(), first)?;
                } else {
                    child.set_format(OutputFormat::Boxed);
                    write!(f, "{child:#}")?;
                }
            } else if kind == BoxKind::Num {
                if let Some(name_alpha) = name_alpha.clone() {
                    write_alpha(f, child, name_alpha.clone(), first)?;
                } else {
                    child.set_format(OutputFormat::Boxed);
                    write!(f, "{child:#}")?;
                }
            } else {
                write_num(f, child, first)?;
            }
            first = false;
        }
        return Ok(());
    }

    let open = open_bracket(kind).colorize_token(mode, is_anti);
    let close = close_bracket(kind).colorize_token(mode, is_anti);

    write!(f, "{}", open)?;
    let mut first = true;
    for child in box_display.clone() {
        if !first {
            write!(f, ",")?;
        }
        first = false;

        let mult: Natural = child.value.get_multiplicity(0);
        if f.alternate() {
            if mult > 1 {
                write!(f, "{}", to_subscript(mult))?;
            }

            child.fmt(f)?;
        } else if let Ok(count) = usize::try_from(&mult) {
            for i in 0..count {
                if i > 0 {
                    write!(f, ",")?;
                }
                child.fmt(f)?;
            }
        }
    }

    write!(f, "{}", close)
}

impl<'a> Display for BoxDisplay<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let format = self.format;

        match format {
            OutputFormat::Boxed => box_display(self, f),
            OutputFormat::Mixed => mixed_display(self, f),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        BoxValue, BoxVariant,
        display::{BoxDisplay, ColorMode, OutputFormat},
        maxel,
        store::BoxStore,
        vexel,
    };

    #[test]
    fn test_display() {
        let mut store = BoxStore::new();
        let alpha = BoxVariant::alpha();
        store.store_with_name("α", alpha);

        let minus_two = BoxDisplay::from_variant(BoxVariant::from(-2), &store);
        println!("{minus_two}");
        println!("{minus_two:#}");

        let mut minus_two = BoxDisplay::from_variant(BoxVariant::from(-2), &store);
        minus_two.set_mode(ColorMode::Terminal);
        minus_two.set_format(OutputFormat::Boxed);
        println!("{minus_two}");
        println!("{minus_two:#}");

        let sum = BoxVariant::from(3) + BoxVariant::from(-2);
        let disp = BoxDisplay::from_variant(sum, &store);
        println!("{disp}");
        println!("{disp:#}");

        let alpha = BoxVariant::alpha();
        let disp = BoxDisplay::from_variant(alpha.clone(), &store);
        println!("{disp}");
        println!("{disp:#}");

        let poly = BoxVariant::from(-2)
            + 2_u32 * BoxVariant::alpha()
            + BoxVariant::alpha() * BoxVariant::alpha();
        let disp = BoxDisplay::from_variant(poly, &store);
        println!("{disp}");
        println!("{disp:#}");

        let anti_box = BoxVariant::from(1).into_anti();
        let disp = BoxDisplay::from_variant(anti_box, &store);
        println!("{disp}");
        println!("{disp:#}");

        let anti_box = BoxVariant::from(1).into_anti();
        let mut disp = BoxDisplay::from_variant(anti_box, &store);
        disp.set_mode(ColorMode::Terminal);
        println!("{disp}");
        println!("{disp:#}");

        let a = maxel![[[1, 1], [1, 2], [2, 2], [2, 2]]];
        let disp = BoxDisplay::from_variant(a, &store);
        println!("{disp}");
        println!("{disp:#}");

        let a = vexel![[1, 2, 3, 3]];
        let disp = BoxDisplay::from_variant(a, &store);
        println!("{disp}");
        println!("{disp:#}");
    }

    #[test]
    fn test_display_multi() {
        let mut store = BoxStore::new();
        let alpha = BoxVariant::alpha();
        store.store_with_name("α", alpha);
        store.store_with_name("β", BoxValue::beta(1_u32));

        let beta = BoxVariant::beta(0_u32);
        let disp = BoxDisplay::from_variant(beta.clone(), &store);
        println!("{disp}");
        println!("{disp:#}");

        let beta = BoxVariant::beta(1_u32);
        let mut disp = BoxDisplay::from_variant(beta.clone(), &store);
        println!("{disp}");
        println!("{disp:#}");

        disp.set_format(OutputFormat::Boxed);
        println!("{disp}");
        println!("{disp:#}");

        let beta = BoxVariant::beta(2_u32);
        let mut disp = BoxDisplay::from_variant(beta.clone(), &store);
        println!("{disp}");
        println!("{disp:#}");

        disp.set_format(OutputFormat::Boxed);
        println!("{disp}");
        println!("{disp:#}");

        let multi = BoxVariant::beta(2_u32) + BoxVariant::from(1) + BoxVariant::alpha();
        let disp = BoxDisplay::from_variant(multi.clone(), &store);
        println!("{disp}");
        println!("{disp:#}");
    }
}
