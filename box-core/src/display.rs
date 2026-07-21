use crate::{AnyBox, BoxKind, BoxType, BoxValue, BoxVariant};
use colored::Colorize;
use malachite::Natural;
use std::fmt::Display;

#[derive(Debug, Default, Clone, Copy)]
pub enum DisplayMode {
    #[default]
    Markup,
    Terminal,
}

trait ColorizeToken {
    fn colorize_token(&self, mode: DisplayMode, is_anti: bool) -> String;
}

impl ColorizeToken for str {
    fn colorize_token(&self, mode: DisplayMode, is_anti: bool) -> String {
        match (mode, is_anti) {
            (DisplayMode::Markup, true) => format!("<red>{}</red>", self),
            (DisplayMode::Terminal, true) => self.red().to_string(),
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

impl Display for BoxVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kind = self.get_kind(0);
        let is_anti = self.is_anti();
        let mode = DisplayMode::Markup;

        if kind == BoxKind::Empty {
            let zero = "0".colorize_token(mode, self.is_anti());
            return write!(f, "{}", zero);
        } else if kind == BoxKind::Num {
            let mult = self.get_multiplicity(1);
            let num = mult.to_string().colorize_token(mode, self.is_anti());
            return write!(f, "{}", num);
        }

        let open = open_bracket(kind).colorize_token(mode, is_anti);
        let close = close_bracket(kind).colorize_token(mode, is_anti);

        write!(f, "{}", open)?;
        let mut first = true;
        for child in self.clone() {
            if !first {
                write!(f, ",")?;
            }
            first = false;

            let mult: Natural = child.get_multiplicity(0);
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
}

#[derive(Debug)]
pub struct BoxDisplay<T: BoxType> {
    pub value: BoxValue<T>,
    pub mode: DisplayMode,
}

impl<T: BoxType> BoxDisplay<T> {
    pub fn new(value: BoxValue<T>, mode: DisplayMode) -> Self {
        Self { value, mode }
    }
}

impl<'a> From<&'a BoxVariant> for BoxDisplay<AnyBox> {
    fn from(value: &'a BoxVariant) -> Self {
        let raw_any = value.clone().into_any_raw();
        BoxDisplay::new(raw_any, DisplayMode::default())
    }
}

impl<T: BoxType> Display for BoxDisplay<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kind = self.value.kind();
        let is_anti = self.value.is_anti();
        let mode = self.mode;

        let open = open_bracket(kind).colorize_token(mode, is_anti);
        let close = close_bracket(kind).colorize_token(mode, is_anti);

        write!(f, "{}", open)?;

        let mut first = true;
        for child in self.value.clone() {
            if !first {
                write!(f, ",")?;
            }
            first = false;

            let len = child.get_length(0);
            let mult = child.get_multiplicity(0);
            if len > 1 {
                let child = BoxDisplay::new(child, mode);
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
                let symbol = "□".colorize_token(mode, child.is_anti());

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
}

#[cfg(test)]
mod tests {

    use crate::{AnyBox, BoxVariant, display::BoxDisplay, maxel, vexel};

    #[test]
    fn test_display() {
        let three = BoxDisplay::<AnyBox>::from(&BoxVariant::from(3));
        println!("{three}");
        println!("{three:#}");

        let three = BoxVariant::from(3);
        println!("{three}");
        println!("{three:#}");

        let minus_two = BoxVariant::from(-2);
        println!("{minus_two}");
        println!("{minus_two:#}");

        let sum = three + minus_two.clone();
        println!("{sum}");
        println!("{sum:#}");

        let alpha = BoxVariant::alpha();
        println!("{alpha}");
        println!("{alpha:#}");

        let poly = minus_two + 2_u32 * alpha + BoxVariant::alpha() * BoxVariant::alpha();
        println!("{poly}");
        println!("{poly:#}");

        let anti_box = BoxVariant::from(1).into_anti();
        println!("{anti_box}");
        println!("{anti_box:#}");

        let a = maxel![[[1, 1], [1, 2], [2, 2], [2, 2]]];
        // let a = BoxDisplay::<AnyBox>::from(&a);
        println!("{a}");
        println!("{a:#}");

        let a = vexel![[1, 2, 3, 3]];
        println!("{a}");
        println!("{a:#}");

        let a = vexel![[1, 2, 3, 3]];
        let a = BoxDisplay::<AnyBox>::from(&a);
        println!("{a}");
        println!("{a:#}");
    }
}
