//! List open trades - see <https://developer.oanda.com/rest-live-v20/trade-ep/>

mod open_trades_request;
pub use open_trades_request::OpenTradesRequest;
mod trades_request;

use crate::client::Client;

use self::trades_request::TradesRequest;

#[derive(Debug)]
pub struct Trade<'a> {
    client: &'a Client,
    account_id: String,
}

impl<'a> Trade<'a> {
    pub fn new(client: &'a Client, account_id: String) -> Self {
        Self { client, account_id }
    }

    /// List all open trades on the acount
    pub fn open_trades(&self) -> open_trades_request::OpenTradesRequestBuilder<((&Trade,), ())> {
        OpenTradesRequest::builder().trade_endpoint(self)
    }

    /// History of trades on the account
    #[allow(clippy::type_complexity)]
    pub fn trades(
        &self,
    ) -> trades_request::TradesRequestBuilder<((&Trade,), (), (), (), (), (), ())> {
        TradesRequest::builder().trade_endpoint(self)
    }
}
