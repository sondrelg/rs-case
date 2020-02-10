pub fn convert_kebab_case(characters: &[char]) -> String {
    match characters {
        [] => String::new(),
        [first_char, rest @ ..] => rest.iter().fold(
            first_char.to_lowercase().to_string(), // first letter is always lower-case
            |mut acc, c| {
                match c {
                    // underscore becomes dash
                    '_' => acc.push('-'),
                    // upper-case becomes underscore + lower-case
                    _ if c.is_uppercase() => acc.push_str(&format!("-{}", c.to_lowercase())),
                    // everything else remains the same
                    _ => acc.push(*c),
                };
            acc
            }
        ),
    }
}
