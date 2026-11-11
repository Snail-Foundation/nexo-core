use thiserror::Error;

/// Errores de validación del núcleo.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum CoreError {
    #[error("identificador inválido: {0}")]
    InvalidId(String),

    #[error("moneda inválida: {0}")]
    InvalidCurrency(String),

    #[error("monto inválido: {0}")]
    InvalidAmount(String),

    #[error("operación inválida: {0}")]
    InvalidOperation(String),
}
