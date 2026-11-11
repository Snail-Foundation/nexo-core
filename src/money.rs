use std::fmt;

use serde::{Deserialize, Serialize};

use crate::error::CoreError;

/// ISO 4217 currency code in uppercase (for example `USD` or `VES`).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Currency(String);

impl Currency {
    pub const USD_CODE: &'static str = "USD";
    pub const VES_CODE: &'static str = "VES";

    /// Creates a currency and validates the three-letter format.
    pub fn new(code: impl Into<String>) -> Result<Self, CoreError> {
        let code = code.into().to_ascii_uppercase();

        if code.len() != 3 || !code.chars().all(|character| character.is_ascii_uppercase()) {
            return Err(CoreError::InvalidCurrency(code));
        }

        Ok(Self(code))
    }

    pub fn usd() -> Self {
        Self(Self::USD_CODE.to_owned())
    }

    pub fn ves() -> Self {
        Self(Self::VES_CODE.to_owned())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// Amount expressed in a currency's minor unit.
///
/// For example, `Money::usd(1250)` represents USD 12.50. An integer is used to
/// avoid floating-point errors in sales, payments, and cash counts.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Money {
    minor_units: i64,
    currency: Currency,
}

impl Money {
    pub fn new(minor_units: i64, currency: Currency) -> Self {
        Self {
            minor_units,
            currency,
        }
    }

    pub fn usd(minor_units: i64) -> Self {
        Self::new(minor_units, Currency::usd())
    }

    pub fn ves(minor_units: i64) -> Self {
        Self::new(minor_units, Currency::ves())
    }

    pub fn zero(currency: Currency) -> Self {
        Self::new(0, currency)
    }

    pub fn minor_units(&self) -> i64 {
        self.minor_units
    }

    pub fn currency(&self) -> &Currency {
        &self.currency
    }

    pub fn is_zero(&self) -> bool {
        self.minor_units == 0
    }

    pub fn is_negative(&self) -> bool {
        self.minor_units < 0
    }

    /// Adds two amounts, returning an error on currency mismatch or overflow.
    pub fn checked_add(&self, other: &Self) -> Result<Self, CoreError> {
        self.ensure_same_currency(other)?;

        let minor_units = self
            .minor_units
            .checked_add(other.minor_units)
            .ok_or_else(|| CoreError::InvalidAmount("overflow while adding".to_owned()))?;

        Ok(Self::new(minor_units, self.currency.clone()))
    }

    /// Subtracts two amounts, returning an error on currency mismatch or
    /// overflow.
    pub fn checked_sub(&self, other: &Self) -> Result<Self, CoreError> {
        self.ensure_same_currency(other)?;

        let minor_units = self
            .minor_units
            .checked_sub(other.minor_units)
            .ok_or_else(|| CoreError::InvalidAmount("overflow while subtracting".to_owned()))?;

        Ok(Self::new(minor_units, self.currency.clone()))
    }

    fn ensure_same_currency(&self, other: &Self) -> Result<(), CoreError> {
        if self.currency != other.currency {
            return Err(CoreError::InvalidAmount(format!(
                "cannot operate on amounts in {} and {}",
                self.currency, other.currency
            )));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn currency_is_normalized() {
        assert_eq!(Currency::new("usd").unwrap().as_str(), "USD");
        assert!(Currency::new("US").is_err());
        assert!(Currency::new("US1").is_err());
    }

    #[test]
    fn money_adds_and_subtracts_in_minor_units() {
        let price = Money::usd(1250);
        let tax = Money::usd(250);

        assert_eq!(price.checked_add(&tax).unwrap().minor_units(), 1500);
        assert_eq!(price.checked_sub(&tax).unwrap().minor_units(), 1000);
    }

    #[test]
    fn money_rejects_mixed_currencies() {
        assert!(Money::usd(100).checked_add(&Money::ves(100)).is_err());
    }
}
