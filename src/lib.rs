use std::collections::HashSet;

pub fn split_string_on_uppercase_chars(s: &str, special_chars: &[char]) -> Vec<String> {
    let mut result = Vec::with_capacity(s.len());
    let mut current_word = String::with_capacity(s.len());

    let mut prev_char: Option<char> = None;
    let special_chars_set: HashSet<char> = special_chars.iter().copied().collect();
    for c in s.chars() {
        if let Some(p) = prev_char {
            if c.is_uppercase() && !(p.is_uppercase() || p.is_whitespace() || special_chars_set.contains(&p)) {
                if !current_word.is_empty() {
                    result.push(current_word);
                    current_word = String::new();
                }
            }
        }
        current_word.push(c);
        prev_char = Some(c);
    }

    if !current_word.is_empty() {
        result.push(current_word);
    }

    result
}
