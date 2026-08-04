use crate::{AnyBox, BoxKind, BoxValue, BoxVariant, Color, NumBox, store::BoxStore};

use chumsky::{prelude::*, util::MaybeRef};
use logos::{Lexer, Logos};
use malachite::Natural;

// TODO: parse superscripts

fn parse_subscript(lex: &mut Lexer<Token>) -> Option<Natural> {
    let slice = lex.slice();
    let mut result = Natural::from(0_u32);

    for ch in slice.chars() {
        let digit: u32 = match ch {
            '₀' => 0,
            '₁' => 1,
            '₂' => 2,
            '₃' => 3,
            '₄' => 4,
            '₅' => 5,
            '₆' => 6,
            '₇' => 7,
            '₈' => 8,
            '₉' => 9,
            _ => return None,
        };

        let digit = Natural::from(digit);
        let base = Natural::from(10_u32);

        result = base * result + digit;
    }

    Some(result)
}

fn parse_superscript(lex: &mut Lexer<Token>) -> Option<Natural> {
    let slice = lex.slice();
    let mut result = Natural::from(0_u32);

    for ch in slice.chars() {
        let digit: u32 = match ch {
            '⁰' => 0,
            '¹' => 1,
            '²' => 2,
            '³' => 3,
            '⁴' => 4,
            '⁵' => 5,
            '⁶' => 6,
            '⁷' => 7,
            '⁸' => 8,
            '⁹' => 9,
            _ => return None,
        };

        let digit = Natural::from(digit);
        let base = Natural::from(10_u32);

        result = base * result + digit;
    }

    Some(result)
}

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    #[token("□")]
    Empty,
    #[regex(r"[0-9]+", |lex|lex.slice().parse())]
    Num(Natural),
    #[regex(r"[\p{Greek}a-zA-Z_][\p{Greek}a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Var(String),
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Multiply,
    #[token("/")]
    Divide,
    #[token("%")]
    Remainder,
    #[token("^")]
    Caret,
    #[token("∩")]
    Intersection,
    #[token("∪")]
    Union,
    #[token("(")]
    OpenGroup,
    #[token(")")]
    CloseGroup,
    #[token("⌊")]
    OpenBox,
    #[token("⌋")]
    CloseBox,
    #[token("⌈")]
    OpenList,
    #[token("⌉")]
    CloseList,
    #[token("{")]
    OpenSet,
    #[token("}")]
    CloseSet,
    #[token(",")]
    Comma,
    #[regex(r"[₀₁₂₃₄₅₆₇₈₉]+", parse_subscript)]
    Subscript(Natural),
    #[regex("[⁰¹²³⁴⁵⁶⁷⁸⁹]+", parse_superscript)]
    Superscript(Natural),
    #[token("<red>")]
    RedOpen,
    #[token("</red>")]
    RedClose,
    #[token("der")]
    Der,
}

#[derive(Debug, Clone, PartialEq)]
pub enum IndexPos {
    Pre,
    Post,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Empty,
    Num(Natural),
    Var(String),
    Neg(Box<Expr>),
    Subscript {
        pos: IndexPos,
        expr: Box<Expr>,
        index: Natural,
    },
    Anti(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Rem(Box<Expr>, Box<Expr>),
    Caret(Box<Expr>, Natural),
    Intersection(Box<Expr>, Box<Expr>),
    Union(Box<Expr>, Box<Expr>),
    Box(Vec<Expr>),
    Set(Vec<Expr>),
    List(Vec<Expr>),
    Unixel(Box<Expr>),
    Vexel(Vec<Expr>),
    Pixel(Box<Expr>, Box<Expr>),
    Maxel(Vec<Expr>),
    Der(Box<Expr>),
    DerMulti(Box<Expr>, Natural),
}

fn subscript<'a>() -> impl Parser<'a, &'a [Token], Natural, extra::Err<Simple<'a, Token>>> + Clone {
    select! {
        Token::Subscript(num) => num,
    }
}

fn superscript<'a>() -> impl Parser<'a, &'a [Token], Natural, extra::Err<Simple<'a, Token>>> + Clone
{
    select! {
        Token::Superscript(num) => num,
    }
}

fn colored_token<'a>(
    token: Token,
) -> Boxed<'a, 'a, &'a [Token], Color, extra::Err<Simple<'a, Token>>> {
    just(token.clone())
        .to(Color::Black)
        .or(just(Token::RedOpen)
            .ignore_then(just(token))
            .then_ignore(just(Token::RedClose))
            .to(Color::Red))
        .boxed()
}

fn open_box<'a>() -> Boxed<'a, 'a, &'a [Token], Color, extra::Err<Simple<'a, Token>>> {
    colored_token(Token::OpenBox)
}

fn close_box<'a>() -> Boxed<'a, 'a, &'a [Token], Color, extra::Err<Simple<'a, Token>>> {
    colored_token(Token::CloseBox)
}

fn open_list<'a>() -> Boxed<'a, 'a, &'a [Token], Color, extra::Err<Simple<'a, Token>>> {
    colored_token(Token::OpenList)
}

fn close_list<'a>() -> Boxed<'a, 'a, &'a [Token], Color, extra::Err<Simple<'a, Token>>> {
    colored_token(Token::CloseList)
}

fn open_set<'a>() -> Boxed<'a, 'a, &'a [Token], Color, extra::Err<Simple<'a, Token>>> {
    colored_token(Token::OpenSet)
}

fn close_set<'a>() -> Boxed<'a, 'a, &'a [Token], Color, extra::Err<Simple<'a, Token>>> {
    colored_token(Token::CloseSet)
}

fn box_parser<'a, P>(parser: P) -> Boxed<'a, 'a, &'a [Token], Expr, extra::Err<Simple<'a, Token>>>
where
    P: Parser<'a, &'a [Token], Expr, extra::Err<Simple<'a, Token>>> + Clone + 'a,
{
    let items = parser.separated_by(just(Token::Comma)).collect::<Vec<_>>();

    subscript()
        .or_not()
        .then(open_box())
        .then(items)
        .then(close_box())
        .validate(
            |(((outer_sub, open_color), items), close_color), e, emitter| {
                if open_color != close_color {
                    emitter.emit(Simple::new(None, e.span()));
                }

                let base_box = if open_color == Color::Red {
                    Expr::Anti(Box::new(Expr::Box(items)))
                } else {
                    Expr::Box(items)
                };

                match outer_sub {
                    Some(index) => Expr::Subscript {
                        pos: IndexPos::Pre,
                        expr: Box::new(base_box),
                        index,
                    },
                    None => base_box,
                }
            },
        )
        .boxed()
}

fn list_parser<'a, P>(parser: P) -> Boxed<'a, 'a, &'a [Token], Expr, extra::Err<Simple<'a, Token>>>
where
    P: Parser<'a, &'a [Token], Expr, extra::Err<Simple<'a, Token>>> + Clone + 'a,
{
    let items = parser.separated_by(just(Token::Comma)).collect::<Vec<_>>();

    subscript()
        .or_not()
        .then(open_list())
        .then(items)
        .then(close_list())
        .validate(
            |(((outer_sub, open_color), items), close_color), e, emitter| {
                if open_color != close_color {
                    emitter.emit(Simple::new(None, e.span()));
                }

                let base_box = if open_color == Color::Red {
                    Expr::Anti(Box::new(Expr::List(items)))
                } else {
                    Expr::List(items)
                };

                match outer_sub {
                    Some(index) => Expr::Subscript {
                        pos: IndexPos::Pre,
                        expr: Box::new(base_box),
                        index,
                    },
                    None => base_box,
                }
            },
        )
        .boxed()
}

fn set_parser<'a, P>(parser: P) -> Boxed<'a, 'a, &'a [Token], Expr, extra::Err<Simple<'a, Token>>>
where
    P: Parser<'a, &'a [Token], Expr, extra::Err<Simple<'a, Token>>> + Clone + 'a,
{
    let items = parser.separated_by(just(Token::Comma)).collect::<Vec<_>>();

    subscript()
        .or_not()
        .then(open_set())
        .then(items)
        .then(close_set())
        .validate(
            |(((outer_sub, open_color), items), close_color), e, emitter| {
                if open_color != close_color {
                    emitter.emit(Simple::new(None, e.span()));
                }

                let base_box = if open_color == Color::Red {
                    Expr::Anti(Box::new(Expr::Set(items)))
                } else {
                    Expr::Set(items)
                };

                match outer_sub {
                    Some(index) => Expr::Subscript {
                        pos: IndexPos::Pre,
                        expr: Box::new(base_box),
                        index,
                    },
                    None => base_box,
                }
            },
        )
        .boxed()
}

fn vexel_parser<'a, P>(parser: P) -> Boxed<'a, 'a, &'a [Token], Expr, extra::Err<Simple<'a, Token>>>
where
    P: Parser<'a, &'a [Token], Expr, extra::Err<Simple<'a, Token>>> + Clone + 'a,
{
    let unixel_with_subscript = subscript()
        .or_not()
        .then(open_list())
        .then(parser)
        .then(close_list())
        .validate(
            |(((outer_sub, open_color), item), close_color), e, emitter| {
                if open_color != close_color {
                    emitter.emit(Simple::new(None, e.span()));
                }

                let base_box = if open_color == Color::Red {
                    Expr::Anti(Box::new(Expr::Unixel(Box::new(item))))
                } else {
                    Expr::Unixel(Box::new(item))
                };

                match outer_sub {
                    Some(index) => Expr::Subscript {
                        pos: IndexPos::Pre,
                        expr: Box::new(base_box),
                        index,
                    },
                    None => base_box,
                }
            },
        );

    box_parser(unixel_with_subscript).boxed()
}

fn maxel_parser<'a, P>(parser: P) -> Boxed<'a, 'a, &'a [Token], Expr, extra::Err<Simple<'a, Token>>>
where
    P: Parser<'a, &'a [Token], Expr, extra::Err<Simple<'a, Token>>> + Clone + 'a,
{
    let values = parser
        .clone()
        .then_ignore(just(Token::Comma))
        .then(parser)
        .map(|(left, right)| Expr::Pixel(Box::new(left), Box::new(right)));

    let pixel_with_subscript = subscript()
        .or_not()
        .then(open_list())
        .then(values)
        .then(close_list())
        .validate(
            |(((outer_sub, open_color), pix), close_color), e, emitter| {
                if open_color != close_color {
                    emitter.emit(Simple::new(Some(MaybeRef::Val(Token::CloseBox)), e.span()));
                }

                let base_box = if open_color == Color::Red {
                    Expr::Anti(Box::new(pix))
                } else {
                    pix
                };

                match outer_sub {
                    Some(index) => Expr::Subscript {
                        pos: IndexPos::Pre,
                        expr: Box::new(base_box),
                        index,
                    },
                    None => base_box,
                }
            },
        );

    box_parser(pixel_with_subscript).boxed()
}

fn der_parser<'a, P>(parser: P) -> Boxed<'a, 'a, &'a [Token], Expr, extra::Err<Simple<'a, Token>>>
where
    P: Parser<'a, &'a [Token], Expr, extra::Err<Simple<'a, Token>>> + Clone + 'a,
{
    just(Token::Der)
        .ignore_then(
            parser
                .delimited_by(just(Token::OpenGroup), just(Token::CloseGroup))
                .map(|expr| Expr::Der(Box::new(expr))),
        )
        .boxed()
}

fn der_multi_parser<'a, P>(
    parser: P,
) -> Boxed<'a, 'a, &'a [Token], Expr, extra::Err<Simple<'a, Token>>>
where
    P: Parser<'a, &'a [Token], Expr, extra::Err<Simple<'a, Token>>> + Clone + 'a,
{
    let num = select! { Token::Num(num) => num };

    just(Token::Der)
        .ignore_then(
            parser
                .then_ignore(just(Token::Comma))
                .then(num)
                .delimited_by(just(Token::OpenGroup), just(Token::CloseGroup)),
        )
        .map(|(val, num)| Expr::DerMulti(Box::new(val), num))
        .boxed()
}

pub fn parser<'src>() -> Boxed<'src, 'src, &'src [Token], Expr, extra::Err<Simple<'src, Token>>> {
    recursive(|p| {
        let just_num = select! { Token::Num(num) => Expr::Num(num) };

        let number = just_num
            .delimited_by(just(Token::RedOpen), just(Token::RedClose))
            .map(|expr| Expr::Anti(Box::new(expr)))
            .or(just_num);

        let empty_box = colored_token(Token::Empty).map(|col| match col {
            Color::Black => Expr::Empty,
            Color::Red => Expr::Anti(Box::new(Expr::Empty)),
        });

        let just_var = select! { Token::Var(name) => Expr::Var(name) };
        let base_or_anti = just_var
            .delimited_by(just(Token::RedOpen), just(Token::RedClose))
            .map(|expr| Expr::Anti(Box::new(expr)))
            .or(just_var);
        let var = base_or_anti
            .then(subscript().or_not())
            .map(|(expr, sub)| match sub {
                Some(index) => Expr::Subscript {
                    pos: IndexPos::Post,
                    expr: Box::new(expr),
                    index,
                },
                None => expr,
            });

        let parenthesized = p
            .clone()
            .delimited_by(just(Token::OpenGroup), just(Token::CloseGroup));

        let base_atom = number
            .or(empty_box)
            .or(var)
            .or(vexel_parser(p.clone()))
            .or(maxel_parser(p.clone()))
            .or(list_parser(p.clone()))
            .or(box_parser(p.clone()))
            .or(parenthesized)
            .boxed();

        let atom = just(Token::Minus)
            .repeated()
            .collect::<Vec<_>>()
            .then(subscript().or_not())
            .then(base_atom)
            .map(|((minuses, num), expr)| {
                let expr = match num {
                    Some(index) => Expr::Subscript {
                        pos: IndexPos::Pre,
                        expr: Box::new(expr),
                        index,
                    },
                    None => expr,
                };
                minuses
                    .into_iter()
                    .fold(expr, |acc, _| Expr::Neg(Box::new(acc)))
            })
            .boxed();

        let caret = atom
            .clone()
            .then(
                just(Token::Caret)
                    .ignore_then(select! { Token::Num(n) => n })
                    .or(superscript())
                    .or_not(),
            )
            .map(|(base, exp)| match exp {
                Some(n) => Expr::Caret(Box::new(base), n),
                None => base,
            });

        let prod = caret
            .clone()
            .foldl(
                just(Token::Multiply)
                    .or(just(Token::Divide))
                    .or(just(Token::Remainder))
                    .then(caret)
                    .repeated(),
                |lhs, (op, rhs)| match op {
                    Token::Multiply => Expr::Mul(Box::new(lhs), Box::new(rhs)),
                    Token::Divide => Expr::Div(Box::new(lhs), Box::new(rhs)),
                    Token::Remainder => Expr::Rem(Box::new(lhs), Box::new(rhs)),
                    _ => unreachable!(),
                },
            )
            .boxed();

        let sum = prod
            .clone()
            .foldl(
                just(Token::Plus)
                    .or(just(Token::Minus))
                    .then(prod)
                    .repeated(),
                |lhs, (op, rhs)| match op {
                    Token::Plus => Expr::Add(Box::new(lhs), Box::new(rhs)),
                    Token::Minus => Expr::Sub(Box::new(lhs), Box::new(rhs)),
                    _ => unreachable!(),
                },
            )
            .boxed();

        let set = set_parser(p.clone());
        let set_op = set
            .clone()
            .foldl(
                just(Token::Union)
                    .or(just(Token::Intersection))
                    .then(set)
                    .repeated(),
                |lhs, (op, rhs)| match op {
                    Token::Union => Expr::Union(Box::new(lhs), Box::new(rhs)),
                    Token::Intersection => Expr::Intersection(Box::new(lhs), Box::new(rhs)),
                    _ => unreachable!(),
                },
            )
            .boxed();

        set_op
            .or(sum.clone())
            .or(der_parser(sum.clone()))
            .or(der_multi_parser(sum))
            .boxed()
    })
    .boxed()
}

impl Expr {
    pub fn eval(&self, store: &BoxStore) -> BoxVariant {
        match self {
            Expr::Empty => BoxVariant::Empty(BoxValue::zero()),
            Expr::Anti(v) => v.eval(store).into_anti(),
            Expr::Num(n) => BoxVariant::Num(BoxValue::from(n.clone())),
            Expr::Var(name) => store
                .fetch_box_by_name(name)
                .expect("Undefined Var assignment"),
            Expr::Subscript { pos, expr, index } => {
                let mut val = expr.eval(store);
                if *pos == IndexPos::Pre {
                    val.set_multiplicity(0, index.clone());
                } else {
                    if val.get_kind(0) == BoxKind::Multinum {
                        val.set_multiplicity(3, index.clone());
                    }
                }
                val
            }
            Expr::Neg(rhs) => BoxVariant::Num(BoxValue::from(-1)) * rhs.eval(store),
            Expr::Add(lhs, rhs) => lhs.eval(store) + rhs.eval(store),
            Expr::Mul(lhs, rhs) => lhs.eval(store) * rhs.eval(store),
            Expr::Sub(lhs, rhs) => {
                lhs.eval(store) + BoxVariant::Num(BoxValue::from(-1)) * rhs.eval(store)
            }
            Expr::Div(lhs, rhs) => lhs.eval(store) / rhs.eval(store),
            Expr::Rem(lhs, rhs) => lhs.eval(store) % rhs.eval(store),
            Expr::Caret(v, n) => {
                let variant = v.eval(store);
                if *n == 0 {
                    return BoxVariant::Num(BoxValue::one());
                }

                let mut acc = variant.clone();
                let mut i = n.clone();
                let one = malachite::Natural::from(1u32);

                // Multiply (n - 1) times
                while i > one {
                    acc = acc * variant.clone();
                    i -= &one;
                }

                acc
            }
            Expr::Intersection(lhs, rhs) => {
                BoxVariant::intersection(lhs.eval(store), rhs.eval(store))
            }
            Expr::Union(lhs, rhs) => BoxVariant::union(lhs.eval(store), rhs.eval(store)),
            Expr::Unixel(x) => BoxVariant::Unixel(BoxValue::unixel(x.eval(store).into_any_raw())),
            Expr::Vexel(xs) => {
                let mut vs = Vec::new();
                for x in xs {
                    let variant = x.eval(store);
                    match variant {
                        BoxVariant::Unixel(v) => vs.push(v),
                        _ => unreachable!(),
                    }
                }
                BoxVariant::Vexel(vs.into())
            }
            Expr::Pixel(x, y) => BoxVariant::Pixel(BoxValue::pixel(
                x.eval(store).into_any_raw(),
                y.eval(store).into_any_raw(),
            )),
            Expr::Maxel(pxs) => {
                let mut vs = Vec::new();
                for px in pxs {
                    let variant = px.eval(store);
                    match variant {
                        BoxVariant::Pixel(px) => vs.push(px),
                        _ => unreachable!(),
                    }
                }

                BoxVariant::Maxel(vs.into())
            }
            Expr::Box(bxs) => {
                let mut vs = Vec::new();
                for bx in bxs {
                    let var = bx.eval(store);
                    vs.push(var.into_any_raw());
                }

                // structural type inference
                let is_num = vs.iter().all(|v| v.get_kind(0) == BoxKind::Empty);
                if is_num {
                    let len = vs.len();
                    if len == 1 {
                        let mul = vs[0].get_multiplicity(0);
                        let col = vs[0].get_color(0);
                        let mut num: BoxValue<NumBox> = mul.into();
                        if col == Color::Red {
                            num.set_color(1, Color::Red);
                        }
                        return BoxVariant::Num(num);
                    } else {
                        let sum = vs.iter().fold(0, |mut acc, v| {
                            if v.get_color(0) == Color::Red {
                                acc -= 1;
                            } else {
                                acc += 1;
                            }
                            acc
                        });
                        let mut num: BoxValue<NumBox> = sum.into();
                        if sum < 0 {
                            num.set_color(1, Color::Red);
                        }
                        return BoxVariant::Num(num);
                    }
                }

                let is_polynum = vs
                    .iter()
                    .all(|v| v.get_kind(0) == BoxKind::Num || v.get_kind(0) == BoxKind::Empty);
                if is_polynum {
                    let mut poly: BoxValue<AnyBox> = vs.into();
                    poly.set_kind(0, BoxKind::Polynum);
                    return BoxVariant::Polynum(poly.cast());
                }

                let is_multinum = vs.iter().all(|v| {
                    v.get_kind(0) == BoxKind::Polynum
                        || v.get_kind(0) == BoxKind::Num
                        || v.get_kind(0) == BoxKind::Empty
                });
                if is_multinum {
                    let mut poly: BoxValue<AnyBox> = vs.into();
                    poly.set_kind(0, BoxKind::Multinum);
                    return BoxVariant::Multinum(poly.cast());
                }

                let is_vexel = vs.iter().all(|v| v.get_kind(0) == BoxKind::Unixel);
                if is_vexel {
                    let mut poly: BoxValue<AnyBox> = vs.into();
                    poly.set_kind(0, BoxKind::Vexel);
                    return BoxVariant::Vexel(poly.cast());
                }

                let is_maxel = vs.iter().all(|v| v.get_kind(0) == BoxKind::Pixel);
                if is_maxel {
                    let mut poly: BoxValue<AnyBox> = vs.into();
                    poly.set_kind(0, BoxKind::Maxel);
                    return BoxVariant::Maxel(poly.cast());
                }

                BoxVariant::Any(vs.into())
            }
            Expr::Set(elems) => {
                let mut vs = Vec::new();
                for elem in elems {
                    let var = elem.eval(store).into_any();
                    vs.push(var.into_any_raw());
                }
                BoxVariant::Set(vs.into())
            }
            Expr::List(elems) => {
                let mut vs = Vec::new();
                for elem in elems {
                    let var = elem.eval(store).into_any();
                    vs.push(var.into_any_raw());
                }
                BoxVariant::List(vs.into())
            }
            Expr::Der(expr) => {
                let val = expr.eval(store);
                val.derivative()
            }
            Expr::DerMulti(expr, idx) => {
                let val = expr.eval(store);
                val.derivative_multi(idx.clone())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use logos::{Lexer, Logos};

    use crate::{
        BoxKind, BoxValue, BoxVariant,
        display::BoxDisplay,
        parser::{Parser, Token, parser},
        store::BoxStore,
    };

    fn collect_tokens(lexer: Lexer<Token>) -> Result<Vec<Token>, ()> {
        lexer
            .spanned()
            .map(|(token, span)| token.map_err(|_| eprintln!("could not lex token at: {span:?}")))
            .collect()
    }

    fn eval_input(input: &str) -> Result<BoxVariant, ()> {
        let mut store = BoxStore::new();
        store.store_with_name("α", BoxValue::alpha());
        store.store_with_name("β", BoxValue::beta(1_u32));

        let lexer = Token::lexer(input);
        let tokens = collect_tokens(lexer)?;

        parser()
            .parse(&tokens)
            .into_result()
            .map(|ast| ast.eval(&store))
            .map_err(|e| eprintln!("could not parse expression: {e:?}"))
    }

    #[test]
    fn test_num() {
        let input = "2";
        let val = eval_input(input).expect("eval_input failed");
        assert_eq!(val.get_kind(0), BoxKind::Num);

        let input = "⌊□,□⌋";
        let val = eval_input(input).expect("eval_input failed");
        assert_eq!(val.get_kind(0), BoxKind::Num);

        let input = "⌊₂□⌋";
        let val = eval_input(input).expect("eval_input failed");
        assert_eq!(val.get_kind(0), BoxKind::Num);

        let input = "⌊□,□⌋ + ⌊□⌋";
        let val = eval_input(input).expect("eval_input failed");
        assert_eq!(val.get_kind(0), BoxKind::Num);

        let input = "⌊<red>□</red>,□,□⌋";
        let val = eval_input(input).expect("eval_input failed");
        let exp = BoxVariant::from(1);
        assert_eq!(val, exp);
    }

    #[test]
    fn test_polynum() {
        let input = "α+1";
        let val = eval_input(input).expect("eva_input failed");
        assert_eq!(val.get_kind(0), BoxKind::Polynum);

        let input = "⌊⌊□⌋⌋";
        let val = eval_input(input).expect("eva_input failed");
        assert_eq!(val.get_kind(0), BoxKind::Polynum);

        let input = "α*α";
        let val = eval_input(input).expect("eva_input failed");
        assert_eq!(val.get_kind(0), BoxKind::Polynum);

        let input = "<red>α</red>";
        let val = eval_input(input).expect("eval_input failed");
        let exp = BoxVariant::anti_alpha();
        assert_eq!(val, exp);

        let input = "<red>9</red>";
        let val = eval_input(input).expect("eval_input failed");
        let exp = BoxVariant::from(9_u32).into_anti();
        assert_eq!(val, exp);

        let input = "α²";
        let val = eval_input(input).expect("eval_input failed");
        let exp = BoxVariant::alpha() * BoxVariant::alpha();
        assert_eq!(val, exp);
    }

    #[test]
    fn test_multinum() {
        let input = "⌊⌊⌊□⌋⌋⌋";
        let val = eval_input(input).expect("eva_input failed");
        assert_eq!(val.get_kind(0), BoxKind::Multinum);

        let input = "β₂";
        let val = eval_input(input).expect("eval_input failed");
        let exp = BoxVariant::beta(2_u32);
        assert_eq!(val, exp);

        let input = "<red>β</red>₂";
        let val = eval_input(input).expect("eval_input failed");
        let exp = BoxVariant::anti_beta(2_u32);
        assert_eq!(val, exp);

        let input = "β₂+1";
        let val = eval_input(input).expect("eval_input failed");
        let exp = BoxVariant::beta(2_u32) + BoxVariant::from(1_u32);
        assert_eq!(val, exp);

        let input = "<red>β</red>₂+1";
        let val = eval_input(input).expect("eval_input failed");
        let exp = BoxVariant::anti_beta(2_u32) + BoxVariant::from(1_u32);
        assert_eq!(val, exp);

        let input = "β₂²";
        let val = eval_input(input).expect("eval_input failed");
        let exp = BoxVariant::beta(2_u32) * BoxVariant::beta(2_u32);
        assert_eq!(val, exp);
    }

    #[test]
    fn test_div() {
        let input = "α² / α";
        let val = eval_input(input).expect("eval_input failed");
        let exp = BoxVariant::alpha();
        assert_eq!(val, exp);
    }

    #[test]
    fn test_rem() {
        let input = "(1 + α²) % α";
        let val = eval_input(input).expect("eval_input failed");
        let exp = BoxVariant::one();
        assert_eq!(val, exp);
    }

    #[test]
    fn test_vexel() {
        let input = "⌊⌈1⌉,⌈2⌉,⌈3⌉⌋";
        let val = eval_input(input).expect("eva_input failed");
        assert_eq!(val.get_kind(0), BoxKind::Vexel);
    }

    #[test]
    fn test_maxel() {
        let input = "⌊⌈1,1⌉,⌈1,2⌉,⌈2,2⌉⌋";
        let val = eval_input(input).expect("eva_input failed");
        assert_eq!(val.get_kind(0), BoxKind::Maxel);
    }

    #[test]
    fn test_list() {
        let input = "⌈1,2,3⌉";
        let val = eval_input(input).expect("eva_input failed");
        println!("{val:?}");
        assert_eq!(val.get_kind(0), BoxKind::List);
    }

    #[test]
    fn test_set() {
        let input = "{1,2,3}";
        let val = eval_input(input).expect("eva_input failed");
        assert_eq!(val.get_kind(0), BoxKind::Set);
    }

    #[test]
    fn test_der() {
        let input = "der(1+α)";
        let val = eval_input(input).expect("eva_input failed");
        let exp = BoxVariant::Polynum(BoxValue::from(1_u32).cast());
        assert_eq!(val, exp);

        let input = "der(1+α^2)";
        let val = eval_input(input).expect("eva_input failed");
        let exp = 2 * BoxVariant::alpha();
        assert_eq!(val, exp);

        let input = "der(1+β,1)";
        let val = eval_input(input).expect("eva_input failed");
        let exp = BoxVariant::Multinum(BoxValue::from(1_u32).cast());
        assert_eq!(val, exp);

        let input = "der(1+β^2,1)";
        let val = eval_input(input).expect("eva_input failed");
        let exp = 2 * BoxVariant::beta(1_u32);
        assert_eq!(val, exp);
    }

    #[test]
    fn test_parse() {
        let mut store = BoxStore::new();
        let alpha = BoxValue::alpha();
        store.store_with_name("alpha", alpha);

        let input = "-2 + 3 - 2*alpha + 5*alpha^2";
        let lexer = Token::lexer(input);
        let tokens = collect_tokens(lexer).unwrap();

        // parse the tokens to construct an AST
        let ast = match parser().parse(&tokens).into_result() {
            Ok(expr) => {
                println!("[AST]\n{:#?}", expr);
                expr
            }
            Err(e) => {
                println!("parse error: {:#?}", e);
                return;
            }
        };

        // evaluates the AST to get the result
        let val = ast.eval(&store);
        println!("\n[result]\n{:#}", BoxDisplay::from_variant(val, &store));

        // let input = "⌊⌈1,1⌉,⌈1,2⌉,₂⌈2,2⌉⌋";
        // let input = "⌊⌈⌊□⌋,⌊□⌋⌉,⌈⌊□⌋,⌊□,□⌋⌉,⌈⌊□,□⌋,⌊□,□⌋⌉,⌈⌊□,□⌋,⌊□,□⌋⌉⌋";
        // let input = "⌊⌈⌊□⌋,⌊□⌋⌉,⌈⌊□⌋,⌊₂□⌋⌉,₂⌈⌊₂□⌋,⌊₂□⌋⌉⌋";
        // let input = "{2, 3, 4} ∪ {2, 5}";
        // let input = "{2, 3, 4} ∩ {2, 5}";
        // let input = "⌊₂<red>⌊</red>1,2,3<red>⌋</red>⌋";
        let input = "⌊⌈⌊□⌋,⌊□⌋⌉,⌈⌊□⌋,⌊₂□⌋⌉,₂⌈⌊₂□⌋,⌊₂□⌋⌉⌋";
        let lexer = Token::lexer(input);
        let tokens = collect_tokens(lexer).unwrap();

        // parse the tokens to construct an AST
        let ast = match parser().parse(&tokens).into_result() {
            Ok(expr) => {
                println!("[AST]\n{:#?}", expr);
                expr
            }
            Err(e) => {
                println!("parse error: {:#?}", e);
                return;
            }
        };

        // evaluates the AST to get the result
        let val = ast.eval(&store);
        println!("\n[result]\n{:#}", BoxDisplay::from_variant(val, &store));
    }
}
