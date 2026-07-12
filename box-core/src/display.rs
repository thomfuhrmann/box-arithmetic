use std::fmt::Display;

use colored::Colorize;
use malachite::Natural;

use crate::{AnyBox, BoxKind, BoxType, BoxValue, BoxVariant};

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
        if kind == BoxKind::Empty {
            let zero = if self.is_anti() {
                "0".red()
            } else {
                "0".black()
            };

            return write!(f, "{}", zero);
        } else if kind == BoxKind::Num {
            let mult = self.get_multiplicity(1);
            let num = if self.is_anti() {
                mult.to_string().red()
            } else {
                mult.to_string().black()
            };

            return write!(f, "{}", num);
        }

        let open_bracket = if kind == BoxKind::Unixel || kind == BoxKind::Pixel {
            "⌈"
        } else {
            "⌊"
        };

        let close_bracket = if kind == BoxKind::Unixel || kind == BoxKind::Pixel {
            "⌉"
        } else {
            "⌋"
        };

        let open = if self.is_anti() {
            open_bracket.red()
        } else {
            open_bracket.black()
        };

        let close = if self.is_anti() {
            close_bracket.red()
        } else {
            close_bracket.black()
        };

        write!(f, "{}", open)?;
        let mut first = true;
        for child in self.clone() {
            if !first {
                write!(f, ",")?;
            }
            first = false;

            let mult = child.get_multiplicity(0);
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
pub struct BoxDisplay<T: BoxType>(pub(crate) BoxValue<T>);

impl<T: BoxType> BoxDisplay<T> {
    pub fn new(variant: BoxValue<T>) -> Self {
        Self(variant)
    }
}

impl<'a> From<&'a BoxVariant> for BoxDisplay<AnyBox> {
    fn from(value: &'a BoxVariant) -> Self {
        let raw_any = value.clone().into_any_raw();
        BoxDisplay::new(raw_any)
    }
}

impl<T: BoxType> Display for BoxDisplay<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kind = self.0.get_kind(0);

        let open_bracket = if kind == BoxKind::Unixel || kind == BoxKind::Pixel {
            "⌈"
        } else {
            "⌊"
        };

        let close_bracket = if kind == BoxKind::Unixel || kind == BoxKind::Pixel {
            "⌉"
        } else {
            "⌋"
        };

        let open = if self.0.is_anti() {
            open_bracket.red()
        } else {
            open_bracket.black()
        };

        let close = if self.0.is_anti() {
            close_bracket.red()
        } else {
            close_bracket.black()
        };

        write!(f, "{}", open)?;

        let mut first = true;
        for child in self.0.clone() {
            if !first {
                write!(f, ",")?;
            }
            first = false;

            let len = child.get_length(0);
            let mult = child.get_multiplicity(0);
            if len > 1 {
                let child = BoxDisplay::new(child);
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
                let symbol = if child.is_anti() {
                    "□".red()
                } else {
                    "□".black()
                };

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
