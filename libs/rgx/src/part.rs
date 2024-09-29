use crate::error::RgxError;
use crate::utils::escape_special_characters;

#[derive(Clone)]
pub struct Part {
    pub pattern: String,
}

impl Part {
    pub fn new(pattern: Option<&str>) -> Self {
        Self {
            pattern: pattern.unwrap_or("").to_string(),
        }
    }

    /// times - Repeats the pattern exactly `n` times.
    pub fn times(&mut self, count: usize) -> &mut Self {
        self.pattern.push_str(&format!("{{{}}}", count));
        self
    }

    /// starts_with - Matches the start of the string.
    pub fn starts_with(&mut self) -> &mut Self {
        self.pattern.push_str("^");
        self
    }

    /// ends_with - Matches the end of the string.
    pub fn ends_with(&mut self) -> &mut Self {
        self.pattern.push_str("$");
        self
    }
    /// group_as - Name the capture group as `name`.
    pub fn group_as(&mut self, name: &str) -> &mut Self {
        self.pattern.push_str(&format!("(?P<{}>", name));
        self
    }
    /// add - Concatenates the current pattern with another.
    pub fn add(&mut self, other: &Part) -> &mut Self {
        self.pattern.push_str(&other.pattern);
        self
    }

    /// digit - Matches any single digit (`\d`).
    pub fn digit(&mut self) -> &mut Self {
        self.pattern.push_str("\\d");
        self
    }

    /// exactly - Matches the exact string `s`, escaping special regex characters.
    pub fn exactly(&mut self, s: &str) -> &mut Self {
        self.pattern.push_str(&escape_special_characters(s));
        self
    }

    /// alfanumeric - Matches any alphanumeric character.
    pub fn alfanumeric(&mut self) -> &mut Self {
        self.pattern.push_str("\\w");
        self
    }
    /// alphabetic - Matches any alphabetic character.
    pub fn alphabetic(&mut self) -> &mut Self {
        self.pattern.push_str("([a-zA-Z])");
        self
    }

    /// any_of - Matches any one of the provided patterns.
    pub fn any_of(&mut self, parts: Vec<Part>) -> Result<&mut Self, RgxError> {
        let patterns: Result<Vec<String>, RgxError> = parts.into_iter()
            .map(|part| Ok(part.pattern))
            .collect();
        let patterns = patterns?;
        self.pattern.push_str(&format!("({})", patterns.join("|")));
        Ok(self)
    }

    // /// exactly - Matches the exact string `s`, escaping special regex characters.
    // pub fn exactly(&mut self, parts: Vec<Part>) -> Result<&mut Self, Err> {
    //     let patterns: Result<Vec<String>, Err> = parts.into_iter()
    //         .map(|part| Ok(part.pattern))
    //         .collect();
    //     let patterns = patterns?;
    //     self.pattern.push_str(&patterns.join(""));
    //     Ok(self)
    // }

    /// any_character - Matches any character.
    pub fn any_character(&mut self) -> &mut Self {
        self.pattern.push_str(".");
        self
    }

    /// infinity - Matches any character.
    pub fn infinity(&mut self) -> &mut Self {
        self.pattern.push_str("+");
        self
    }

}
