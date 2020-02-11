extern crate pyo3;

use std::str;
use std::string::String;
use std::vec::Vec;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use camel_case::convert_camel_case;
use kebab_case::convert_kebab_case;
use pascal_case::convert_pascal_case;
use snake_case::convert_snake_case;

mod snake_case;
mod camel_case;
mod pascal_case;
mod kebab_case;

#[pyfunction]
/// Converts a string to snake_case.
///
/// * `s` - The string you wish to convert.
pub fn snake_case(string: &str) -> PyResult<String> {
    Ok(convert_snake_case(&string.chars().collect::<Vec<char>>()))
}

#[pyfunction]
/// Converts a string to camelCase.
///
/// * `s` - The string you wish to convert.
pub fn camel_case(string: &str) -> PyResult<String> {
    Ok(convert_camel_case(&string.chars().collect::<Vec<char>>()))
}

#[pyfunction]
/// Converts a string from camelCase or snake_case to PascalCase.
///
/// * `s` - The string you wish to convert.
pub fn pascal_case(string: &str) -> PyResult<String> {
    Ok(convert_pascal_case(&string.chars().collect::<Vec<char>>()))
}

#[pyfunction]
/// Converts a string from camelCase or snake_case  to kebab-case.
///
/// * `s` - The string you wish to convert.
pub fn kebab_case(string: &str) -> PyResult<String> {
    Ok(convert_kebab_case(&string.chars().collect::<Vec<char>>()))
}

#[pyfunction]
/// Converts a string from camelCase or snake_case  to TRAIN-CASE.
///
/// * `s` - The string you wish to convert.
pub fn train_case(string: &str) -> PyResult<String> {
    let cased_s = convert_kebab_case(&string.chars().collect::<Vec<char>>());
    Ok(cased_s.to_uppercase())
}


#[pymodule]
#[allow(unused_variables)]
#[cfg_attr(tarpaulin, skip)]
fn rscase(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(camel_case))?;
    m.add_wrapped(wrap_pyfunction!(snake_case))?;
    m.add_wrapped(wrap_pyfunction!(pascal_case))?;
    m.add_wrapped(wrap_pyfunction!(kebab_case))?;
    m.add_wrapped(wrap_pyfunction!(train_case))?;
    Ok(())
}

#[cfg(test)]
mod test;
