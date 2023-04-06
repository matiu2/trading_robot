use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

/// The price for your account
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde_as]
pub struct ClientPrice {
    /// The string “PRICE”. Used to identify the a Price object when found in a
    /// stream.
    #[serde(rename = "type")]
    pub type_: String,

    /// The Price’s Instrument.
    pub instrument: String,

    /// The date/time when the Price was created
    pub time: DateTime<Utc>,

    /// Flag indicating if the Price is tradeable or not
    pub tradeable: bool,

    /// The list of prices and liquidity available on the Instrument’s bid side.
    /// It is possible for this list to be empty if there is no bid liquidity
    /// currently available for the Instrument in the Account.
    pub bids: Vec<PriceBucket>,

    /// The list of prices and liquidity available on the Instrument’s ask side.
    /// It is possible for this list to be empty if there is no ask liquidity
    /// currently available for the Instrument in the Account.
    pub asks: Vec<PriceBucket>,

    /// The closeout bid Price. This Price is used when a bid is required to
    /// closeout a Position (margin closeout or manual) yet there is no bid
    /// liquidity. The closeout bid is never used to open a new position.
    #[serde_as(as = "DisplayFromStr")]
    pub closeout_bid: f32,

    /// The closeout ask Price. This Price is used when a ask is required to
    /// closeout a Position (margin closeout or manual) yet there is no ask
    /// liquidity. The closeout ask is never used to open a new position.
    #[serde_as(as = "DisplayFromStr")]
    pub closeout_ask: f32,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde_as]
pub struct PriceBucket {
    /// The Price offered by the PriceBucket
    #[serde_as(as = "DisplayFromStr")]
    pub price: f32,

    /// The amount of liquidity offered by the PriceBucket
    pub liquidity: f32,
}
