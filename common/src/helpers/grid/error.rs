use std::fmt;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GridError {
    #[error("Conversion error: {0}")]
    ConversionError(String),

    #[error("Point ({x}, {y}) out of bounds ({rows}x{cols})")]
    OutOfBounds {
        x: String,
        y: String,
        rows: usize,
        cols: usize,
    },

    #[error("Builder error: {0}")]
    BuilderError(String),

    #[error("Operation error: {0}")]
    OperationError(String),
}

pub type Result<T> = std::result::Result<T, GridError>;

impl GridError {
    pub fn conversion<M: fmt::Display>(message: M) -> Self {
        Self::ConversionError(message.to_string())
    }

    pub fn out_of_bounds<T: fmt::Display>(x: T, y: T, rows: usize, cols: usize) -> Self {
        Self::OutOfBounds {
            x: x.to_string(),
            y: y.to_string(),
            rows,
            cols,
        }
    }

    pub fn builder<M: fmt::Display>(message: M) -> Self {
        Self::BuilderError(message.to_string())
    }

    pub fn operation<M: fmt::Display>(message: M) -> Self {
        Self::OperationError(message.to_string())
    }
}

impl From<std::num::TryFromIntError> for GridError {
    fn from(err: std::num::TryFromIntError) -> Self {
        Self::conversion(err.to_string())
    }
}
