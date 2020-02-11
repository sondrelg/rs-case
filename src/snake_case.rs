pub fn convert_snake_case(characters: &[char]) -> String {
    match characters {
        [] => String::new(),
        [first_char, rest @ ..] => rest.iter().fold(
            first_char.to_lowercase().to_string(), // first letter is always lower-case
            |mut acc, c| {
                match c {
                    // dash becomes underscore
                    '-' => acc.push('_'),
                    // upper-case becomes underscore + lower-case
                    _ if c.is_uppercase() => acc.push_str(&format!("_{}", c.to_lowercase())),
                    // everything else remains the same
                    _ => acc.push(*c),
                };
            acc
            }
        ),
    }
}
