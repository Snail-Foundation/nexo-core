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
            /// Generates a time-ordered UUIDv7 identifier.
            pub fn new() -> Self {
                Self(Uuid::now_v7())
            }

            /// Wraps an existing UUID.
            pub fn from_uuid(value: Uuid) -> Self {
                Self(value)
            }

            /// Returns the underlying UUID.
            pub fn as_uuid(&self) -> &Uuid {
                &self.0
            }

            /// Consumes the identifier and returns the UUID.
            pub fn into_uuid(self) -> Uuid {
                self.0
            }

            /// Returns true if the identifier is the nil UUID.
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
    /// Business identifier.
    BusinessId
);
define_id!(
    /// Branch identifier.
    BranchId
);
define_id!(
    /// Device identifier.
    DeviceId
);
define_id!(
    /// User identifier.
    UserId
);
define_id!(
    /// Product identifier.
    ProductId
);
define_id!(
    /// Category identifier.
    CategoryId
);
define_id!(
    /// Unit of measure identifier.
    UnitId
);
define_id!(
    /// Payment method identifier.
    PaymentMethodId
);
define_id!(
    /// Tax rate identifier.
    TaxRateId
);
define_id!(
    /// Supplier identifier.
    SupplierId
);
define_id!(
    /// Purchase identifier.
    PurchaseId
);
define_id!(
    /// Sale identifier.
    SaleId
);
define_id!(
    /// Payment identifier.
    PaymentId
);
define_id!(
    /// Inventory movement identifier.
    StockMovementId
);
define_id!(
    /// Cash session identifier.
    CashSessionId
);
define_id!(
    /// Cash movement identifier.
    CashMovementId
);
define_id!(
    /// Customer identifier.
    CustomerId
);
define_id!(
    /// Unique identifier for a synchronization operation.
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
        assert!("not-a-uuid".parse::<DeviceId>().is_err());
    }
}
