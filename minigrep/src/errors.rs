use std::error;
use std::fmt;

#[derive(Debug)]
pub enum CustomError {
    InsufficientSize(i32),
}

// Implement Display trait
impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomError::InsufficientSize(val) => {
                write!(f, "Insufficient size. Required {val}")
            }
        }
    }
}

impl error::Error for CustomError {}