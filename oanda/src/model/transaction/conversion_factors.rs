use serde::{Deserialize, Serialize};
use serde_with::serde_as;

/// A HomeConversionFactors message contains information used to convert
/// amounts, from an Instrument’s base or quote currency, to the home
/// currency of an Account.
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HomeConversionFactors {
    /// The ConversionFactor in effect for the Account for converting any gains
    /// realized in Instrument quote units into units of the Account’s home
    /// currency.
    pub gain_quote_home: ConversionFactor,

    /// The ConversionFactor in effect for the Account for converting any losses
    /// realized in Instrument quote units into units of the Account’s home
    /// currency.
    pub loss_quote_home: ConversionFactor,

    /// The ConversionFactor in effect for the Account for converting any gains
    /// realized in Instrument base units into units of the Account’s home
    /// currency.
    pub gain_base_home: ConversionFactor,

    /// The ConversionFactor in effect for the Account for converting any losses
    /// realized in Instrument base units into units of the Account’s home
    /// currency.
    pub loss_base_home: ConversionFactor,
}

/// A ConversionFactor contains information used to convert an amount,
/// from an Instrument’s base or quote currency, to the home currency
/// of an Account.
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde_as]
pub struct ConversionFactor {
    /// The factor by which to multiply the amount in the given currency to
    /// obtain the amount in the home currency of the Account.
    #[serde_as(as = "DisplayFromStr")]
    pub factor: f32,
}
