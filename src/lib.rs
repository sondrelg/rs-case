use std::str;
use std::string::String;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
pub fn snake_case(s: &str) -> String {
    let mut cased_s: String = String::new();
    let character_vector: Vec<char> = s.chars().collect();
    let dash_vector: Vec<char> = "-".chars().collect();
    let dash: char = dash_vector[0];
    for i in 0..character_vector.len() {
        if i == 0 {
            // First letter is always lower-case
            cased_s.push_str(&character_vector[i].to_lowercase().to_string())
        } else if character_vector[i] == dash {
            cased_s.push_str(&"_".to_string())
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
    let dash_vector: Vec<char> = "-".chars().collect();
    let dash: char = dash_vector[0];
    let underscore_vector: Vec<char> = "_".chars().collect();
    let underscore: char = underscore_vector[0];
    let mut capitalize_next: bool = false;
    for i in 0..character_vector.len() {

        // First letter is always lower-case
        if i == 0 {
            cased_s.push_str(&character_vector[i].to_lowercase().to_string())
        } // If capitalize_next is True, we capitalize the letter.
        else if capitalize_next & character_vector[i].is_alphabetic() {
            cased_s.push_str(&character_vector[i].to_uppercase().to_string());
            capitalize_next = false;
        } else if character_vector[i].eq(&underscore) || character_vector[i].eq(&dash) {
            capitalize_next = true;
            continue;
        } // Anything else is returned as lower-case
        else {
            cased_s.push_str(&character_vector[i].to_string());
        }
    };
    cased_s
}

#[pyfunction]
fn pascal_case(s: &str) -> String {
    let mut cased_s: String = String::new();
    let character_vector: Vec<char> = s.chars().collect();
    let dash_vector: Vec<char> = "-".chars().collect();
    let dash: char = dash_vector[0];
    let underscore_vector: Vec<char> = "_".chars().collect();
    let underscore: char = underscore_vector[0];
    let mut capitalize_next: bool = false;
    for i in 0..character_vector.len() {
        // First letter is always lower-case
        if i == 0 {
            cased_s.push_str(&character_vector[i].to_uppercase().to_string())
        } // If capitalize_next is True, we capitalize the letter.
        else if capitalize_next {
            cased_s.push_str(&character_vector[i].to_uppercase().to_string());
            capitalize_next = false;
        } // Underscores are removed, and the next character is capitalized
        else if character_vector[i].eq(&underscore) || character_vector[i].eq(&dash) {
            capitalize_next = true;
            continue;
        } // Anything else is returned as lower-case
        else {
            cased_s.push_str(&character_vector[i].to_string());
        }
    };
    cased_s
}

#[pyfunction]
fn kebab_case(s: &str) -> String {
    let mut cased_s: String = String::new();
    let character_vector: Vec<char> = s.chars().collect();
    let underscore_vector: Vec<char> = "_".chars().collect();
    let underscore = underscore_vector[0];
    for i in 0..character_vector.len() {
        if i == 0 {
            cased_s.push_str(&character_vector[i].to_lowercase().to_string());
        } else if character_vector[i].is_uppercase() {
            cased_s.push_str(&format!("-{}", character_vector[i].to_lowercase()).to_string())
        } else if character_vector[i].eq(&underscore) {
            cased_s.push_str(&"-".to_lowercase().to_string())
        } else {
            cased_s.push_str(&character_vector[i].to_lowercase().to_string());
        }
    };
    cased_s
}

#[pyfunction]
fn train_case(s: &str) -> String {
    let mut cased_s: String = kebab_case(&s);
    cased_s.to_uppercase()
}

#[pymodule]
fn rscase(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(camel_case))?;
    m.add_wrapped(wrap_pyfunction!(snake_case))?;
    m.add_wrapped(wrap_pyfunction!(pascal_case))?;
    m.add_wrapped(wrap_pyfunction!(kebab_case))?;
    m.add_wrapped(wrap_pyfunction!(train_case))?;
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_snake_case_translations() {
        assert_eq!(snake_case("PascalCase"), "pascal_case");
        assert_eq!(snake_case("camelCase"), "camel_case");
        assert_eq!(snake_case("snake_case"), "snake_case");
        assert_eq!(snake_case("kebab-case"), "kebab_case");
        assert_eq!(snake_case("TRAIN-CASE"), "t_r_a_i_n__c_a_s_e");
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
        assert_eq!(camel_case("kebab-case"), "kebabCase");
        assert_eq!(camel_case("TRAIN-CASE"), "tRAINCASE");
        assert_eq!(camel_case(""), "");
        assert_eq!(camel_case("1"), "1");
        assert_eq!(camel_case("-22"), "-22");
        assert_eq!(camel_case("lower"), "lower");
        assert_eq!(camel_case("UPPER"), "uPPER");
    }

    #[test]
    fn valid_pascal_case_translations() {
        assert_eq!(pascal_case("PascalCase"), "PascalCase");
        assert_eq!(pascal_case("camelCase"), "CamelCase");
        assert_eq!(pascal_case("snake_case"), "SnakeCase");
        assert_eq!(pascal_case("kebab-case"), "KebabCase");
        assert_eq!(pascal_case("TRAIN-CASE"), "TRAINCASE");
        assert_eq!(pascal_case(""), "");
        assert_eq!(pascal_case("1"), "1");
        assert_eq!(pascal_case("-22"), "-22");
        assert_eq!(pascal_case("lower"), "Lower");
        assert_eq!(pascal_case("UPPER"), "UPPER");
    }


    #[test]
    fn valid_kebab_case_translations() {
        assert_eq!(kebab_case("PascalCase"), "pascal-case");
        assert_eq!(kebab_case("camelCase"), "camel-case");
        assert_eq!(kebab_case("snake_case"), "snake-case");
        assert_eq!(kebab_case("kebab-case"), "kebab-case");
        assert_eq!(kebab_case("TRAIN-CASE"), "t-r-a-i-n--c-a-s-e");
        assert_eq!(kebab_case(""), "");
        assert_eq!(kebab_case("1"), "1");
        assert_eq!(kebab_case("-22"), "-22");
        assert_eq!(kebab_case("lower"), "lower");
        assert_eq!(kebab_case("UPPER"), "u-p-p-e-r");
    }

    #[test]
    fn valid_train_case_translations() {
        assert_eq!(train_case("PascalCase"), "PASCAL-CASE");
        assert_eq!(train_case("camelCase"), "CAMEL-CASE");
        assert_eq!(train_case("snake_case"), "SNAKE-CASE");
        assert_eq!(train_case("kebab-case"), "KEBAB-CASE");
        assert_eq!(train_case("TRAIN-CASE"), "T-R-A-I-N--C-A-S-E");
        assert_eq!(train_case(""), "");
        assert_eq!(train_case("1"), "1");
        assert_eq!(train_case("-22"), "-22");
        assert_eq!(train_case("lower"), "LOWER");
        assert_eq!(train_case("UPPER"), "U-P-P-E-R");
    }
}
