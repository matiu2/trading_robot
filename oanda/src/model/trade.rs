use chrono::DateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// TradeState represents the state of a trade.
///
/// * `OPEN`: The Trade is currently open.
/// * `CLOSED`: The Trade has been fully closed.
/// * `CLOSE_WHEN_TRADEABLE`: The Trade will be closed as soon as the trade's instrument becomes tradeable.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TradeState {
    /// The Trade is currently open.
    Open,
    /// The Trade has been fully closed.
    Closed,
    /// The Trade will be closed as soon as the trade's instrument becomes tradeable.
    CloseWhenTradeable,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ClientExtensions {
    /// The Client ID of the Order/Trade.
    pub id: String,
    /// A tag associated with the Order/Trade.
    pub tag: String,
    /// A comment associated with the Order/Trade.
    pub comment: String,
}

#[derive(Debug, Deserialize, PartialEq)]
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

/// Specification of which price component should be used when determining
/// if an Order should be triggered and filled. This allows Orders to
/// be triggered based on the bid, ask, mid, default (ask for buy, bid
/// for sell) or inverse (ask for sell, bid for buy) price depending on
/// the desired behaviour. Orders are always filled using their default
/// price component.
///
/// This feature is only provided through the REST API. Clients who choose
/// to specify a non-default trigger condition will not see it reflected
/// in any of OANDA’s proprietary or partner trading platforms, their
/// transaction history or their account statements. OANDA platforms
/// always assume that an Order’s trigger condition is set to the
/// default value when indicating the distance from an Order’s trigger
/// price, and will always provide the default trigger condition when
/// creating or modifying an Order.
///
/// A special restriction applies when creating a Guaranteed Stop
/// Loss Order. In this case the TriggerCondition value must either
/// be “DEFAULT”, or the “natural” trigger side “DEFAULT”
/// results in. So for a Guaranteed Stop Loss Order for a long trade
/// valid values are “DEFAULT” and “BID”, and for short trades
/// “DEFAULT” and “ASK” are valid.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderTriggerCondition {
    /// Trigger an Order the "natural" way: compare its price to the ask for long Orders and bid for short Orders.
    Default,
    /// Trigger an Order the opposite of the "natural" way: compare its price to the bid for long Orders and ask for short Orders.
    Inverse,
    /// Trigger an Order by comparing its price to the bid regardless of whether it is long or short.
    Bid,
    /// Trigger an Order by comparing its price to the ask regardless of whether it is long or short.
    Ask,
    /// Trigger an Order by comparing its price to the midpoint regardless of whether it is long or short.
    Mid,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderState {
    /// The Order is currently pending execution.
    Pending,
    /// The Order has been filled.
    Filled,
    /// The Order has been triggered.
    Triggered,
    /// The Order has been cancelled.
    Cancelled,
}

/// A TakeProfitOrder is an order that is linked to an open Trade and created with a price threshold.
/// The Order will be filled (closing the Trade) by the first price that is equal to or better than the threshold.
/// A TakeProfitOrder cannot be used to open a new Position.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TakeProfitOrder {
    /// The Order’s identifier, unique within the Order’s Account.
    pub id: String,
    /// The time when the Order was created.
    pub create_time: DateTime<Utc>,
    /// The current state of the Order.
    pub state: OrderState,
    /// The client extensions of the Order. Do not set, modify, or delete
    /// clientExtensions if your account is associated with MT4.
    pub client_extensions: ClientExtensions,
    /// The type of the Order. Always set to “TAKE_PROFIT” for Take Profit Orders.
    #[serde(rename = "type")]
    pub order_type: OrderType,
    /// The ID of the Trade to close when the price threshold is breached.
    pub trade_id: String,
    /// The client ID of the Trade to be closed when the price threshold is breached.
    pub client_trade_id: String,
    /// The price threshold specified for the TakeProfit Order. The associated
    /// Trade will be closed by a market price that is equal to or better than
    /// this threshold.
    pub price: String,
    /// The time-in-force requested for the TakeProfit Order. Restricted to
    /// “GTC”, “GFD” and “GTD” for TakeProfit Orders.
    pub time_in_force: TimeInForce,
    /// The date/time when the TakeProfit Order will be cancelled if its
    /// timeInForce is “GTD”.
    pub gtd_time: Option<DateTime<Utc>>,
    /// Specification of which price component should be used when determining if
    /// an Order should be triggered and filled. This allows Orders to be
    /// triggered based on the bid, ask, mid, default (ask for buy, bid for sell)
    /// or inverse (ask for sell, bid for buy) price depending on the desired
    /// behaviour. Orders are always filled using their default price component.
    pub trigger_condition: OrderTriggerCondition,
    /// ID of the Transaction that filled this Order (only provided when the
    /// Order’s state is FILLED)
    pub filling_transaction_id: Option<String>,
    /// Date/time when the Order was filled (only provided when the Order’s state
    /// is FILLED)
    pub filled_time: Option<DateTime<Utc>>,
    /// Trade ID of Trade opened when the Order was filled (only provided when
    /// the Order’s state is FILLED and a Trade was opened as a result of the
    /// fill)
    pub trade_opened_id: Option<String>,
    /// Trade ID of Trade reduced when the Order was filled (only provided when
    /// the Order’s state is FILLED and a Trade was reduced as a result of the
    /// fill)
    pub trade_reduced_id: Option<String>,
    /// Trade IDs of Trades closed when the Order was filled (only provided when
    /// the Order’s state is FILLED and one or more Trades were closed as a
    /// result of the fill)
    pub trade_closed_ids: Option<Vec<String>>,
    /// ID of the Transaction that cancelled the Order (only provided when the
    /// Order’s state is CANCELLED)
    pub cancelling_transaction_id: Option<String>,
    /// Date/time when the Order was cancelled (only provided when the state of
    /// the Order is CANCELLED)
    pub cancelled_time: Option<DateTime<Utc>>,
    /// The ID of the Order that was replaced by this Order (only provided if
    /// this Order was created as part of a cancel/replace).
    pub replaces_order_id: Option<String>,
    /// The ID of the Order that replaced this Order (only provided if this Order
    /// was cancelled as part of a cancel/replace).
    pub replaced_by_order_id: Option<String>,
}

/// The Account's list of open Trades and the ID of the most recent Transaction created for the Account.
#[derive(Debug, Deserialize)]
pub struct Trades {
    /// The Account's list of open Trades.
    pub trades: Vec<Trade>,
    /// The ID of the most recent Transaction created for the Account.
    pub last_transaction_id: String,
}

/// Represents a Trade with all its associated data.
#[derive(Debug, Deserialize)]
pub struct Trade {
    /// The Trade's identifier, unique within the Trade's Account.
    pub id: String,
    /// The Trade's Instrument.
    pub instrument: String,
    /// The execution price of the Trade.
    pub price: String,
    /// The date/time when the Trade was opened.
    pub open_time: DateTime<Utc>,
    /// The current state of the Trade.
    pub state: String,
    /// The initial size of the Trade. Negative values indicate a short Trade, and positive values indicate a long Trade.
    pub initial_units: f32,
    /// The margin required at the time the Trade was created. Note, this is the 'pure' margin required, it is not the 'effective' margin used that factors in the trade risk if a GSLO is attached to the trade.
    pub initial_margin_required: f32,
    /// The number of units currently open for the Trade. This value is reduced to 0.0 as the Trade is closed.
    pub current_units: f32,
    /// The total profit/loss realized on the closed portion of the Trade.
    pub realized_pl: f32,
    /// The unrealized profit/loss on the open portion of the Trade.
    pub unrealized_pl: f32,
    /// Margin currently used by the Trade.
    pub margin_used: f32,
    /// The average closing price of the Trade. Only present if the Trade has been closed or reduced at least once.
    #[serde(default)]
    pub average_close_price: Option<String>,
    /// The IDs of the Transactions that have closed portions of this Trade.
    pub closing_transaction_ids: Vec<String>,
    /// The financing paid/collected for this Trade.
    pub financing: f32,
    /// The dividend adjustment paid for this Trade.
    pub dividend_adjustment: f32,
    /// The date/time when the Trade was fully closed. Only provided for Trades whose state is CLOSED.
    #[serde(default)]
    pub close_time: Option<DateTime<Utc>>,
    /// The client extensions of the Trade.
    #[serde(default)]
    pub client_extensions: Option<ClientExtensions>,
    /// Full representation of the Trade's Take Profit Order, only provided if such an Order exists.
    #[serde(default)]
    pub take_profit_order: Option<HashMap<String, serde_json::Value>>,
    /// Full representation of the Trade's Stop Loss Order, only provided if such an Order exists.
    #[serde(default)]
    pub stop_loss_order: Option<HashMap<String, serde_json::Value>>,
    /// Full representation of the Trade's Trailing Stop Loss Order, only provided if such an Order exists.
    #[serde(default)]
    pub trailing_stop_loss_order: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TimeInForce {
    /// The Order is "Good unTil Cancelled".
    GTC,
    /// The Order is "Good unTil Date" and will be cancelled at the provided time.
    GTD,
    /// The Order is "Good For Day" and will be cancelled at 5pm New York time.
    GFD,
    /// The Order must be immediately "Filled Or Killed".
    FOK,
    /// The Order must be "Immediately partially filled Or Cancelled".
    IOC,
}

impl Default for TimeInForce {
    fn default() -> Self {
        TimeInForce::GTC
    }
}
