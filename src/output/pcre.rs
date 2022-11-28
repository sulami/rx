use crate::expr::{Atom, Class, Expr};
use crate::output::{Output, OutputError};

pub struct PCREOutput {}

impl Output for PCREOutput {
    fn output(&self, expr: &Expr) -> Result<String, OutputError> {
        self.output_expr(expr)
    }
}

impl PCREOutput {
    fn output_expr(&self, expr: &Expr) -> Result<String, OutputError> {
        match expr {
            Expr::Atom(c) => self.output_atom(c),
            Expr::Seq(exprs) => {
                let mut s = String::new();
                for e in exprs {
                    s.push_str(&self.output_expr(e)?);
                }
                Ok(s)
            }
            Expr::Or(exprs) => {
                if exprs.len() == 1 {
                    return self.output_expr(exprs.first().expect(""));
                }
                let mut s = format!("{}|", self.output_expr(exprs.first().expect(""))?);
                for e in exprs[1..].iter() {
                    s.push_str(&self.output_expr(e)?);
                }
                Ok(s)
            }
            Expr::ZeroOrOne(exprs) => {
                let mut s = String::from("(?:");
                for e in exprs {
                    s.push_str(&self.output_expr(e)?);
                }
                s.push_str("?");
                Ok(s)
            }
            Expr::ZeroOrMore(exprs) => {
                let mut s = String::from("(?:");
                for e in exprs {
                    s.push_str(&self.output_expr(e)?);
                }
                s.push_str(")*");
                Ok(s)
            }
            Expr::ZeroOrMoreReluctant(exprs) => {
                let mut s = String::from("(?:");
                for e in exprs {
                    s.push_str(&self.output_expr(e)?);
                }
                s.push_str(")*?");
                Ok(s)
            }
            Expr::OneOrMore(exprs) => {
                let mut s = String::from("(?:");
                for e in exprs {
                    s.push_str(&self.output_expr(e)?);
                }
                s.push_str(")+");
                Ok(s)
            }
            Expr::OneOrMoreReluctant(exprs) => {
                let mut s = String::from("(?:");
                for e in exprs {
                    s.push_str(&self.output_expr(e)?);
                }
                s.push_str(")+?");
                Ok(s)
            }
        }
    }

    fn output_atom(&self, atom: &Atom) -> Result<String, OutputError> {
        match atom {
            Atom::Char(c) => Ok(format!("{c}")),
            Atom::String(s) => Ok(s.clone()),
            Atom::Class(Class::Whitespace) => Ok("\\s".to_string()),
            Atom::Class(Class::Alpha) => Ok("[a-zA-Z]".to_string()),
            Atom::Class(Class::Digit) => Ok("\\d".to_string()),
            Atom::Class(Class::AlphaNum) => Ok("[0-9a-zA-Z]".to_string()),
            Atom::Class(Class::Hex) => Ok("[0-9a-fA-F]".to_string()),
            Atom::Class(Class::LowerCase) => Ok("[a-z]".to_string()),
            Atom::Class(Class::UpperCase) => Ok("[A-Z]".to_string()),
            Atom::Class(Class::Word) => Ok("\\w".to_string()),
            Atom::LineStart => Ok("^".to_string()),
            Atom::LineEnd => Ok("$".to_string()),
        }
    }
}
