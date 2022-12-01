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
fn test_string() {
    assert_eq!(render(r#""f""#), "f");
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
fn test_alphanum() {
    assert_eq!(render("alnum"), "[0-9a-zA-Z]");
}

#[test]
fn test_hex() {
    assert_eq!(render("hex"), "[0-9a-fA-F]");
}

#[test]
fn test_lowercase() {
    assert_eq!(render("lower"), "[a-z]");
}

#[test]
fn test_uppercase() {
    assert_eq!(render("upper"), "[A-Z]");
}

#[test]
fn test_word() {
    assert_eq!(render(r#""foo""#), "foo");
}

#[test]
fn test_line_start() {
    assert_eq!(render(r#"(: bol "foo")"#), "^foo");
}

#[test]
fn test_line_end() {
    assert_eq!(render(r#"(: "foo" eol)"#), "foo$");
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
fn test_zero_or_more_reluctant_char() {
    assert_eq!(render("(*? f)"), "f*?");
}

#[test]
fn test_zero_or_more_two_reluctant_chars() {
    assert_eq!(render("(*? f g)"), "(?:fg)*?");
}

#[test]
fn test_one_or_more_char() {
    assert_eq!(render("(1+ f)"), "f+");
}

#[test]
fn test_one_or_more_two_chars() {
    assert_eq!(render("(1+ f g)"), "(?:fg)+");
}

#[test]
fn test_one_or_more_reluctant_char() {
    assert_eq!(render("(+? f)"), "f+?");
}

#[test]
fn test_one_or_more_two_reluctant_chars() {
    assert_eq!(render("(+? f g)"), "(?:fg)+?");
}

#[test]
fn test_seq() {
    assert_eq!(render(r#"(seq a "bc" alpha)"#), "abc[a-zA-Z]");
}

#[test]
fn test_any_char_class() {
    assert_eq!(render("(any lower)"), "[a-z]");
}

#[test]
fn test_any_char_classes() {
    assert_eq!(render("(any lower upper)"), "[a-zA-Z]");
}

#[test]
fn test_not_char() {
    assert_eq!(render(r#"(not f)"#), "[^f]");
}

#[test]
fn test_not_string() {
    assert_eq!(render(r#"(not "abc")"#), "[^abc]");
}

#[test]
fn test_not_range_string() {
    assert_eq!(render(r#"(not "a-z")"#), "[^a-z]");
}

#[test]
fn test_not_char_class() {
    assert_eq!(render(r#"(not alpha)"#), "[^a-zA-Z]");
}

#[test]
fn test_not_backslash_char_class() {
    assert_eq!(render(r#"(not digit)"#), r#"[^\d]"#);
}

#[test]
fn test_or_string() {
    assert_eq!(render(r#"(or "foo")"#), "foo");
}

#[test]
fn test_or_two_strings() {
    assert_eq!(render(r#"(or "foo" "bar")"#), "foo|bar");
}

#[test]
fn test_or_string_seq() {
    assert_eq!(render(r#"(or "foo" (: a word))"#), r#"foo|a[\w]"#);
}

#[test]
fn test_or_in_seq() {
    assert_eq!(
        render(r#"(seq (or "foo" "bar") (or "dingle" "bop"))"#),
        r#"(?:foo|bar)(?:dingle|bop)"#
    );
}

#[test]
fn test_charclass_in_seq() {
    assert_eq!(render(r#"(seq digit (0+ digit))"#), r#"[\d](?:[\d]*)"#);
}

#[test]
fn test_word_boundary() {
    assert_eq!(
        render(r#"(seq (1+ digit) word-boundary (1+ digit))"#),
        r#"(?:[\d]+)\b(?:[\d]+)"#
    );
}

#[test]
fn test_not_word_boundary() {
    assert_eq!(
        render(r#"(seq (1+ digit) not-word-boundary (1+ digit))"#),
        r#"(?:[\d]+)\B(?:[\d]+)"#
    );
}

#[test]
fn test_exact_count_char() {
    assert_eq!(render(r#"(= 2 f)"#), r#"(?:f){2}"#);
}

#[test]
fn test_exact_count_string() {
    assert_eq!(render(r#"(= 2 "foo")"#), r#"(?:foo){2}"#);
}

#[test]
fn test_exact_count_char_class() {
    assert_eq!(render(r#"(= 2 lower)"#), r#"(?:[a-z]){2}"#);
}

#[test]
fn test_exact_count_char_classes() {
    assert_eq!(render(r#"(= 2 lower upper)"#), r#"(?:[a-z][A-Z]){2}"#);
}

#[test]
fn test_at_least_count_char() {
    assert_eq!(render(r#"(>= 2 f)"#), r#"(?:f){2,}"#);
}

#[test]
fn test_at_least_count_string() {
    assert_eq!(render(r#"(>= 2 "foo")"#), r#"(?:foo){2,}"#);
}

#[test]
fn test_at_least_count_char_class() {
    assert_eq!(render(r#"(>= 2 lower)"#), r#"(?:[a-z]){2,}"#);
}

#[test]
fn test_at_least_count_char_classes() {
    assert_eq!(render(r#"(>= 2 lower upper)"#), r#"(?:[a-z][A-Z]){2,}"#);
}

#[test]
fn test_between_count_char() {
    assert_eq!(render(r#"(** 2 5 f)"#), r#"(?:f){2,5}"#);
}

#[test]
fn test_between_count_string() {
    assert_eq!(render(r#"(** 2 5 "foo")"#), r#"(?:foo){2,5}"#);
}

#[test]
fn test_between_count_char_class() {
    assert_eq!(render(r#"(** 2 5 lower)"#), r#"(?:[a-z]){2,5}"#);
}

#[test]
fn test_between_count_char_classes() {
    assert_eq!(render(r#"(** 2 5 lower upper)"#), r#"(?:[a-z][A-Z]){2,5}"#);
}

#[test]
fn test_group() {
    assert_eq!(render("(group lower)"), "([a-z])");
}

#[test]
fn test_group_n() {
    assert_eq!(render("(group-n 5 lower)"), "(?<n5>[a-z])");
}

#[test]
fn test_back() {
    assert_eq!(render("(backref 5)"), r#"\5"#);
}
