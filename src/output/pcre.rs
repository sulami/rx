use crate::expr::{Assertion, Atom, CharClass, Expr};
use crate::output::{Output, OutputError};

#[derive(Copy, Clone)]
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
            Expr::Assertion(assertion) => self.output_assertion(assertion),
            Expr::Any(atoms) => {
                let mut s = String::from("[");
                for a in atoms {
                    if let Atom::CharClass(c) = a {
                        s.push_str(&self.output_char_class(c)?);
                    } else {
                        s.push_str(&self.output_atom(a)?);
                    }
                }
                s.push_str("]");
                Ok(s)
            }
            Expr::Not(Atom::CharClass(class)) => {
                Ok(format!("[^{}]", self.output_char_class(class)?))
            }
            Expr::Not(atom) => Ok(format!("[^{}]", self.output_atom(atom)?)),
        }
    }

    fn output_atom(&self, atom: &Atom) -> Result<String, OutputError> {
        match atom {
            Atom::Char(c) => Ok(format!("{c}")),
            Atom::String(s) => Ok(s.clone()),
            Atom::CharClass(class) => Ok(format!("[{}]", self.output_char_class(class)?)),
        }
    }

    fn output_char_class(&self, class: &CharClass) -> Result<String, OutputError> {
        match class {
            CharClass::Whitespace => Ok("\\s".to_string()),
            CharClass::Alpha => Ok("a-zA-Z".to_string()),
            CharClass::Digit => Ok("\\d".to_string()),
            CharClass::AlphaNum => Ok("0-9a-zA-Z".to_string()),
            CharClass::Hex => Ok("0-9a-fA-F".to_string()),
            CharClass::LowerCase => Ok("a-z".to_string()),
            CharClass::UpperCase => Ok("A-Z".to_string()),
            CharClass::Word => Ok("\\w".to_string()),
        }
    }

    fn output_assertion(self, assertion: &Assertion) -> Result<String, OutputError> {
        match assertion {
            Assertion::LineStart => Ok("^".to_string()),
            Assertion::LineEnd => Ok("$".to_string()),
        }
    }
}
