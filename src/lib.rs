use std::collections::HashSet;

pub fn split_string_on_uppercase_chars(s: &str, special_chars: &[char]) -> Vec<String> {
    // Preallocate capacity for the result vector and the current_word string
    // to avoid reallocations as they grow
    let mut result = Vec::with_capacity(s.len());
    let mut current_word = String::with_capacity(s.len());

    // Initialize a variable to store the previous character
    let mut prev_char: Option<char> = None;

    // Convert the special_chars slice into a HashSet for faster lookup
    let special_chars_set: HashSet<char> = special_chars.iter().copied().collect();

    // Iterate over the characters in the input string
    for c in s.chars() {
        // Check the previous character if it is not None
        if let Some(p) = prev_char {
            // If the current character is uppercase and the previous character is not uppercase,
            // a whitespace character, or a special character, split the current word
            if c.is_uppercase()
                && !(p.is_uppercase() || p.is_whitespace() || special_chars_set.contains(&p))
            {
                if !current_word.is_empty() {
                    // Push the current word onto the result vector and reset current_word to an empty string
                    result.push(current_word);
                    current_word = String::new();
                }
            }
        }
        // Append the current character to the current word
        current_word.push(c);
        // Update the previous character
        prev_char = Some(c);
    }

    // If the current word is not empty, push it onto the result vector
    if !current_word.is_empty() {
        result.push(current_word);
    }

    // Return the result vector
    result
}
