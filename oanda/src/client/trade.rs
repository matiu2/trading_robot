//! List open trades - see <https://developer.oanda.com/rest-live-v20/trade-ep/>

mod open_trades_request;
pub use open_trades_request::OpenTradesRequest;

use crate::client::Client;

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

    // /// History of trades on the account
    // pub fn trades(&self) -> TradesRequest {
    //     TradesRequest {
    //         accept_date_time_format: DateTimeFormat::default(),
    //         trade_endpoint: self,
    //     }
    // }
}
