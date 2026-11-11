//! Núcleo compartido de Nexo.
//!
//! Esta librería contiene los tipos que deben hablar el mismo idioma en la
//! desktop y en el servidor: identificadores, dinero, operaciones de
//! sincronización y cursores.
//!
//! `nexo-core` es un proyecto independiente. No es un workspace, no contiene
//! lógica de UI ni de base de datos, y no depende de `nexo-desktop` ni de
//! `nexo-server`.

pub mod error;
pub mod ids;
pub mod money;
pub mod sync;

pub use error::CoreError;
pub use ids::{
    BranchId, BusinessId, CashMovementId, CashSessionId, CategoryId, CustomerId, DeviceId,
    OperationId, PaymentId, ProductId, PurchaseId, SaleId, StockMovementId, SupplierId, UserId,
};
pub use money::{Currency, Money};
pub use sync::{Operation, OperationKind, SyncCursor};
pub use uuid::Uuid;
