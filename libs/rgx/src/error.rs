

use thiserror::Error;

#[derive(Error, Debug)]
pub enum RgxError {
    #[error("Pattern compilation error")]
    PatternError,
}