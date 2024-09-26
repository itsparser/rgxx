use pyo3::{pyclass, PyErr, pymethods, PyResult};

use crate::v1::part::RegexPart;

#[pyclass]
pub struct RegExp {
    pub pattern: String,
}

#[pymethods]
impl RegExp {
    #[new]
    #[pyo3(signature = (*parts))]
    pub fn new(parts: Vec<RegexPart>) -> PyResult<Self> {
        let patterns: Result<Vec<String>, PyErr> = parts.into_iter()
            .map(|part| Ok(part.pattern))
            .collect();
        let patterns = patterns?;
        Ok(RegExp {
            pattern: patterns.join(""),
        })
    }

    pub fn compile(&self) -> PyResult<String> {
        Ok(self.pattern.clone())
    }

    pub fn __repr__(&self) -> PyResult<String> {
        Ok(format!("RegExp({})", self.pattern))
    }
}