use pyo3::{PyErr, pyfunction, PyResult};

use crate::v1::part::RegexPart;

pub fn escape_special_characters(s: &str) -> String {
    let mut escaped = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '.' | '+' | '*' | '?' | '^' | '$' | '(' | ')' | '[' | ']' | '{' | '}' | '|' | '\\' => {
                escaped.push('\\');
                escaped.push(c);
            }
            _ => escaped.push(c),
        }
    }
    escaped
}

#[pyfunction]
pub fn digit() -> RegexPart {
    RegexPart {
        pattern: r"\d".to_string(),
    }
}

#[pyfunction]
pub fn exactly(s: &str) -> RegexPart {
    RegexPart {
        pattern: escape_special_characters(s),
    }
}

#[pyfunction]
#[pyo3(signature = (*parts))]
pub fn any_of(parts: Vec<RegexPart>) -> PyResult<RegexPart> {
    let patterns: Result<Vec<String>, PyErr> = parts.into_iter()
        .map(|part| Ok(part.pattern))
        .collect();
    let patterns = patterns?;
    Ok(RegexPart {
        pattern: format!("({})", patterns.join("|")),
    })
}