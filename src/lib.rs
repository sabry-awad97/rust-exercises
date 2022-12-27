pub fn split_string_on_uppercase_chars(s: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut current_word = String::new();

    let mut prev_char: Option<char> = None;
    for c in s.chars() {
        match (prev_char, c) {
            (Some(p), c) if c.is_uppercase() => {
                if !(p.is_uppercase() || p == '-' || p == '.' || p.is_whitespace()) {
                    if !current_word.is_empty() {
                        result.push(current_word);
                    }
                    current_word = String::new();
                }
                current_word.push(c);
            }
            _ => current_word.push(c),
        }
        prev_char = Some(c);
    }

    if !current_word.is_empty() {
        result.push(current_word);
    }

    result
}
