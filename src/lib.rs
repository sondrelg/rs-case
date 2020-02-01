extern crate pyo3;

use std::str;
use std::string::String;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn snake_case(s: &str) -> PyResult<String> {
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
    Ok(cased_s)
}

#[pyfunction]
fn camel_case(s: &str) -> PyResult<String> {
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
    Ok(cased_s)
}

#[pyfunction]
fn pascal_case(s: &str) -> PyResult<String> {
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
    Ok(cased_s)
}

#[pyfunction]
fn kebab_case(s: &str) -> PyResult<String> {
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
    Ok(cased_s)
}

#[pyfunction]
fn train_case(s: &str) -> PyResult<String> {
    let cased_s = kebab_case(&s);
    Ok(cased_s.unwrap().to_uppercase())
}


#[pymodule]
#[allow(unused_variables)]
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
        assert_eq!(snake_case("PascalCase").unwrap(), "pascal_case");
        assert_eq!(snake_case("camelCase").unwrap(), "camel_case");
        assert_eq!(snake_case("snake_case").unwrap(), "snake_case");
        assert_eq!(snake_case("kebab-case").unwrap(), "kebab_case");
        assert_eq!(snake_case("TRAIN-CASE").unwrap(), "t_r_a_i_n__c_a_s_e");
        assert_eq!(snake_case("").unwrap(), "");
        assert_eq!(snake_case("1").unwrap(), "1");
        assert_eq!(snake_case("-22").unwrap(), "-22");
        assert_eq!(snake_case("lower").unwrap(), "lower");
        assert_eq!(snake_case("UPPER").unwrap(), "u_p_p_e_r");
    }

    #[test]
    fn valid_camel_case_translations() {
        assert_eq!(camel_case("PascalCase").unwrap(), "pascalCase");
        assert_eq!(camel_case("camelCase").unwrap(), "camelCase");
        assert_eq!(camel_case("snake_case").unwrap(), "snakeCase");
        assert_eq!(camel_case("kebab-case").unwrap(), "kebabCase");
        assert_eq!(camel_case("TRAIN-CASE").unwrap(), "tRAINCASE");
        assert_eq!(camel_case("").unwrap(), "");
        assert_eq!(camel_case("1").unwrap(), "1");
        assert_eq!(camel_case("-22").unwrap(), "-22");
        assert_eq!(camel_case("lower").unwrap(), "lower");
        assert_eq!(camel_case("UPPER").unwrap(), "uPPER");
    }

    #[test]
    fn valid_pascal_case_translations() {
        assert_eq!(pascal_case("PascalCase").unwrap(), "PascalCase");
        assert_eq!(pascal_case("camelCase").unwrap(), "CamelCase");
        assert_eq!(pascal_case("snake_case").unwrap(), "SnakeCase");
        assert_eq!(pascal_case("kebab-case").unwrap(), "KebabCase");
        assert_eq!(pascal_case("TRAIN-CASE").unwrap(), "TRAINCASE");
        assert_eq!(pascal_case("").unwrap(), "");
        assert_eq!(pascal_case("1").unwrap(), "1");
        assert_eq!(pascal_case("-22").unwrap(), "-22");
        assert_eq!(pascal_case("lower").unwrap(), "Lower");
        assert_eq!(pascal_case("UPPER").unwrap(), "UPPER");
    }


    #[test]
    fn valid_kebab_case_translations() {
        assert_eq!(kebab_case("PascalCase").unwrap(), "pascal-case");
        assert_eq!(kebab_case("camelCase").unwrap(), "camel-case");
        assert_eq!(kebab_case("snake_case").unwrap(), "snake-case");
        assert_eq!(kebab_case("kebab-case").unwrap(), "kebab-case");
        assert_eq!(kebab_case("TRAIN-CASE").unwrap(), "t-r-a-i-n--c-a-s-e");
        assert_eq!(kebab_case("").unwrap(), "");
        assert_eq!(kebab_case("1").unwrap(), "1");
        assert_eq!(kebab_case("-22").unwrap(), "-22");
        assert_eq!(kebab_case("lower").unwrap(), "lower");
        assert_eq!(kebab_case("UPPER").unwrap(), "u-p-p-e-r");
    }

    #[test]
    fn valid_train_case_translations() {
        assert_eq!(train_case("PascalCase").unwrap(), "PASCAL-CASE");
        assert_eq!(train_case("camelCase").unwrap(), "CAMEL-CASE");
        assert_eq!(train_case("snake_case").unwrap(), "SNAKE-CASE");
        assert_eq!(train_case("kebab-case").unwrap(), "KEBAB-CASE");
        assert_eq!(train_case("TRAIN-CASE").unwrap(), "T-R-A-I-N--C-A-S-E");
        assert_eq!(train_case("").unwrap(), "");
        assert_eq!(train_case("1").unwrap(), "1");
        assert_eq!(train_case("-22").unwrap(), "-22");
        assert_eq!(train_case("lower").unwrap(), "LOWER");
        assert_eq!(train_case("UPPER").unwrap(), "U-P-P-E-R");
    }
}
