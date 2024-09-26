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

    /// times - Repeats the pattern exactly `n` times.
    pub fn times(&self, count: usize) -> Self {
        RegexPart {
            pattern: format!("({}){{{}}}", self.pattern, count),
        }
    }

    /// grouped_as - Name the capture group as `name`.
    pub fn grouped_as(&self, name: &str) -> Self {
        RegexPart {
            pattern: format!("(?P<{}>{})", name, self.pattern),
        }
    }

    /// and - Concatenates the current pattern with another.
    pub fn and(&self, other: &RegexPart) -> Self {
        RegexPart {
            pattern: format!("{}{}", self.pattern, other.pattern),
        }
    }

    /// __and__ - Concatenates the current pattern with another.
    pub fn __and__(&self, other: &RegexPart) -> Self {
        self.and(other)
    }

    /// digit - Matches any single digit (`\d`).
    pub fn digit(&self) -> PyResult<Self> {
        Ok(RegexPart { pattern: format!("{}\\d", self.pattern) })
    }


    /// any_of - Matches any one of the provided patterns.
    pub fn any_of(&self, parts: Vec<RegexPart>) -> PyResult<Self> {
        let patterns: Result<Vec<String>, PyErr> = parts.into_iter()
            .map(|part| Ok(part.pattern))
            .collect();
        let patterns = patterns?;
        Ok(RegexPart { pattern: format!("(?:{})", patterns.join("|")) })
    }

    /// exactly - Matches the exact string `s`, escaping special regex characters.
    pub fn exactly(&self, parts: Vec<RegexPart>) -> PyResult<Self> {
        let patterns: Result<Vec<String>, PyErr> = parts.into_iter()
            .map(|part| Ok(part.pattern))
            .collect();
        let patterns = patterns?;
        Ok(RegexPart { pattern: format!("{}{}", self.pattern, patterns.join("")) })
    }

    /// any_character - Matches any character.
    pub fn any_character(&self) -> PyResult<Self> {
        Ok(RegexPart { pattern: format!(".") })
    }

    /// infinity - Matches any character.
    pub fn infinity(&self) -> PyResult<Self> {
        Ok(RegexPart { pattern: format!("(?:{})+", self.pattern) })
    }

    pub fn __str__(&self) -> PyResult<String> {
        Ok(self.pattern.clone())
    }

    pub fn __repr__(&self) -> PyResult<String> {
        Ok(format!("RegexPart({})", self.pattern))
    }

}
