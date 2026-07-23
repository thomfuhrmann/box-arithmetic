use crate::{BoxKind, BoxType, BoxValue, BoxVariant, BoxVariantIter, Color, store::BoxStore};
use colored::Colorize;
use malachite::Natural;
use std::{
    fmt::{Display, Formatter},
    hash::BuildHasher,
    sync::Arc,
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
pub struct BoxDisplay {
    pub value: BoxVariant,
    pub mode: ColorMode,
    pub format: OutputFormat,
    pub store: Arc<BoxStore>,
}

impl BoxDisplay {
    pub fn new(
        store: Arc<BoxStore>,
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

    pub fn set_store(&mut self, store: Arc<BoxStore>) {
        self.store = store;
    }
}

impl From<BoxVariant> for BoxDisplay {
    fn from(value: BoxVariant) -> Self {
        BoxDisplay::new(
            Arc::new(BoxStore::default()),
            value,
            ColorMode::default(),
            OutputFormat::default(),
        )
    }
}

#[derive(Debug)]
pub struct BoxDisplayIter {
    inner: BoxVariantIter,
    mode: ColorMode,
    format: OutputFormat,
    store: Arc<BoxStore>,
}

impl Iterator for BoxDisplayIter {
    type Item = BoxDisplay;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.inner.next();

        if let Some(value) = value {
            return Some(BoxDisplay {
                value,
                mode: self.mode,
                format: self.format,
                store: self.store.clone(),
            });
        }

        None
    }
}

impl IntoIterator for BoxDisplay {
    type Item = BoxDisplay;

    type IntoIter = BoxDisplayIter;

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
        if let Some(name) = name {
            let mut first = true;
            for child in box_display.clone() {
                let kind = child.value.get_kind(0);
                let col = child.value.get_color(0);
                let mul = child.value.get_multiplicity(0);
                let op = if first {
                    if col == Color::Red { "-" } else { "" }
                } else {
                    if col == Color::Red { " - " } else { " + " }
                };
                if kind == BoxKind::Num {
                    let exp = child.value.get_multiplicity(1);
                    if mul > 1 {
                        write!(f, "{op}{mul}*{name}")?;
                    } else {
                        write!(f, "{op}{name}")?;
                    };
                    if exp > 1 {
                        write!(f, "^{exp}")?;
                    }
                } else {
                    write!(f, "{op}{mul}")?;
                }
                first = false;
            }
            return write!(f, "");
        }
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

impl Display for BoxDisplay {
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
    use std::sync::Arc;

    use crate::{
        BoxVariant,
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
        let arc_store = Arc::new(store);

        let minus_two = BoxDisplay::from(BoxVariant::from(-2));
        println!("{minus_two}");
        println!("{minus_two:#}");

        let mut minus_two = BoxDisplay::from(BoxVariant::from(-2));
        minus_two.set_mode(ColorMode::Terminal);
        minus_two.set_format(OutputFormat::Boxed);
        println!("{minus_two}");
        println!("{minus_two:#}");

        let sum = BoxVariant::from(3) + BoxVariant::from(-2);
        let disp = BoxDisplay::from(sum);
        println!("{disp}");
        println!("{disp:#}");

        let alpha = BoxVariant::alpha();
        let mut disp = BoxDisplay::from(alpha.clone());
        disp.set_store(arc_store.clone());
        println!("{disp}");
        println!("{disp:#}");

        let poly = BoxVariant::from(-2)
            + 2_u32 * BoxVariant::alpha()
            + BoxVariant::alpha() * BoxVariant::alpha();
        let mut disp = BoxDisplay::from(poly);
        disp.set_store(arc_store.clone());
        println!("{disp}");
        println!("{disp:#}");

        let anti_box = BoxVariant::from(1).into_anti();
        let disp = BoxDisplay::from(anti_box);
        println!("{disp}");
        println!("{disp:#}");

        let anti_box = BoxVariant::from(1).into_anti();
        let mut disp = BoxDisplay::from(anti_box);
        disp.set_mode(ColorMode::Terminal);
        println!("{disp}");
        println!("{disp:#}");

        let a = maxel![[[1, 1], [1, 2], [2, 2], [2, 2]]];
        let disp = BoxDisplay::from(a);
        println!("{disp}");
        println!("{disp:#}");

        let a = vexel![[1, 2, 3, 3]];
        let disp = BoxDisplay::from(a);
        println!("{disp}");
        println!("{disp:#}");
    }
}
