mod stop_loss;
use super::trade::TimeInForce;
use crate::model::trade::ClientExtensions;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
pub use stop_loss::{SLTrigger, StopLoss, TrailingStopLoss};

#[serde_as]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct TakeProfitDetails {
    /// The price that the Take Profit Order will be triggered at. Only one of
    /// the price and distance fields may be specified.
    #[serde_as(as = "DisplayFromStr")]
    pub price: f32,

    /// The time in force for the created Take Profit Order. This may only be
    /// GTC, GTD or GFD.
    pub time_in_force: TimeInForce,

    /// The date when the Take Profit Order will be cancelled on if timeInForce
    /// is GTD.
    pub gtd_time: Option<DateTime<Utc>>,

    /// The Client Extensions to add to the Take Profit Order when created.
    pub client_extensions: Option<ClientExtensions>,
}
