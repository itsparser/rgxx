use pyo3::{pyclass, pymethods};
use pyo3::prelude::*;

#[pyclass]
#[derive(Clone)]
pub struct RegexPart {
    pub pattern: String,
}

#[pymethods]
impl RegexPart {
    #[new]
    pub fn new(pattern: &str) -> Self {
        RegexPart {
            pattern: pattern.to_string(),
        }
    }

    pub fn times(&self, count: usize) -> Self {
        RegexPart {
            pattern: format!("({}){{{}}}", self.pattern, count),
        }
    }

    pub fn grouped_as(&self, name: &str) -> Self {
        RegexPart {
            pattern: format!("(?P<{}>{})", name, self.pattern),
        }
    }

    pub fn and(&self, other: &RegexPart) -> Self {
        RegexPart {
            pattern: format!("{}{}", self.pattern, other.pattern),
        }
    }

    pub fn __and__(&self, other: &RegexPart) -> Self {
        self.and(other)
    }

    pub fn digit(&self) -> PyResult<Self> {
        Ok(RegexPart { pattern: format!("{}\\d", self.pattern) })
    }


    pub fn any_of(&self, parts: Vec<RegexPart>) -> PyResult<Self> {
        let patterns: Result<Vec<String>, PyErr> = parts.into_iter()
            .map(|part| Ok(part.pattern))
            .collect();
        let patterns = patterns?;
        Ok(RegexPart { pattern: format!("(?:{})", patterns.join("|")) })
    }

    pub fn exactly(&self, parts: Vec<RegexPart>) -> PyResult<Self> {
        let patterns: Result<Vec<String>, PyErr> = parts.into_iter()
            .map(|part| Ok(part.pattern))
            .collect();
        let patterns = patterns?;
        Ok(RegexPart { pattern: format!("{}{}", self.pattern, patterns.join("")) })
    }

    pub fn __str__(&self) -> PyResult<String> {
        Ok(self.pattern.clone())
    }

    pub fn __repr__(&self) -> PyResult<String> {
        Ok(format!("RegexPart({})", self.pattern))
    }

}
