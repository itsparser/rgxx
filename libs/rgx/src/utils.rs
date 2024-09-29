use crate::error::RgxError;
use crate::Part;

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
pub fn digit() -> Part {
    Part::new(None).digit().to_owned()
}

/// exactly - Matches the exact string `s`, escaping special regex characters.
pub fn exactly(s: &str) -> Part {
    Part::new(None).exactly(s).to_owned()
}

/// starts_with - Matches the start of the string.
pub fn starts_with() -> Part {
    Part::new(None).starts_with().to_owned()
}

/// ends_with - Matches the end of the string.
pub fn ends_with() -> Part {
    Part::new(None).ends_with().to_owned()
}


/// any_of - Matches any one of the provided patterns.
pub fn any_of(parts: Vec<Part>) -> Result<Part, RgxError> {
    let patterns: Result<Vec<String>, RgxError> = parts.into_iter()
        .map(|part| Ok(part.pattern))
        .collect();
    let patterns = patterns?;
    Ok(Part {
        pattern: format!("({})", patterns.join("|")),
    })
}


/// alfanumeric - Matches any alphanumeric character.
pub fn alfanumeric() -> Part {
    Part::new(None).alfanumeric().to_owned()
}

/// alphabetic - Matches any alphabetic character.
pub fn alphabetic() -> Part {
    Part::new(None).alphabetic().to_owned()
}