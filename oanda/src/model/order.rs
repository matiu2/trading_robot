use crate::model::trade::{ClientExtensions, TimeInForce};
use crate::model::transaction::{OrderFillTransaction, StopLoss};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

use super::transaction::{Transaction, TransactionType};
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
#[derive(Default, Serialize, Deserialize, Debug, PartialEq, Eq)]
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

pub enum OrderResponse {
    /// Order was created (201)
    Created(OrderGoodResponse),
    /// Order specification was invalid
    BadSpec(OrderFailedResponse),
    /// Order or Account not found
    NotFound(OrderFailedResponse),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderGoodResponse {
    /// The Transaction that created the Order specified by the request.
    order_create_transaction: Transaction,

    /// The Transaction that filled the newly created Order. Only provided when
    /// the Order was immediately filled.
    #[serde(rename = "orderFillTransaction")]
    order_fill_transaction: OrderFillTransaction,

    /// The Transaction that cancelled the newly created Order. Only provided
    /// when the Order was immediately cancelled.
    #[serde(rename = "orderCancelTransaction")]
    order_cancel_transaction: OrderCancelTransaction,

    /// The Transaction that reissues the Order. Only provided when the Order is
    /// configured to be reissued for its remaining units after a partial fill
    /// and the reissue was successful.
    #[serde(rename = "orderReissueTransaction")]
    order_reissue_transaction: Transaction,

    /// The Transaction that rejects the reissue of the Order. Only provided when
    /// the Order is configured to be reissued for its remaining units after a
    /// partial fill and the reissue was rejected.
    #[serde(rename = "orderReissueRejectTransaction")]
    order_reissue_reject_transaction: Transaction,

    /// The IDs of all Transactions that were created while satisfying the
    /// request.
    #[serde(rename = "relatedTransactionIDs")]
    related_transaction_ids: Vec<String>,

    /// The ID of the most recent Transaction created for the Account
    #[serde(rename = "lastTransactionID")]
    last_transaction_id: String,
}

#[derive(Debug, Deserialize)]
pub struct OrderFailedResponse {
    /// The Transaction that rejected the creation of the Order as requested
    #[serde(rename = "orderRejectTransaction")]
    order_reject_transaction: Transaction,

    /// The IDs of all Transactions that were created while satisfying the
    /// request.
    #[serde(rename = "relatedTransactionIDs")]
    related_transaction_ids: Vec<String>,

    /// The ID of the most recent Transaction created for the Account
    #[serde(rename = "lastTransactionID")]
    last_transaction_id: String,

    /// The code of the error that has occurred. This field may not be returned
    /// for some errors.
    error_code: Option<String>,

    /// The human-readable description of the error that has occurred.
    error_message: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderFillReason {
    /// The Order filled was a Limit Order
    LimitOrder,

    /// The Order filled was a Stop Order
    StopOrder,

    /// The Order filled was a Market-if-touched Order
    MarketIfTouchedOrder,

    /// The Order filled was a Take Profit Order
    TakeProfitOrder,

    /// The Order filled was a Stop Loss Order
    StopLossOrder,

    /// The Order filled was a Guaranteed Stop Loss Order
    GuaranteedStopLossOrder,

    /// The Order filled was a Trailing Stop Loss Order
    TrailingStopLossOrder,

    /// The Order filled was a Market Order
    MarketOrder,

    /// The Order filled was a Market Order used to explicitly close a Trade
    MarketOrderTradeClose,

    /// The Order filled was a Market Order used to explicitly close a Position
    MarketOrderPositionCloseout,

    /// The Order filled was a Market Order used for a Margin Closeout
    MarketOrderMarginCloseout,

    /// The Order filled was a Market Order used for a delayed Trade close
    MarketOrderDelayedTradeClose,

    /// The Order filled was a Fixed Price Order
    FixedPriceOrder,

    /// The Order filled was a Fixed Price Order created as part of a platform account migration
    FixedPriceOrderPlatformAccountMigration,

    /// The Order filled was a Fixed Price Order created to close a Trade as part of division account migration
    FixedPriceOrderDivisionAccountMigration,

    /// The Order filled was a Fixed Price Order created to close a Trade administratively
    FixedPriceOrderAdministrativeAction,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct OrderCancelTransaction {
    /// The Transaction’s Identifier.
    pub id: String,

    /// The date/time when the Transaction was created.
    pub time: DateTime<Utc>,

    /// The ID of the user that initiated the creation of the Transaction.
    pub user_id: i64,

    /// The ID of the Account the Transaction was created for.
    pub account_id: String,

    /// The ID of the “batch” that the Transaction belongs to.
    pub batch_id: String,

    /// The Request ID of the request which generated the transaction.
    pub request_id: String,

    /// The Type of the Transaction. Always set to “ORDER_CANCEL” for an OrderCancelTransaction.
    #[serde(rename = "type")]
    pub transaction_type: TransactionType,

    /// The ID of the Order cancelled.
    pub order_id: String,

    /// The client ID of the Order cancelled (only provided if the Order has a client Order ID).
    pub client_order_id: Option<String>,

    /// The reason that the Order was cancelled.
    pub reason: OrderCancelReason,

    /// The ID of the Order that replaced this Order (only provided if this Order was cancelled for replacement).
    pub replaced_by_order_id: Option<String>,
}


use serde::{Deserialize, Serialize};

/// Enum representing reasons for order cancellation.
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderCancelReason {
    /// The Order was cancelled because at the time of filling, an unexpected internal server error occurred.
    InternalServerError,
    /// The Order was cancelled because at the time of filling the account was locked.
    AccountLocked,
    /// The order was to be filled, however the account is configured to not allow new positions to be created.
    AccountNewPositionsLocked,
    /// Filling the Order wasn’t possible because it required the creation of a dependent Order and the Account is locked for Order creation.
    AccountOrderCreationLocked,
    /// Filling the Order was not possible because the Account is locked for filling Orders.
    AccountOrderFillLocked,
    /// The Order was cancelled explicitly at the request of the client.
    ClientRequest,
    /// The Order cancelled because it is being migrated to another account.
    Migration,
    /// Filling the Order wasn’t possible because the Order’s instrument was halted.
    MarketHalted,
    /// The Order is linked to an open Trade that was closed.
    LinkedTradeClosed,
    /// The time in force specified for this order has passed.
    TimeInForceExpired,
    /// Filling the Order wasn’t possible because the Account had insufficient margin.
    InsufficientMargin,
    /// Filling the Order would have resulted in a a FIFO violation.
    FifoViolation,
    /// Filling the Order would have violated the Order’s price bound.
    BoundsViolation,
    /// The Order was cancelled for replacement at the request of the client.
    ClientRequestReplaced,
    /// The Order was cancelled for replacement with an adjusted fillPrice to accommodate for the price movement caused by a dividendAdjustment.
    DividendAdjustmentReplaced,
    /// Filling the Order wasn’t possible because enough liquidity available.
    InsufficientLiquidity,
    /// Filling the Order would have resulted in the creation of a Take Profit Order with a GTD time in the past.
    TakeProfitOnFillGtdTimestampInPast,
    /// Filling the Order would result in the creation of a Take Profit Order that would have been filled immediately, closing the new Trade at a loss.
    TakeProfitOnFillLoss,
    /// Filling the Order would result in the creation of a Take Profit Loss Order that would close the new Trade at a loss when filled.
    LosingTakeProfit,
    /// Filling the Order would have resulted in the creation of a Stop Loss Order with a GTD time in the past.
    StopLossOnFillGtdTimestampInPast,
    /// Filling the Order would result in the creation of a Stop Loss Order that would have been filled immediately, closing the new Trade at a loss.
    StopLossOnFillLoss,
    /// Filling the Order would result in the creation of a Stop Loss Order whose price would be zero or negative due to the specified distance.
    StopLossOnFillPriceDistanceMaximumExceeded,
    /// Filling the Order would not result in the creation of Stop Loss Order, however the Account’s configuration requires that all Trades have a Stop Loss Order attached to them.
    StopLossOnFillRequired,
    /// Filling the Order would not result in the creation of a guaranteed Stop Loss Order, however the Account’s configuration requires that all Trades have a guaranteed Stop Loss Order attached to them.
    StopLossOnFillGuaranteedRequired,
    todo!("more fields")
}
