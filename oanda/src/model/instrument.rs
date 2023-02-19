use serde::{Deserialize, Serialize};

use serde_with::{serde_as, DisplayFromStr};

#[derive(Debug, Deserialize)]
pub struct Instruments {
    pub instruments: Vec<Instrument>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Instrument {
    /// The name of the instrument. eg. EUR_USD
    pub name: String,

    /// The type of the instrument
    #[serde(rename = "type")]
    pub instrument_type: InstrumentType,

    /// The display name of the Instrument
    pub display_name: String,

    /// The location of the “pip” for this instrument. The decimal
    /// position of the pip in this Instrument’s price can be found
    /// at 10 ^ pipLocation (e.g.  -4 pipLocation results in a decimal
    /// pip position of 10 ^ -4 = 0.0001).
    pub pip_location: i32,

    /// The number of decimal places that should be used to display
    /// prices for this instrument. (e.g. a displayPrecision of 5 would
    /// result in a price of “1” being displayed as “1.00000”)
    pub display_precision: i32,

    /// The amount of decimal places that may be provided when specifying
    /// the number of units traded for this instrument.
    pub trade_units_precision: i32,

    /// The smallest number of units allowed to be traded for this instrument.
    #[serde_as(as = "DisplayFromStr")]
    pub minimum_trade_size: f32,

    /// The maximum trailing stop distance allowed for a trailing stop
    /// loss created for this instrument. Specified in price units.
    #[serde_as(as = "DisplayFromStr")]
    pub maximum_trailing_stop_distance: f32,

    /// The minimum distance allowed between the Trade’s fill price
    /// and the configured price for guaranteed Stop Loss Orders created
    /// for this instrument. Specified in price units.
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub minimum_guaranteed_stop_loss_distance: Option<f32>,

    /// The minimum trailing stop distance allowed for a trailing stop
    /// loss created for this instrument. Specified in price units.
    #[serde_as(as = "DisplayFromStr")]
    pub minimum_trailing_stop_distance: f32,

    /// The maximum position size allowed for this instrument. Specified
    /// in units.
    #[serde_as(as = "DisplayFromStr")]
    pub maximum_position_size: u32,

    /// The maximum units allowed for an Order placed for this instrument.
    /// Specified in units.
    #[serde_as(as = "DisplayFromStr")]
    pub maximum_order_units: u32,

    /// The margin rate for this instrument.
    #[serde_as(as = "DisplayFromStr")]
    pub margin_rate: f32,

    /// The commission structure for this instrument.
    pub commission: InstrumentCommission,

    /// The current Guaranteed Stop Loss Order mode of the Account for
    /// this Instrument.
    pub guaranteed_stop_loss_order_mode: GuaranteedStopLossOrderModeForInstrument,

    /// The amount that is charged to the account if a guaranteed Stop Loss Order
    /// is triggered and filled. The value is in price units and is charged for
    /// each unit of the Trade. This field will only be present if the Account’s
    /// guaranteedStopLossOrderMode for this Instrument is not ‘DISABLED’.
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub guaranteed_stop_loss_order_execution_premium: Option<f32>,

    /// The guaranteed Stop Loss Order level restriction for this instrument.
    /// This field will only be present if the Account’s guaranteedStopLossOrderMode
    /// for this Instrument is not ‘DISABLED’.
    pub guaranteed_stop_loss_order_level_restriction:
        Option<GuaranteedStopLossOrderLevelRestriction>,

    /// Financing data for this instrument.
    pub financing: InstrumentFinancing,

    /// The tags associated with this instrument.
    pub tags: Vec<Tag>,
}

/// The type of an instrument
/// [See docs](https://developer.oanda.com/rest-live-v20/primitives-df/#InstrumentType)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InstrumentType {
    /// Represents a Currency instrument type
    Currency,
    /// Represents a Contract For Difference instrument type
    #[serde(rename = "CFD")]
    ContractForDifference,
    /// Represents a Metal instrument type
    Metal,
}

/// The overall behaviour of the Account regarding Guaranteed Stop Loss
/// Orders for a specific Instrument.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GuaranteedStopLossOrderModeForInstrument {
    /// The Account is not permitted to create Guaranteed Stop Loss Orders for this Instrument.
    Disabled,
    /// The Account is able, but not required to have Guaranteed Stop Loss Orders for open Trades for this Instrument.
    Allowed,
    /// The Account is required to have Guaranteed Stop Loss Orders for all open Trades for this Instrument.
    Required,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentCommission {
    /// The commission amount (in the Account’s home currency) charged per
    /// unitsTraded of the instrument.
    #[serde_as(as = "DisplayFromStr")]
    pub commission: f32,

    /// The number of units traded that the commission amount is based on.
    #[serde_as(as = "DisplayFromStr")]
    pub units_traded: f32,

    /// The minimum commission amount (in the Account’s home currency) that is
    /// charged when an Order is filled for this instrument.
    #[serde_as(as = "DisplayFromStr")]
    pub minimum_commission: f32,
}

#[serde_as]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GuaranteedStopLossOrderLevelRestriction {
    /// Applies to Trades with a guaranteed Stop Loss Order attached for the
    /// specified Instrument. This is the total allowed Trade volume that can
    /// exist within the priceRange based on the trigger prices of the guaranteed
    /// Stop Loss Orders.
    #[serde_as(as = "DisplayFromStr")]
    pub volume: f32,
    /// The price range the volume applies to. This value is in price units.
    #[serde_as(as = "DisplayFromStr")]
    pub price_range: f32,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentFinancing {
    /// The financing rate to be used for a long position for the instrument. The
    /// value is in decimal rather than percentage points, i.e. 5% is represented
    /// as 0.05.
    #[serde_as(as = "DisplayFromStr")]
    pub long_rate: f32,

    /// The financing rate to be used for a short position for the instrument.
    /// The value is in decimal rather than percentage points, i.e. 5% is
    /// represented as 0.05.
    #[serde_as(as = "DisplayFromStr")]
    pub short_rate: f32,

    /// The days of the week to debit or credit financing charges; the exact time
    /// of day at which to charge the financing is set in the
    /// DivisionTradingGroup for the client’s account.
    pub financing_days_of_week: Vec<FinancingDayOfWeek>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FinancingDayOfWeek {
    /// The day of the week to charge the financing.
    pub day_of_week: DayOfWeek,

    /// The number of days worth of financing to be charged on dayOfWeek.
    pub days_charged: i32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DayOfWeek {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

/// A tag associated with an instrument.
#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    /// The type of the tag.
    #[serde(rename = "type")]
    pub tag_type: String,

    /// The name of the tag.
    pub name: String,
}
