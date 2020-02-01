use std::str;
use std::string::String;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

mod functions;


#[pymodule]
fn rscase(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(functions::camel_case))?;
    m.add_wrapped(wrap_pyfunction!(functions::snake_case))?;
    m.add_wrapped(wrap_pyfunction!(functions::pascal_case))?;
    m.add_wrapped(wrap_pyfunction!(functions::kebab_case))?;
    Ok(())
}


