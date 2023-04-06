mod stop_loss;
use crate::model::trade::ClientExtensions;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
pub mod conversion_factors;
pub use conversion_factors::HomeConversionFactors;

use super::{order::OrderFillReason, pricing::ClientPrice, trade::TimeInForce};
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    /// The Transaction’s Identifier.
    id: String,

    /// The date/time when the Transaction was created.
    time: DateTime<Utc>,

    /// The ID of the user that initiated the creation of the Transaction.
    #[serde(rename = "userID")]
    user_id: i64,

    /// The ID of the Account the Transaction was created for.
    #[serde(rename = "accountID")]
    account_id: String,

    /// The ID of the “batch” that the Transaction belongs to. Transactions in
    /// the same batch are applied to the Account simultaneously.
    batch_id: String,

    /// The Request ID of the request which generated the transaction.
    request_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct OrderFillTransaction {
    /// The Transaction’s Identifier.
    pub id: String,

    /// The date/time when the Transaction was created.
    pub time: DateTime<Utc>,

    /// The ID of the user that initiated the creation of the Transaction.
    #[serde(rename = "userID")]
    pub user_id: i64,

    /// The ID of the Account the Transaction was created for.
    #[serde(rename = "accountID")]
    pub account_id: String,

    /// The ID of the “batch” that the Transaction belongs to. Transactions in
    /// the same batch are applied to the Account simultaneously.
    #[serde(rename = "batchID")]
    pub batch_id: String,

    /// The Request ID of the request which generated the transaction.
    #[serde(rename = "requestID")]
    pub request_id: String,

    /// The Type of the Transaction. Always set to “ORDER_FILL” for an
    /// OrderFillTransaction.
    #[serde(rename = "type")]
    pub transaction_type: TransactionType,

    /// The ID of the Order filled.
    #[serde(rename = "orderID")]
    pub order_id: String,

    /// The client Order ID of the Order filled (only provided if the client has
    /// assigned one).
    #[serde(rename = "clientOrderID")]
    pub client_order_id: String,

    /// The name of the filled Order’s instrument.
    pub instrument: String,

    /// The number of units filled by the OrderFill.
    pub units: f32,

    /// The HomeConversionFactors in effect at the time of the OrderFill.
    pub home_conversion_factors: HomeConversionFactors,

    /// The price that all of the units of the OrderFill should have been filled
    /// at, in the absence of guaranteed price execution. This factors in the
    /// Account’s current ClientPrice, used liquidity and the units of the
    /// OrderFill only. If no Trades were closed with their price clamped for
    /// guaranteed stop loss enforcement, then this value will match the price
    /// fields of each Trade opened, closed, and reduced, and they will all be
    /// the exact same.
    pub full_vwap: f32,

    /// The price in effect for the account at the time of the Order fill.
    pub full_price: ClientPrice,

    /// The reason that an Order was filled
    pub reason: OrderFillReason,

    /// The profit or loss incurred when the Order was filled.
    pub pl: f32,

    /// The profit or loss incurred when the Order was filled, in the
    /// Instrument’s quote currency.
    pub quote_pl: f32,

    /// The financing paid or collected when the Order was filled.
    pub financing: f32,

    /// The financing paid or collected when the Order was filled, in the
    /// Instrument’s base currency.
    pub base_financing: f32,

    /// The financing paid or collected when the Order was filled, in the
    /// Instrument’s quote currency.
    pub quote_financing: f32,

    /// The commission charged in the Account’s home currency as a result of
    /// filling the Order. The commission is always represented as a positive
    /// quantity of the Account’s home currency, however it reduces the balance
    /// in the Account.
    pub commission: f32,

    /// The total guaranteed execution fees charged for all Trades opened, closed
    /// or reduced with guaranteed Stop Loss Orders.
    pub guaranteed_execution_fee: f32,

    /// The total guaranteed execution fees charged for all Trades opened, closed
    /// or reduced with guaranteed Stop Loss Orders, expressed in the
    /// Instrument’s quote currency.
    #[serde(rename = "quoteGuaranteedExecutionFee")]
    #[serde_as(as = "DisplayFromStr")]
    pub quote_guaranteed_execution_fee: f32,

    /// The Account’s balance after the Order was filled.
    #[serde(rename = "accountBalance")]
    #[serde_as(as = "DisplayFromStr")]
    pub account_balance: f32,

    /// The Trade that was opened when the Order was filled (only provided if
    /// filling the Order resulted in a new Trade).
    #[serde(rename = "tradeOpened")]
    pub trade_opened: Option<TradeOpen>,

    /// The Trades that were closed when the Order was filled (only provided if
    /// filling the Order resulted in a closing open Trades).
    #[serde(rename = "tradesClosed")]
    pub trades_closed: Option<Vec<TradeReduce>>,

    /// The Trade that was reduced when the Order was filled (only provided if
    /// filling the Order resulted in reducing an open Trade).
    #[serde(rename = "tradeReduced")]
    pub trade_reduced: Option<TradeReduce>,

    /// The half spread cost for the OrderFill, which is the sum of the
    /// halfSpreadCost values in the tradeOpened, tradesClosed and tradeReduced
    /// fields. This can be a positive or negative value and is represented in
    /// the home currency of the Account.
    #[serde(rename = "halfSpreadCost")]
    pub half_spread_cost: AccountUnits,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionType {
    /// Account Create Transaction
    Create,

    /// Account Close Transaction
    Close,

    /// Account Reopen Transaction
    Reopen,

    /// Client Configuration Transaction
    ClientConfigure,

    /// Client Configuration Reject Transaction
    ClientConfigureReject,

    /// Transfer Funds Transaction
    TransferFunds,

    /// Transfer Funds Reject Transaction
    TransferFundsReject,

    /// Market Order Transaction
    MarketOrder,

    /// Market Order Reject Transaction
    MarketOrderReject,

    /// Fixed Price Order Transaction
    FixedPriceOrder,

    /// Limit Order Transaction
    LimitOrder,

    /// Limit Order Reject Transaction
    LimitOrderReject,

    /// Stop Order Transaction
    StopOrder,

    /// Stop Order Reject Transaction
    StopOrderReject,

    /// Market if Touched Order Transaction
    MarketIfTouchedOrder,

    /// Market if Touched Order Reject Transaction
    MarketIfTouchedOrderReject,

    /// Take Profit Order Transaction
    TakeProfitOrder,

    /// Take Profit Order Reject Transaction
    TakeProfitOrderReject,

    /// Stop Loss Order Transaction
    StopLossOrder,

    /// Stop Loss Order Reject Transaction
    StopLossOrderReject,

    /// Guaranteed Stop Loss Order Transaction
    GuaranteedStopLossOrder,

    /// Guaranteed Stop Loss Order Reject Transaction
    GuaranteedStopLossOrderReject,

    /// Trailing Stop Loss Order Transaction
    TrailingStopLossOrder,

    /// Trailing Stop Loss Order Reject Transaction
    TrailingStopLossOrderReject,

    /// Order Fill Transaction
    OrderFill,

    /// Order Cancel Transaction
    OrderCancel,

    /// Order Cancel Reject Transaction
    OrderCancelReject,

    /// Order Client Extensions Modify Transaction
    OrderClientExtensionsModify,

    /// Order Client Extensions Modify Reject Transaction
    OrderClientExtensionsModifyReject,

    /// Trade Client Extensions Modify Transaction
    TradeClientExtensionsModify,

    /// Trade Client Extensions Modify Reject Transaction
    TradeClientExtensionsModifyReject,

    /// Margin Call Enter Transaction
    MarginCallEnter,

    /// Margin Call Extend Transaction
    MarginCallExtend,

    /// Margin Call Exit Transaction
    MarginCallExit,

    /// Delayed Trade Closure Transaction
    DelayedTradeClosure,

    /// Daily Financing Transaction
    DailyFinancing,

    /// Dividend Adjustment Transaction
    DividendAdjustment,

    /// Reset Resettable PL Transaction
    ResetResettablePl,
}
