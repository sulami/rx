/// A regular expression. The top-level type.
#[derive(Debug)]
pub enum Expr {
    /// Just this expression
    Atom(Atom),
    /// Zero-width assertion, e.g. line end
    Assertion(Assertion),
    /// All expressions in sequence
    Seq(Vec<Expr>),
    /// Any single one of the expressions
    Or(Vec<Expr>),
    /// Zero or one
    ZeroOrOne(Vec<Expr>),
    /// 0+, greedy
    ZeroOrMore(Vec<Expr>),
    /// 0+, non-greedy
    ZeroOrMoreReluctant(Vec<Expr>),
    /// 1+, greedy
    OneOrMore(Vec<Expr>),
    /// 1+, non-greedy
    OneOrMoreReluctant(Vec<Expr>),
    /// Not the atom
    Not(Atom),
    /// Any character from the sets
    Any(Vec<Atom>),
}

/// A single static element
#[derive(Debug)]
pub enum Atom {
    /// A single character
    Char(char),
    /// A literal string
    String(String),
    /// A character class
    CharClass(CharClass),
}

/// Zero-width assertion, e.g. line end
#[derive(Debug)]
pub enum Assertion {
    /// Start of a line or input
    LineStart,
    /// End of a line or input
    LineEnd,
}

/// A character class
#[derive(Debug)]
pub enum CharClass {
    /// Any whitespace character
    Whitespace,
    /// Any alphabetic letter
    Alpha,
    /// Any decimal digit
    Digit,
    /// Any alphabetic letter or decimal digit
    AlphaNum,
    /// A hexadecimal digit, 0-9, a-f, A-F
    Hex,
    /// A lowercase alphabetic letter
    LowerCase,
    /// A uppercase alphabetic letter
    UpperCase,
    /// A "word" character
    Word,
}
