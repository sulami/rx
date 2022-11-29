use rx::convert;
use rx::output::pcre::PCREOutput;

fn render(input: &str) -> String {
    let output = &PCREOutput::default();
    convert(input, output).expect("failed to convert")
}

#[test]
fn test_char() {
    assert_eq!(render("f"), "f");
}

#[test]
fn test_whitespace() {
    assert_eq!(render("whitespace"), r#"[\s]"#);
}

#[test]
fn test_alpha() {
    assert_eq!(render("alpha"), "[a-zA-Z]");
}

#[test]
fn test_digit() {
    assert_eq!(render("digit"), r#"[\d]"#);
}

#[test]
fn test_word() {
    assert_eq!(render(r#""foo""#), "foo");
}

#[test]
fn test_zero_or_more_char() {
    assert_eq!(render("(0+ f)"), "f*");
}

#[test]
fn test_zero_or_more_two_chars() {
    assert_eq!(render("(0+ f g)"), "(?:fg)*");
}

#[test]
fn test_one_or_more_char() {
    assert_eq!(render("(1+ f)"), "f+");
}

#[test]
fn test_one_or_more_two_chars() {
    assert_eq!(render("(1+ f g)"), "(?:fg)+");
}
