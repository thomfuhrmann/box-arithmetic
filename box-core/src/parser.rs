use crate::{BoxKind, BoxValue, BoxVariant, Color, store::BoxStore};

use chumsky::{prelude::*, util::MaybeRef};
use logos::{Lexer, Logos};
use malachite::Natural;

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

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    #[token("□")]
    Empty,
    #[regex(r"[0-9]+", |lex|lex.slice().parse())]
    Number(Natural),
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Var(String),
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Multiply,
    #[token("/")]
    Divide,
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
    #[token("<red>")]
    RedOpen,
    #[token("</red>")]
    RedClose,
    #[token("der")]
    Der,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Empty,
    Num(Natural),
    Var(String),
    Neg(Box<Expr>),
    Subscript(Natural, Box<Expr>),
    Anti(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
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
}

fn subscript<'a>() -> impl Parser<'a, &'a [Token], Natural, extra::Err<Simple<'a, Token>>> + Clone {
    any().filter_map(|token| match token {
        Token::Subscript(num) => Some(num),
        _ => None,
    })
}

fn colored_token<'a>(
    token: Token,
) -> impl Parser<'a, &'a [Token], Color, extra::Err<Simple<'a, Token>>> + Clone {
    just(token.clone()).to(Color::Black).or(just(Token::RedOpen)
        .ignore_then(just(token))
        .then_ignore(just(Token::RedClose))
        .to(Color::Red))
}

fn open_box<'a>() -> impl Parser<'a, &'a [Token], Color, extra::Err<Simple<'a, Token>>> + Clone {
    colored_token(Token::OpenBox)
}

fn close_box<'a>() -> impl Parser<'a, &'a [Token], Color, extra::Err<Simple<'a, Token>>> + Clone {
    colored_token(Token::CloseBox)
}

fn open_list<'a>() -> impl Parser<'a, &'a [Token], Color, extra::Err<Simple<'a, Token>>> + Clone {
    colored_token(Token::OpenList)
}

fn close_list<'a>() -> impl Parser<'a, &'a [Token], Color, extra::Err<Simple<'a, Token>>> + Clone {
    colored_token(Token::CloseList)
}

fn open_set<'a>() -> impl Parser<'a, &'a [Token], Color, extra::Err<Simple<'a, Token>>> + Clone {
    colored_token(Token::OpenSet)
}

fn close_set<'a>() -> impl Parser<'a, &'a [Token], Color, extra::Err<Simple<'a, Token>>> + Clone {
    colored_token(Token::CloseSet)
}

fn box_parser<'a, P>(
    parser: P,
) -> impl Parser<'a, &'a [Token], Expr, extra::Err<Simple<'a, Token>>> + Clone
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
                    Some(num) => Expr::Subscript(num, Box::new(base_box)),
                    None => base_box,
                }
            },
        )
}

fn list_parser<'a, P>(
    parser: P,
) -> impl Parser<'a, &'a [Token], Expr, extra::Err<Simple<'a, Token>>> + Clone
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
                    Some(num) => Expr::Subscript(num, Box::new(base_box)),
                    None => base_box,
                }
            },
        )
}

fn set_parser<'a, P>(
    parser: P,
) -> impl Parser<'a, &'a [Token], Expr, extra::Err<Simple<'a, Token>>> + Clone
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
                    Some(num) => Expr::Subscript(num, Box::new(base_box)),
                    None => base_box,
                }
            },
        )
}

fn vexel_parser<'a, P>(
    parser: P,
) -> impl Parser<'a, &'a [Token], Expr, extra::Err<Simple<'a, Token>>> + Clone
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
                    Some(num) => Expr::Subscript(num, Box::new(base_box)),
                    None => base_box,
                }
            },
        );

    box_parser(unixel_with_subscript)
}

fn maxel_parser<'a, P>(
    parser: P,
) -> impl Parser<'a, &'a [Token], Expr, extra::Err<Simple<'a, Token>>> + Clone
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
                    Some(num) => Expr::Subscript(num, Box::new(base_box)),
                    None => base_box,
                }
            },
        );

    box_parser(pixel_with_subscript)
}

pub fn parser<'src>()
-> impl Parser<'src, &'src [Token], Expr, chumsky::extra::Err<chumsky::error::Simple<'src, Token>>>
{
    recursive(|p| {
        let number = select! {
            Token::Number(n) => Expr::Num(n),
        };

        let empty_box = select! {
            Token::Empty => Expr::Empty,
        };

        let var = select! { Token::Var(name) => Expr::Var(name) };

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
            .or(set_parser(p.clone()))
            .or(parenthesized);

        let atom = just(Token::Minus)
            .repeated()
            .collect::<Vec<_>>()
            .then(subscript().or_not())
            .then(base_atom)
            .map(|((minuses, num), expr)| {
                let mut expr = if let Some(num) = num {
                    Expr::Subscript(num, Box::new(expr))
                } else {
                    expr
                };
                for _ in minuses {
                    expr = Expr::Neg(Box::new(expr));
                }
                expr
            });

        let caret = atom
            .clone()
            .then_ignore(just(Token::Caret))
            .then(select! { Token::Number(n) => n })
            .map(|(base, n)| Expr::Caret(Box::new(base), n));

        let prod = caret.clone().or(atom.clone()).foldl(
            just(Token::Multiply)
                .or(just(Token::Divide))
                // Bugfix: check for caret before falling back to a bare atom
                .then(caret.or(atom))
                .repeated(),
            |lhs, (op, rhs)| match op {
                Token::Multiply => Expr::Mul(Box::new(lhs), Box::new(rhs)),
                Token::Divide => Expr::Div(Box::new(lhs), Box::new(rhs)),
                _ => unreachable!(),
            },
        );

        let sum = prod.clone().foldl(
            just(Token::Plus)
                .or(just(Token::Minus))
                .then(prod)
                .repeated(),
            |lhs, (op, rhs)| match op {
                Token::Plus => Expr::Add(Box::new(lhs), Box::new(rhs)),
                Token::Minus => Expr::Sub(Box::new(lhs), Box::new(rhs)),
                _ => unreachable!(),
            },
        );

        sum.clone().foldl(
            just(Token::Union)
                .or(just(Token::Intersection))
                .then(sum)
                .repeated(),
            |lhs, (op, rhs)| match op {
                Token::Union => Expr::Union(Box::new(lhs), Box::new(rhs)),
                Token::Intersection => Expr::Intersection(Box::new(lhs), Box::new(rhs)),
                _ => unreachable!(),
            },
        )
    })
}

impl Expr {
    pub fn eval(&self, store: &BoxStore) -> BoxVariant {
        match self {
            Expr::Empty => BoxVariant::Empty(BoxValue::zero()),
            Expr::Num(n) => BoxVariant::Num(BoxValue::from(n.clone())),
            Expr::Var(name) => store
                .fetch_box_by_name(name)
                .expect("Undefined Var assignment"),
            Expr::Subscript(n, v) => {
                let mut variant = v.eval(store);
                variant.set_multiplicity(0, n.clone());
                variant
            }
            Expr::Neg(rhs) => BoxVariant::Num(BoxValue::from(-1)) * rhs.eval(store),
            Expr::Add(lhs, rhs) => lhs.eval(store) + rhs.eval(store),
            Expr::Mul(lhs, rhs) => lhs.eval(store) * rhs.eval(store),
            Expr::Sub(lhs, rhs) => {
                lhs.eval(store) + BoxVariant::Num(BoxValue::from(-1)) * rhs.eval(store)
            }
            // Expr::Div(lhs, rhs) => todo!(),
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
                    let var = bx.eval(store).into_any();
                    vs.push(var.into_any_raw());
                }

                // check if box represents a number
                let empty = vs.iter().all(|v| v.get_kind(0) == BoxKind::Empty);
                if empty && !vs.is_empty() {
                    let mul = vs[0].get_multiplicity(0);
                    return BoxVariant::Num(mul.into());
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
            Expr::Anti(v) => v.eval(store).into_anti(),
            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use logos::Logos;

    use crate::{
        AnyBox, BoxValue,
        display::BoxDisplay,
        parser::{Parser, Token, parser},
        store::BoxStore,
    };

    #[test]
    fn test_parse() {
        let mut store = BoxStore::new();
        let alpha = BoxValue::alpha();
        store.store_box_with_name("alpha", alpha);

        let input = "-2 + 3 - 2*alpha + 5*alpha^2";
        let lexer = Token::lexer(input);
        let mut tokens = vec![];
        for (token, span) in lexer.spanned() {
            match token {
                Ok(token) => tokens.push(token),
                Err(e) => {
                    println!("lexer error at {:?}: {:?}", span, e);
                    return;
                }
            }
        }

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
        println!("\n[result]\n{:#}", val);

        // let input = "⌊⌈1,1⌉,⌈1,2⌉,₂⌈2,2⌉⌋";
        // let input = "⌊⌈⌊□⌋,⌊□⌋⌉,⌈⌊□⌋,⌊□,□⌋⌉,⌈⌊□,□⌋,⌊□,□⌋⌉,⌈⌊□,□⌋,⌊□,□⌋⌉⌋";
        // let input = "⌊⌈⌊□⌋,⌊□⌋⌉,⌈⌊□⌋,⌊₂□⌋⌉,₂⌈⌊₂□⌋,⌊₂□⌋⌉⌋";
        // let input = "{2, 3, 4} ∪ {2, 5}";
        // let input = "{2, 3, 4} ∩ {2, 5}";
        // let input = "⌊₂<red>⌊</red>1,2,3<red>⌋</red>⌋";
        let input = "⌊⌈⌊□⌋,⌊□⌋⌉,⌈⌊□⌋,⌊₂□⌋⌉,₂⌈⌊₂□⌋,⌊₂□⌋⌉⌋";
        let lexer = Token::lexer(input);
        let mut tokens = vec![];
        for (token, span) in lexer.spanned() {
            match token {
                Ok(token) => tokens.push(token),
                Err(e) => {
                    println!("lexer error at {:?}: {:?}", span, e);
                    return;
                }
            }
        }

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
        let any_val: BoxDisplay<AnyBox> = (&val).into();

        println!("{any_val:?}");
        println!("\n[result]\n{:#}", val);
    }
}
