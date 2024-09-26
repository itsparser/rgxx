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

/// digit - Matches any single digit (`\d`).
#[pyfunction]
pub fn digit() -> RegexPart {
    RegexPart {
        pattern: r"\d".to_string(),
    }
}

/// exactly - Matches the exact string `s`, escaping special regex characters.
#[pyfunction]
pub fn exactly(s: &str) -> RegexPart {
    RegexPart {
        pattern: escape_special_characters(s),
    }
}

/// any_of - Matches any one of the provided patterns.
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


/// alfanumeric - Matches any alphanumeric character.
#[pyfunction]
pub fn alfanumeric() -> RegexPart {
    RegexPart {
        pattern: r"\w".to_string(),
    }
}

/// alphabetic - Matches any alphabetic character.
#[pyfunction]
pub fn alphabetic() -> RegexPart {
    RegexPart {
        pattern: r"([a-zA-Z])".to_string(),
    }
}