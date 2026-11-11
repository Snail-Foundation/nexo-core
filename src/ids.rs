use std::{fmt, str::FromStr};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::CoreError;

macro_rules! define_id {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(
            Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
        )]
        #[serde(transparent)]
        pub struct $name(Uuid);

        impl $name {
            /// Genera un identificador UUIDv7 ordenable por tiempo.
            pub fn new() -> Self {
                Self(Uuid::now_v7())
            }

            /// Envuelve un UUID existente.
            pub fn from_uuid(value: Uuid) -> Self {
                Self(value)
            }

            /// Devuelve el UUID subyacente.
            pub fn as_uuid(&self) -> &Uuid {
                &self.0
            }

            /// Consume el identificador y devuelve el UUID.
            pub fn into_uuid(self) -> Uuid {
                self.0
            }

            /// Indica si el identificador es el UUID nulo.
            pub fn is_nil(&self) -> bool {
                self.0.is_nil()
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }

        impl FromStr for $name {
            type Err = CoreError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Uuid::parse_str(value)
                    .map(Self)
                    .map_err(|error| CoreError::InvalidId(error.to_string()))
            }
        }

        impl From<Uuid> for $name {
            fn from(value: Uuid) -> Self {
                Self(value)
            }
        }

        impl From<$name> for Uuid {
            fn from(value: $name) -> Self {
                value.0
            }
        }
    };
}

define_id!(
    /// Identificador del negocio.
    BusinessId
);
define_id!(
    /// Identificador de una sucursal.
    BranchId
);
define_id!(
    /// Identificador de un dispositivo.
    DeviceId
);
define_id!(
    /// Identificador de un usuario.
    UserId
);
define_id!(
    /// Identificador de un producto.
    ProductId
);
define_id!(
    /// Identificador de una categoría.
    CategoryId
);
define_id!(
    /// Identificador de un proveedor.
    SupplierId
);
define_id!(
    /// Identificador de una compra.
    PurchaseId
);
define_id!(
    /// Identificador de una venta.
    SaleId
);
define_id!(
    /// Identificador de un pago.
    PaymentId
);
define_id!(
    /// Identificador de un movimiento de inventario.
    StockMovementId
);
define_id!(
    /// Identificador de una sesión de caja.
    CashSessionId
);
define_id!(
    /// Identificador de un movimiento de caja.
    CashMovementId
);
define_id!(
    /// Identificador de un cliente.
    CustomerId
);
define_id!(
    /// Identificador único de una operación de sincronización.
    OperationId
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids_are_unique_and_parseable() {
        let first = SaleId::new();
        let second = SaleId::new();

        assert_ne!(first, second);
        assert!(!first.is_nil());
        assert_eq!(first.to_string().parse::<SaleId>().unwrap(), first);
    }

    #[test]
    fn invalid_ids_are_rejected() {
        assert!("no-es-un-uuid".parse::<DeviceId>().is_err());
    }
}
