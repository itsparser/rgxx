use pyo3::{PyErr, pyfunction, PyResult};

use crate::v1::part::Part;

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

/// digit - Matches any single digit (`\d`).
#[pyfunction]
pub fn digit() -> Part {
    Part {
        pattern: r"\d".to_string(),
    }
}

/// exactly - Matches the exact string `s`, escaping special regex characters.
#[pyfunction]
pub fn exactly(s: &str) -> Part {
    Part {
        pattern: escape_special_characters(s),
    }
}

/// starts_with - Matches the start of the string.
#[pyfunction]
pub fn starts_with() -> Part {
    Part {
        pattern: r"^".to_string(),
    }
}

/// ends_with - Matches the end of the string.
#[pyfunction]
pub fn ends_with() -> Part {
    Part {
        pattern: r"$".to_string(),
    }
}


/// any_of - Matches any one of the provided patterns.
#[pyfunction]
#[pyo3(signature = (*parts))]
pub fn any_of(parts: Vec<Part>) -> PyResult<Part> {
    let patterns: Result<Vec<String>, PyErr> = parts.into_iter()
        .map(|part| Ok(part.pattern))
        .collect();
    let patterns = patterns?;
    Ok(Part {
        pattern: format!("({})", patterns.join("|")),
    })
}


/// alfanumeric - Matches any alphanumeric character.
#[pyfunction]
pub fn alfanumeric() -> Part {
    Part {
        pattern: r"\w".to_string(),
    }
}

/// alphabetic - Matches any alphabetic character.
#[pyfunction]
pub fn alphabetic() -> Part {
    Part {
        pattern: r"([a-zA-Z])".to_string(),
    }
}