use std::str;
use std::string::String;

#[pyfunction]
pub fn snake_case(s: &str) -> String {
    let mut cased_s: String = String::new();
    let character_vector: Vec<char> = s.chars().collect();
    for i in 0..character_vector.len() {
        if i == 0 {
            // First letter is always lower-case
            cased_s.push_str(&character_vector[i].to_lowercase().to_string())
        } else if character_vector[i].is_uppercase() {
            // Upper cased letters are replaced with "_{lower-cased letter}"
            cased_s.push_str(&format!("_{}", character_vector[i].to_lowercase()).to_string());
        } else {
            // Anything else is returned as lower-case
            cased_s.push_str(&character_vector[i].to_string());
        }
    }
    cased_s
}


#[pyfunction]
pub fn camel_case(s: &str) -> String {
    let mut cased_s: String = String::new();
    let character_vector: Vec<char> = s.chars().collect();
    let underscore_vector: Vec<char> = "_".chars().collect();
    let underscore = underscore_vector[0];
    let mut capitalize_next: bool = false;
    for i in 0..character_vector.len() {
        if i == 0 {
            // First letter is always lower-case
            cased_s.push_str(&character_vector[i].to_lowercase().to_string())
        } else if capitalize_next {
            // If capitalize_next is True, we capitalize the letter.
            cased_s.push_str(&character_vector[i].to_uppercase().to_string());
            capitalize_next = false;
        } else if character_vector[i].eq(&underscore) {
            // Underscores are removed, and the next character is capitalized
            capitalize_next = true;
            continue;
        } else {
            // Anything else is returned as lower-case
            cased_s.push_str(&character_vector[i].to_string());
        }
    };
    cased_s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_snake_case_translations() {
        assert_eq!(snake_case("PascalCase"), "pascal_case");
        assert_eq!(snake_case("camelCase"), "camel_case");
        assert_eq!(snake_case("snake_case"), "snake_case");
        assert_eq!(snake_case(""), "");
        assert_eq!(snake_case("1"), "1");
        assert_eq!(snake_case("-22"), "-22");
        assert_eq!(snake_case("lower"), "lower");
        assert_eq!(snake_case("UPPER"), "u_p_p_e_r");
    }

    #[test]
    fn valid_camel_case_translations() {
        assert_eq!(camel_case("PascalCase"), "pascalCase");
        assert_eq!(camel_case("camelCase"), "camelCase");
        assert_eq!(camel_case("snake_case"), "snakeCase");
        assert_eq!(camel_case(""), "");
        assert_eq!(camel_case("1"), "1");
        assert_eq!(camel_case("-22"), "-22");
        assert_eq!(camel_case("lower"), "lower");
        assert_eq!(camel_case("UPPER"), "uPPER");
    }
}