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


mod exp;
mod part;
mod utils;
pub mod error;

pub use crate::exp::RegExp;
pub use crate::part::Part;
pub use crate::utils::{digit, exactly, any_of};