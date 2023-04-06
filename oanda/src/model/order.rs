use crate::model::trade::{ClientExtensions, TimeInForce};
use crate::model::transaction::StopLoss;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

use super::{trade::MarketOrderTimeInForce, transaction::TakeProfitDetails};

/// Order structure
#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    /// The type of the Order to Create. Must be set to “MARKET” when creating a
    /// Market Order.
    #[serde(rename = "type")]
    pub order_type: OrderType,

    /// The Market Order’s Instrument.
    pub instrument: String,

    /// The quantity requested to be filled by the Market Order. A positive
    /// number of units results in a long Order, and a negative number of units
    /// results in a short Order.
    #[serde_as(as = "DisplayFromStr")]
    pub units: f32,

    /// The worst price that the client is willing to have the Market Order
    /// filled at.
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub price_bound: Option<f32>,

    /// Specification of how Positions in the Account are modified when the Order
    /// is filled.
    pub position_fill: OrderPositionFill,

    /// The client extensions to add to the Order. Do not set, modify, or delete
    /// clientExtensions if your account is associated with MT4.
    pub client_extensions: Option<ClientExtensions>,

    /// TakeProfitDetails specifies the details of a Take Profit Order to be
    /// created on behalf of a client. This may happen when an Order is filled
    /// that opens a Trade requiring a Take Profit, or when a Trade’s dependent
    /// Take Profit Order is modified directly through the Trade.
    pub take_profit_on_fill: Option<TakeProfitDetails>,

    /// StopLoss specifies the details of a Stop Loss Order to be created
    /// on behalf of a client. This may happen when an Order is filled that opens
    /// a Trade requiring a Stop Loss, or when a Trade’s dependent Stop Loss
    /// Order is modified directly through the Trade.
    pub stop_loss_on_fill: Option<StopLoss>,

    /// GuaranteedStopLoss specifies the details of a Guaranteed Stop Loss
    /// Order to be created on behalf of a client. This may happen when an Order
    /// is filled that opens a Trade requiring a Guaranteed Stop Loss, or when a
    /// Trade’s dependent Guaranteed Stop Loss Order is modified directly through
    /// the Trade.
    pub guaranteed_stop_loss_on_fill: Option<StopLoss>,

    /// TrailingStopLoss specifies the details of a Trailing Stop Loss
    /// Order to be created on behalf of a client. This may happen when an Order
    /// is filled that opens a Trade requiring a Trailing Stop Loss, or when a
    /// Trade’s dependent Trailing Stop Loss Order is modified directly through
    /// the Trade.
    // TODO: TrailingStopLoss
    // pub trailing_stop_loss_on_fill: Option<TrailingStopLoss>,

    /// Client Extensions to add to the Trade created when the Order is filled
    /// (if such a Trade is created). Do not set, modify, or delete
    /// tradeClientExtensions if your account is associated with MT4.
    pub trade_client_extensions: Option<ClientExtensions>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketOrder {
    #[serde(flatten)]
    order: Order,
    /// The time-in-force requested for the Market Order. Restricted to FOK or
    /// IOC for a MarketOrder.
    pub time_in_force: MarketOrderTimeInForce,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LimitOrder {
    #[serde(flatten)]
    order: Order,

    /// The price threshold specified for the Limit Order. The Limit Order will
    /// only be filled by a market price that is equal to or better than this
    /// price.
    #[serde_as(as = "DisplayFromStr")]
    price: f32,

    /// The date/time when the Limit Order will be cancelled if its timeInForce
    /// is “GTD”.
    gtd_time: DateTime<Utc>,

    /// The time-in-force requested for the Limit Order.
    time_in_force: TimeInForce,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderType {
    /// A Market Order.
    Market,
    /// A Limit Order.
    Limit,
    /// A Stop Order.
    Stop,
    /// A Market-if-touched Order.
    MarketIfTouched,
    /// A Take Profit Order.
    TakeProfit,
    /// A Stop Loss Order.
    StopLoss,
    /// A Guaranteed Stop Loss Order.
    GuaranteedStopLoss,
    /// A Trailing Stop Loss Order.
    TrailingStopLoss,
    /// A Fixed Price Order.
    FixedPrice,
}

/// Enum representing the behavior for filling an order.
#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderPositionFill {
    /// When the Order is filled, only allow Positions to be opened or extended.
    OpenOnly,
    /// When the Order is filled, always fully reduce an existing Position before opening a new Position.
    ReduceFirst,
    /// When the Order is filled, only reduce an existing Position.
    ReduceOnly,
    /// When the Order is filled, use REDUCE_FIRST behaviour for non-client hedging Accounts,
    /// and OPEN_ONLY behaviour for client hedging Accounts.
    #[default]
    Default,
}
