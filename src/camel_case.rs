pub fn convert_camel_case(characters: &[char]) -> String {
    match characters {
        [] => String::new(),
        [first_char, rest @ ..] => {
            rest.iter()
                .fold(
                    (first_char.to_lowercase().to_string(), false), // first letter is always lower-case
                    |(mut accumulator, _), c| {
                        if accumulator.ends_with("-") || accumulator.ends_with("_") {
                            if c.is_alphabetic() {
                                // Capitalize a letter when followed by - or _
                                accumulator.pop();
                                accumulator.push_str(&format!("{}", c.to_uppercase()));
                            } else if c.is_numeric() {
                                // In case of a double - or _, if it's followed by something other than another
                                // - or _, leave it?
                                accumulator.push(*c);
                            } else {
                                // In case of a double - or _, just pop the old one and deal with the new one later
                                accumulator.pop();
                                accumulator.push(*c);
                            }
                        } else {
                            accumulator.push(*c);
                        }
                        (accumulator, false)
                    },
                )
            .0
        }
    }
}