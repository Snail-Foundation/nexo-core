//! Shared core for Nexo.
//!
//! This library contains the types that must speak the same language in the
//! desktop and the server: identifiers, money, synchronization operations, and
//! cursors.
//!
//! `nexo-core` is an independent project. It is not a workspace, contains no UI
//! or database logic, and does not depend on `nexo-desktop` or `nexo-server`.

pub mod error;
pub mod ids;
pub mod money;
pub mod sync;

pub use error::CoreError;
pub use ids::{
    BranchId, BusinessId, CashMovementId, CashSessionId, CategoryId, CustomerId, DeviceId,
    OperationId, PaymentId, PaymentMethodId, ProductId, PurchaseId, SaleId, StockMovementId,
    SupplierId, TaxRateId, UnitId, UserId,
};
pub use money::{Currency, Money};
pub use sync::{Operation, OperationKind, SyncCursor};
pub use uuid::Uuid;
