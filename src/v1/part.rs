use pyo3::{pyclass, pymethods};
use pyo3::prelude::*;

#[pyclass]
#[derive(Clone)]
pub struct Part {
    pub pattern: String,
}

#[pymethods]
impl Part {
    #[new]
    pub fn new(pattern: &str) -> Self {
        Self {
            pattern: pattern.to_string(),
        }
    }

    /// times - Repeats the pattern exactly `n` times.
    pub fn times(&self, count: usize) -> Self {
        Self::new(&format!("({}){{{}}}", self.pattern, count))
    }

    /// grouped_as - Name the capture group as `name`.
    pub fn grouped_as(&self, name: &str) -> Self {
        Part {
            pattern: format!("(?P<{}>{})", name, self.pattern),
        }
    }

    /// and - Concatenates the current pattern with another.
    pub fn and(&self, other: &Part) -> Self {
        Part {
            pattern: format!("{}{}", self.pattern, other.pattern),
        }
    }

    /// __and__ - Concatenates the current pattern with another.
    pub fn __and__(&self, other: &Part) -> Self {
        self.and(other)
    }

    /// digit - Matches any single digit (`\d`).
    pub fn digit(&self) -> PyResult<Self> {
        Ok(Part { pattern: format!("{}\\d", self.pattern) })
    }


    /// any_of - Matches any one of the provided patterns.
    pub fn any_of(&self, parts: Vec<Part>) -> PyResult<Self> {
        let patterns: Result<Vec<String>, PyErr> = parts.into_iter()
            .map(|part| Ok(part.pattern))
            .collect();
        let patterns = patterns?;
        Ok(Part { pattern: format!("(?:{})", patterns.join("|")) })
    }

    /// exactly - Matches the exact string `s`, escaping special regex characters.
    pub fn exactly(&self, parts: Vec<Part>) -> PyResult<Self> {
        let patterns: Result<Vec<String>, PyErr> = parts.into_iter()
            .map(|part| Ok(part.pattern))
            .collect();
        let patterns = patterns?;
        Ok(Part { pattern: format!("{}{}", self.pattern, patterns.join("")) })
    }

    /// any_character - Matches any character.
    pub fn any_character(&self) -> PyResult<Self> {
        Ok(Part { pattern: format!(".") })
    }

    /// infinity - Matches any character.
    pub fn infinity(&self) -> PyResult<Self> {
        Ok(Part { pattern: format!("(?:{})+", self.pattern) })
    }

    pub fn __str__(&self) -> PyResult<String> {
        Ok(self.pattern.clone())
    }

    pub fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Part({})", self.pattern))
    }

}
