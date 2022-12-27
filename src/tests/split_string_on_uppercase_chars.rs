// #[test] is a macro attribute that tells the Rust testing framework to treat this function as a test
#[test]
fn test_split_string_on_capital_letters() {
    let test_cases = [
        ("HelloWorld", vec!["Hello", "World"]),
        ("Hello World!", vec!["Hello World!"]),
        ("Hello-World", vec!["Hello-World"]),
        ("Hello.World", vec!["Hello.World"]),
        ("", vec![]),
        ("A", vec!["A"]),
        ("Aa", vec!["Aa"]),
        ("AA", vec!["AA"]),
    ];

    for (s, expected) in test_cases.iter() {
        let result = split_string_on_uppercase_chars(s);
        assert_eq!(result, *expected);
    }
}
