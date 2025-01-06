use std::fmt;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PointError {
    #[error("Point conversion failed: {message}")]
    ConversionError { message: String },

    #[error("Point ({x}, {y}) is outside valid bounds {bounds}")]
    OutOfBounds {
        x: String,
        y: String,
        bounds: String,
    },

    #[error("Arithmetic error in {operation}: {message}")]
    ArithmeticError { operation: String, message: String },
}

pub type Result<T> = std::result::Result<T, PointError>;

impl PointError {
    pub fn conversion<M: fmt::Display>(message: M) -> Self {
        Self::ConversionError {
            message: message.to_string(),
        }
    }

    pub fn out_of_bounds<T: fmt::Display>(x: T, y: T, bounds: impl fmt::Display) -> Self {
        Self::OutOfBounds {
            x: x.to_string(),
            y: y.to_string(),
            bounds: bounds.to_string(),
        }
    }

    pub fn arithmetic<M: fmt::Display>(operation: &str, message: M) -> Self {
        Self::ArithmeticError {
            operation: operation.to_string(),
            message: message.to_string(),
        }
    }
}

impl From<std::num::TryFromIntError> for PointError {
    fn from(err: std::num::TryFromIntError) -> Self {
        Self::conversion(err.to_string())
    }
}
