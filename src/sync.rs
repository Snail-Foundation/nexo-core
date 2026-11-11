use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{BranchId, CoreError, DeviceId, OperationId, Uuid};

macro_rules! operation_kinds {
    ($($variant:ident => $name:literal),+ $(,)?) => {
        /// Tipo de operación que viaja por el log de sincronización.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
        #[serde(rename_all = "snake_case")]
        #[non_exhaustive]
        pub enum OperationKind {
            $($variant),+
        }

        impl OperationKind {
            /// Nombre estable usado en la base de datos y en el protocolo.
            pub const fn as_str(self) -> &'static str {
                match self {
                    $(Self::$variant => $name),+
                }
            }
        }

        impl std::str::FromStr for OperationKind {
            type Err = CoreError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                match value {
                    $($name => Ok(Self::$variant),)+
                    other => Err(CoreError::InvalidOperation(format!(
                        "tipo de operación desconocido: {other}"
                    ))),
                }
            }
        }
    };
}

operation_kinds! {
    SaleCreated => "sale_created",
    SaleCancelled => "sale_cancelled",
    PaymentRecorded => "payment_recorded",
    StockMoved => "stock_moved",
    PurchaseRecorded => "purchase_recorded",
    ProductUpserted => "product_upserted",
    ProductArchived => "product_archived",
    PriceChanged => "price_changed",
    SupplierUpserted => "supplier_upserted",
    CustomerUpserted => "customer_upserted",
    CashSessionOpened => "cash_session_opened",
    CashSessionClosed => "cash_session_closed",
    CashMovementRecorded => "cash_movement_recorded",
    SettingChanged => "setting_changed",
}

/// Operación append-only generada por un dispositivo.
///
/// La desktop escribe la operación de negocio y esta entrada de outbox en la
/// misma transacción local. El servidor la recibe, valida su idempotencia y le
/// asigna un número de secuencia global.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    pub id: OperationId,
    pub device_id: DeviceId,
    pub branch_id: Option<BranchId>,
    pub kind: OperationKind,
    pub entity_id: Uuid,
    pub payload: Value,
    pub created_at_ms: i64,
    pub schema_version: u16,
}

impl Operation {
    pub const CURRENT_SCHEMA_VERSION: u16 = 1;

    pub fn new(
        device_id: DeviceId,
        branch_id: Option<BranchId>,
        kind: OperationKind,
        entity_id: Uuid,
        payload: Value,
        created_at_ms: i64,
    ) -> Self {
        Self {
            id: OperationId::new(),
            device_id,
            branch_id,
            kind,
            entity_id,
            payload,
            created_at_ms,
            schema_version: Self::CURRENT_SCHEMA_VERSION,
        }
    }

    /// Valida los campos mínimos antes de persistir o enviar la operación.
    pub fn validate(&self) -> Result<(), CoreError> {
        if self.id.is_nil() {
            return Err(CoreError::InvalidOperation(
                "el id de la operación es nulo".to_owned(),
            ));
        }

        if self.device_id.is_nil() {
            return Err(CoreError::InvalidOperation(
                "el dispositivo es obligatorio".to_owned(),
            ));
        }

        if self.branch_id.is_some_and(|branch_id| branch_id.is_nil()) {
            return Err(CoreError::InvalidOperation(
                "la sucursal no puede ser nula".to_owned(),
            ));
        }

        if self.entity_id.is_nil() {
            return Err(CoreError::InvalidOperation(
                "la entidad afectada es obligatoria".to_owned(),
            ));
        }

        if self.created_at_ms < 0 {
            return Err(CoreError::InvalidOperation(
                "la fecha de creación no puede ser negativa".to_owned(),
            ));
        }

        if self.schema_version == 0 {
            return Err(CoreError::InvalidOperation(
                "la versión de esquema debe ser mayor que cero".to_owned(),
            ));
        }

        if self.payload.is_null() {
            return Err(CoreError::InvalidOperation(
                "el payload no puede ser nulo".to_owned(),
            ));
        }

        Ok(())
    }
}

/// Posición del último cambio aplicado desde el servidor.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct SyncCursor(u64);

impl SyncCursor {
    pub const ZERO: Self = Self(0);

    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u64 {
        self.0
    }

    /// Avanza el cursor sin retroceder.
    pub fn advance_to(&mut self, value: u64) {
        if value > self.0 {
            self.0 = value;
        }
    }
}

impl From<u64> for SyncCursor {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<SyncCursor> for u64 {
    fn from(cursor: SyncCursor) -> Self {
        cursor.0
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn operation_round_trips_through_json() {
        let operation = Operation::new(
            DeviceId::new(),
            None,
            OperationKind::SaleCreated,
            Uuid::now_v7(),
            json!({ "total_minor_units": 1500, "currency": "USD" }),
            1_700_000_000_000,
        );

        operation.validate().unwrap();

        let encoded = serde_json::to_string(&operation).unwrap();
        let decoded: Operation = serde_json::from_str(&encoded).unwrap();

        assert_eq!(decoded, operation);
    }

    #[test]
    fn operation_kind_names_match_serde() {
        let kinds = [
            OperationKind::SaleCreated,
            OperationKind::SaleCancelled,
            OperationKind::PaymentRecorded,
            OperationKind::StockMoved,
            OperationKind::PurchaseRecorded,
            OperationKind::ProductUpserted,
            OperationKind::ProductArchived,
            OperationKind::PriceChanged,
            OperationKind::SupplierUpserted,
            OperationKind::CustomerUpserted,
            OperationKind::CashSessionOpened,
            OperationKind::CashSessionClosed,
            OperationKind::CashMovementRecorded,
            OperationKind::SettingChanged,
        ];

        for kind in kinds {
            let encoded = serde_json::to_string(&kind).unwrap();
            assert_eq!(encoded, format!("\"{}\"", kind.as_str()));
            assert_eq!(kind.as_str().parse::<OperationKind>().unwrap(), kind);
        }
    }

    #[test]
    fn operation_rejects_empty_payload() {
        let operation = Operation::new(
            DeviceId::new(),
            None,
            OperationKind::SaleCreated,
            Uuid::now_v7(),
            Value::Null,
            1_700_000_000_000,
        );

        assert!(operation.validate().is_err());
    }

    #[test]
    fn cursor_never_moves_backwards() {
        let mut cursor = SyncCursor::new(10);
        cursor.advance_to(5);
        assert_eq!(cursor.value(), 10);

        cursor.advance_to(11);
        assert_eq!(cursor.value(), 11);
    }
}
