//! Restaurant domain types shared by the desktop, server, and web app.

use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::error::CoreError;
use crate::ids::TaxRateId;

/// How an order is served.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderType {
    DineIn,
    Takeout,
}

impl OrderType {
    /// Stable name used in the database and in the operation payloads.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DineIn => "dine_in",
            Self::Takeout => "takeout",
        }
    }

    /// Returns true when the order must reference a dining table.
    pub const fn requires_table(self) -> bool {
        matches!(self, Self::DineIn)
    }
}

impl FromStr for OrderType {
    type Err = CoreError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "dine_in" => Ok(Self::DineIn),
            "takeout" => Ok(Self::Takeout),
            other => Err(CoreError::InvalidStatus(format!(
                "unknown order type: {other}"
            ))),
        }
    }
}

/// Lifecycle of an order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    /// No lines have been sent to the kitchen yet.
    Open,
    /// At least one line is pending, sent, ready, or served.
    Active,
    /// All non-voided lines have been served.
    Served,
    /// The order has a sale.
    Billed,
    /// The order was cancelled with a reason.
    Cancelled,
}

impl OrderStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Active => "active",
            Self::Served => "served",
            Self::Billed => "billed",
            Self::Cancelled => "cancelled",
        }
    }

    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Billed | Self::Cancelled)
    }

    /// Returns true when the order can move to `next`.
    ///
    /// A served order can become active again when a new round is added. An
    /// active or served order can be billed before every line is served.
    pub const fn can_transition_to(self, next: Self) -> bool {
        matches!(
            (self, next),
            (Self::Open, Self::Active)
                | (Self::Open, Self::Cancelled)
                | (Self::Active, Self::Served)
                | (Self::Active, Self::Billed)
                | (Self::Active, Self::Cancelled)
                | (Self::Served, Self::Active)
                | (Self::Served, Self::Billed)
                | (Self::Served, Self::Cancelled)
        )
    }
}

impl FromStr for OrderStatus {
    type Err = CoreError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "open" => Ok(Self::Open),
            "active" => Ok(Self::Active),
            "served" => Ok(Self::Served),
            "billed" => Ok(Self::Billed),
            "cancelled" => Ok(Self::Cancelled),
            other => Err(CoreError::InvalidStatus(format!(
                "unknown order status: {other}"
            ))),
        }
    }
}

/// Lifecycle of a single order line.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderLineStatus {
    /// Added but not sent to the kitchen.
    Pending,
    /// Sent to the kitchen.
    Sent,
    /// Finished by the kitchen.
    Ready,
    /// Delivered to the table or handed to the takeout customer.
    Served,
    /// Removed with a reason; never billed.
    Voided,
}

impl OrderLineStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Sent => "sent",
            Self::Ready => "ready",
            Self::Served => "served",
            Self::Voided => "voided",
        }
    }

    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Served | Self::Voided)
    }

    /// Returns true when the line can move to `next`.
    pub const fn can_transition_to(self, next: Self) -> bool {
        matches!(
            (self, next),
            (Self::Pending, Self::Sent)
                | (Self::Pending, Self::Voided)
                | (Self::Sent, Self::Ready)
                | (Self::Sent, Self::Served)
                | (Self::Sent, Self::Voided)
                | (Self::Ready, Self::Served)
                | (Self::Ready, Self::Voided)
        )
    }
}

impl FromStr for OrderLineStatus {
    type Err = CoreError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "pending" => Ok(Self::Pending),
            "sent" => Ok(Self::Sent),
            "ready" => Ok(Self::Ready),
            "served" => Ok(Self::Served),
            "voided" => Ok(Self::Voided),
            other => Err(CoreError::InvalidStatus(format!(
                "unknown order line status: {other}"
            ))),
        }
    }
}

/// Global IVA configuration.
///
/// Menu prices are final and include IVA when enabled. The rate is expressed in
/// basis points: `1600` means 16%.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaxRate {
    pub id: TaxRateId,
    pub name: String,
    pub rate_basis_points: u16,
    pub enabled: bool,
}

impl TaxRate {
    pub const MAX_BASIS_POINTS: u16 = 10_000;

    /// Creates a validated tax rate.
    pub fn new(
        id: TaxRateId,
        name: impl Into<String>,
        rate_basis_points: u16,
        enabled: bool,
    ) -> Result<Self, CoreError> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(CoreError::InvalidTaxRate("name is required".to_owned()));
        }

        if rate_basis_points > Self::MAX_BASIS_POINTS {
            return Err(CoreError::InvalidTaxRate(
                "rate cannot exceed 100%".to_owned(),
            ));
        }

        Ok(Self {
            id,
            name,
            rate_basis_points,
            enabled,
        })
    }

    /// Returns the IVA portion already included in a final total.
    ///
    /// The result is rounded half up to the nearest minor unit. A disabled tax
    /// rate always returns zero.
    pub fn included_tax_minor_units(&self, total_minor_units: i64) -> Result<i64, CoreError> {
        if total_minor_units < 0 {
            return Err(CoreError::InvalidAmount(
                "total cannot be negative".to_owned(),
            ));
        }

        if !self.enabled || self.rate_basis_points == 0 || total_minor_units == 0 {
            return Ok(0);
        }

        let denominator = 10_000i128 + i128::from(self.rate_basis_points);
        let numerator = i128::from(total_minor_units) * i128::from(self.rate_basis_points);
        let tax = (numerator + denominator / 2) / denominator;

        i64::try_from(tax)
            .map_err(|_| CoreError::InvalidAmount("tax amount is too large".to_owned()))
    }

    /// Returns the net amount after removing the included IVA.
    pub fn net_minor_units(&self, total_minor_units: i64) -> Result<i64, CoreError> {
        let tax = self.included_tax_minor_units(total_minor_units)?;

        total_minor_units
            .checked_sub(tax)
            .ok_or_else(|| CoreError::InvalidAmount("net amount is too small".to_owned()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn order_types_match_serde_and_table_rules() {
        for order_type in [OrderType::DineIn, OrderType::Takeout] {
            let encoded = serde_json::to_string(&order_type).unwrap();
            assert_eq!(encoded, format!("\"{}\"", order_type.as_str()));
            assert_eq!(
                order_type.as_str().parse::<OrderType>().unwrap(),
                order_type
            );
        }

        assert!(OrderType::DineIn.requires_table());
        assert!(!OrderType::Takeout.requires_table());
    }

    #[test]
    fn order_status_transitions_are_explicit() {
        assert!(OrderStatus::Open.can_transition_to(OrderStatus::Active));
        assert!(OrderStatus::Open.can_transition_to(OrderStatus::Cancelled));
        assert!(OrderStatus::Active.can_transition_to(OrderStatus::Served));
        assert!(OrderStatus::Active.can_transition_to(OrderStatus::Billed));
        assert!(OrderStatus::Served.can_transition_to(OrderStatus::Active));
        assert!(OrderStatus::Served.can_transition_to(OrderStatus::Billed));

        assert!(!OrderStatus::Open.can_transition_to(OrderStatus::Billed));
        assert!(!OrderStatus::Billed.can_transition_to(OrderStatus::Cancelled));
        assert!(!OrderStatus::Cancelled.can_transition_to(OrderStatus::Active));
    }

    #[test]
    fn order_status_names_match_serde() {
        let statuses = [
            OrderStatus::Open,
            OrderStatus::Active,
            OrderStatus::Served,
            OrderStatus::Billed,
            OrderStatus::Cancelled,
        ];

        for status in statuses {
            let encoded = serde_json::to_string(&status).unwrap();
            assert_eq!(encoded, format!("\"{}\"", status.as_str()));
            assert_eq!(status.as_str().parse::<OrderStatus>().unwrap(), status);
        }
    }

    #[test]
    fn order_line_status_transitions_are_explicit() {
        assert!(OrderLineStatus::Pending.can_transition_to(OrderLineStatus::Sent));
        assert!(OrderLineStatus::Sent.can_transition_to(OrderLineStatus::Ready));
        assert!(OrderLineStatus::Ready.can_transition_to(OrderLineStatus::Served));
        assert!(OrderLineStatus::Sent.can_transition_to(OrderLineStatus::Voided));

        assert!(!OrderLineStatus::Pending.can_transition_to(OrderLineStatus::Ready));
        assert!(!OrderLineStatus::Served.can_transition_to(OrderLineStatus::Voided));
        assert!(!OrderLineStatus::Voided.can_transition_to(OrderLineStatus::Pending));
    }

    #[test]
    fn tax_rate_removes_the_included_iva() {
        let tax_rate = TaxRate::new(TaxRateId::new(), "IVA", 1_600, true).unwrap();

        assert_eq!(tax_rate.included_tax_minor_units(1_160).unwrap(), 160);
        assert_eq!(tax_rate.net_minor_units(1_160).unwrap(), 1_000);

        assert_eq!(tax_rate.included_tax_minor_units(1_000).unwrap(), 138);
        assert_eq!(tax_rate.net_minor_units(1_000).unwrap(), 862);
    }

    #[test]
    fn disabled_tax_rate_returns_zero() {
        let tax_rate = TaxRate::new(TaxRateId::new(), "IVA", 1_600, false).unwrap();

        assert_eq!(tax_rate.included_tax_minor_units(1_160).unwrap(), 0);
        assert_eq!(tax_rate.net_minor_units(1_160).unwrap(), 1_160);
    }

    #[test]
    fn tax_rate_rejects_invalid_values() {
        assert!(TaxRate::new(TaxRateId::new(), "", 1_600, true).is_err());
        assert!(TaxRate::new(TaxRateId::new(), "IVA", 10_001, true).is_err());
    }
}
