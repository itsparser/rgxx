use crate::error::RgxError;
use crate::part::Part;


/// RGX - Natural Language Regex Generator Expression for Rust
///
/// A minimal rust library to generate regular expressions using natural language chainable functions.
///
///
/// ## Features
///
/// *   **Zero-Dependency and Ultra-Minimal Runtime**: Designed to be lightweight with no external dependencies, ensuring fast execution and minimal overhead.
/// *   **Pure RegExp Compilation**: Generates standard regular expressions compatible with Python's `re` module and other regex engines.
/// *   **Automatically Typed Capture Groups**: Easily define named capture groups that are automatically recognized, simplifying pattern matching and data extraction.
/// *   **Natural Language Syntax**: Utilize chainable functions that read like natural language, enhancing code readability and maintainability.
/// *   **IDE Support**: Generated regular expressions display on hover in supported IDEs, aiding in development and debugging.
///
/// ## example
/// ```
/// use rgx::{RegExp, exactly, any_of, Part};
///
/// let date_pattern = RegExp::new(vec![
///     Part::new(None).exactly("2023").group_as("year").to_owned(),
///     Part::new(None).exactly("-").to_owned(),
///     Part::new(None).exactly("10").group_as("month").to_owned(),
///     Part::new(None).exactly("-").to_owned(),
///     Part::new(None).exactly("05").group_as("day").to_owned(),
/// ]).unwrap();
///
/// assert_eq!(date_pattern.compile().unwrap(), "(?P<year>2023)-(?P<month>10)-(?P<day>05)");
/// ```
#[derive(Clone)]
pub struct RegExp {
    pub pattern: String,
}

impl RegExp {
    pub fn new(parts: Vec<Part>) -> Result<Self, RgxError> {
        let patterns: Result<Vec<String>, RgxError> = parts.into_iter()
            .map(|part| Ok(part.pattern))
            .collect();
        let patterns = patterns?;
        Ok(RegExp {
            pattern: patterns.join(""),
        })
    }
    pub fn compile(&self) -> Result<String, RgxError> {
        Ok(self.pattern.clone())
    }
}