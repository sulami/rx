use nom::branch::alt;
use nom::bytes::complete::{escaped, tag};
use nom::character::complete::{anychar, char, digit1, multispace0, multispace1, none_of, one_of};
use nom::combinator::{eof, map};
use nom::multi::many1;
use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::IResult;

use crate::expr::{Assertion, Atom, CharClass, Expr};

pub fn parse(i: &str) -> IResult<&str, Expr> {
    terminated(parse_expr, eof)(i)
}

fn parse_expr(i: &str) -> IResult<&str, Expr> {
    alt((
        parse_seq,
        parse_or,
        parse_zero_or_one,
        parse_zero_or_more,
        parse_zero_or_more_reluctant,
        parse_one_or_more,
        parse_one_or_more_reluctant,
        parse_exactly,
        parse_at_least,
        parse_between,
        parse_not,
        parse_any,
        parse_assertion,
        parse_group,
        parse_group_n,
        parse_backref,
        parse_atom_expr,
    ))(i)
}

fn parse_seq(i: &str) -> IResult<&str, Expr> {
    map(
        delimited(
            tuple((
                char('('),
                multispace0,
                alt((tag("seq"), tag(":"), tag("sequence"), tag("and"))),
            )),
            many1(preceded(multispace1, parse_expr)),
            tuple((multispace0, char(')'))),
        ),
        |exprs| Expr::Seq(exprs),
    )(i)
}

fn parse_or(i: &str) -> IResult<&str, Expr> {
    map(
        delimited(
            tuple((char('('), multispace0, alt((tag("or"), tag("|"))))),
            many1(preceded(multispace1, parse_expr)),
            tuple((multispace0, char(')'))),
        ),
        |exprs| Expr::Or(exprs),
    )(i)
}

fn parse_any(i: &str) -> IResult<&str, Expr> {
    map(
        delimited(
            tuple((
                char('('),
                multispace0,
                alt((tag("any"), tag("in"), tag("char"))),
            )),
            many1(preceded(multispace1, parse_atom)),
            tuple((multispace0, char(')'))),
        ),
        |atoms| Expr::Any(atoms),
    )(i)
}

fn parse_not(i: &str) -> IResult<&str, Expr> {
    map(
        delimited(
            tuple((char('('), multispace0, tag("not"), multispace1)),
            parse_atom,
            tuple((multispace0, char(')'))),
        ),
        |atom| Expr::Not(atom),
    )(i)
}

fn parse_zero_or_one(i: &str) -> IResult<&str, Expr> {
    map(
        delimited(
            tuple((
                char('('),
                multispace0,
                alt((tag("zero-or-one"), tag("opt"), tag("optional"))),
            )),
            many1(preceded(multispace1, parse_expr)),
            tuple((multispace0, char(')'))),
        ),
        |exprs| Expr::ZeroOrOne(exprs),
    )(i)
}

fn parse_zero_or_more(i: &str) -> IResult<&str, Expr> {
    map(
        delimited(
            tuple((
                char('('),
                multispace0,
                alt((tag("zero-or-more"), tag("0+"), tag("*"))),
            )),
            many1(preceded(multispace1, parse_expr)),
            tuple((multispace0, char(')'))),
        ),
        |exprs| Expr::ZeroOrMore(exprs),
    )(i)
}

fn parse_zero_or_more_reluctant(i: &str) -> IResult<&str, Expr> {
    map(
        delimited(
            tuple((char('('), multispace0, tag("*?"))),
            many1(preceded(multispace1, parse_expr)),
            tuple((multispace0, char(')'))),
        ),
        |exprs| Expr::ZeroOrMoreReluctant(exprs),
    )(i)
}

fn parse_one_or_more(i: &str) -> IResult<&str, Expr> {
    map(
        delimited(
            tuple((
                char('('),
                multispace0,
                alt((tag("one-or-more"), tag("1+"), tag("+"))),
            )),
            many1(preceded(multispace1, parse_expr)),
            tuple((multispace0, char(')'))),
        ),
        |exprs| Expr::OneOrMore(exprs),
    )(i)
}

fn parse_one_or_more_reluctant(i: &str) -> IResult<&str, Expr> {
    map(
        delimited(
            tuple((char('('), multispace0)),
            preceded(tag("+?"), many1(preceded(multispace1, parse_expr))),
            tuple((multispace0, char(')'))),
        ),
        |exprs| Expr::OneOrMoreReluctant(exprs),
    )(i)
}

fn parse_exactly(i: &str) -> IResult<&str, Expr> {
    map(
        delimited(
            tuple((char('('), multispace0, tag("="), multispace1)),
            tuple((digit1, many1(preceded(multispace1, parse_expr)))),
            tuple((multispace0, char(')'))),
        ),
        |(n, exprs)| {
            Expr::Exactly(
                n.parse().expect("failed to parse exactly quantifier"),
                exprs,
            )
        },
    )(i)
}

fn parse_at_least(i: &str) -> IResult<&str, Expr> {
    map(
        delimited(
            tuple((char('('), multispace0, tag(">="), multispace1)),
            tuple((digit1, many1(preceded(multispace1, parse_expr)))),
            tuple((multispace0, char(')'))),
        ),
        |(n, exprs)| {
            Expr::AtLeast(
                n.parse().expect("failed to parse exactly quantifier"),
                exprs,
            )
        },
    )(i)
}

fn parse_between(i: &str) -> IResult<&str, Expr> {
    map(
        delimited(
            tuple((char('('), multispace0, tag("**"), multispace1)),
            tuple((
                digit1,
                preceded(multispace1, digit1),
                many1(preceded(multispace1, parse_expr)),
            )),
            tuple((multispace0, char(')'))),
        ),
        |(n, m, exprs)| {
            Expr::Between(
                n.parse().expect("failed to parse exactly quantifier"),
                m.parse().expect("failed to parse exactly quantifier"),
                exprs,
            )
        },
    )(i)
}

fn parse_group(i: &str) -> IResult<&str, Expr> {
    map(
        delimited(
            tuple((char('('), multispace0, alt((tag("group"), tag("submatch"))))),
            many1(preceded(multispace1, parse_expr)),
            tuple((multispace0, char(')'))),
        ),
        |exprs| Expr::Group(exprs),
    )(i)
}

fn parse_group_n(i: &str) -> IResult<&str, Expr> {
    map(
        delimited(
            tuple((
                char('('),
                multispace0,
                alt((tag("group-n"), tag("submatch-n"))),
                multispace1,
            )),
            tuple((digit1, many1(preceded(multispace1, parse_expr)))),
            tuple((multispace0, char(')'))),
        ),
        |(n, exprs)| Expr::GroupN(n.parse().expect("failed to parse group identifier"), exprs),
    )(i)
}

fn parse_backref(i: &str) -> IResult<&str, Expr> {
    map(
        delimited(
            tuple((char('('), multispace0, tag("backref"), multispace1)),
            alt((
                digit1,
                delimited(
                    char('"'),
                    escaped(none_of("\\\""), '\\', one_of(r#""\"#)),
                    char('"'),
                ),
            )),
            tuple((multispace0, char(')'))),
        ),
        |n: &str| Expr::BackRef(n.to_string()),
    )(i)
}

fn parse_string(i: &str) -> IResult<&str, Atom> {
    map(
        delimited(
            char('"'),
            escaped(none_of("\\\""), '\\', one_of(r#""\"#)),
            char('"'),
        ),
        |s: &str| Atom::String(s.to_string()),
    )(i)
}

fn parse_char(i: &str) -> IResult<&str, Atom> {
    map(anychar, |c| Atom::Char(c))(i)
}

fn parse_atom(i: &str) -> IResult<&str, Atom> {
    alt((parse_class, parse_string, parse_char))(i)
}

fn parse_atom_expr(i: &str) -> IResult<&str, Expr> {
    map(alt((parse_class, parse_string, parse_char)), |a| {
        Expr::Atom(a)
    })(i)
}

fn parse_line_start(i: &str) -> IResult<&str, Assertion> {
    map(alt((tag("line-start"), tag("bol"))), |_| {
        Assertion::LineStart
    })(i)
}

fn parse_line_end(i: &str) -> IResult<&str, Assertion> {
    map(alt((tag("line-end"), tag("eol"))), |_| Assertion::LineEnd)(i)
}

fn parse_word_boundary(i: &str) -> IResult<&str, Assertion> {
    map(tag("word-boundary"), |_| Assertion::WordBoundary)(i)
}

fn parse_not_word_boundary(i: &str) -> IResult<&str, Assertion> {
    map(tag("not-word-boundary"), |_| Assertion::NotWordBoundary)(i)
}

fn parse_assertion(i: &str) -> IResult<&str, Expr> {
    map(
        alt((
            parse_line_start,
            parse_line_end,
            parse_word_boundary,
            parse_not_word_boundary,
        )),
        |a| Expr::Assertion(a),
    )(i)
}

fn parse_whitespace(i: &str) -> IResult<&str, CharClass> {
    map(alt((tag("space"), tag("whitespace"), tag("white"))), |_| {
        CharClass::Whitespace
    })(i)
}

fn parse_alpha(i: &str) -> IResult<&str, CharClass> {
    map(
        alt((tag("alpha"), tag("alphabetic"), tag("letter"))),
        |_| CharClass::Alpha,
    )(i)
}

fn parse_digit(i: &str) -> IResult<&str, CharClass> {
    map(alt((tag("digit"), tag("numeric"), tag("num"))), |_| {
        CharClass::Digit
    })(i)
}

fn parse_alphanum(i: &str) -> IResult<&str, CharClass> {
    map(alt((tag("alnum"), tag("alphanumeric"))), |_| {
        CharClass::AlphaNum
    })(i)
}

fn parse_hex(i: &str) -> IResult<&str, CharClass> {
    map(alt((tag("xdigit"), tag("hex-digit"), tag("hex"))), |_| {
        CharClass::Hex
    })(i)
}

fn parse_lowercase(i: &str) -> IResult<&str, CharClass> {
    map(alt((tag("lower"), tag("lower-case"))), |_| {
        CharClass::LowerCase
    })(i)
}

fn parse_uppercase(i: &str) -> IResult<&str, CharClass> {
    map(alt((tag("upper"), tag("upper-case"))), |_| {
        CharClass::UpperCase
    })(i)
}

fn parse_word(i: &str) -> IResult<&str, CharClass> {
    map(alt((tag("word"), tag("wordchar"))), |_| CharClass::Word)(i)
}

fn parse_class(i: &str) -> IResult<&str, Atom> {
    map(
        alt((
            parse_whitespace,
            parse_alpha,
            parse_digit,
            parse_alphanum,
            parse_hex,
            parse_lowercase,
            parse_uppercase,
            parse_word,
        )),
        |c| Atom::CharClass(c),
    )(i)
}
