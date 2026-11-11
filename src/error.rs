use thiserror::Error;

/// Core validation errors.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum CoreError {
    #[error("invalid identifier: {0}")]
    InvalidId(String),

    #[error("invalid currency: {0}")]
    InvalidCurrency(String),

    #[error("invalid amount: {0}")]
    InvalidAmount(String),

    #[error("invalid operation: {0}")]
    InvalidOperation(String),
}
