//! List open trades - see <http://developer.oanda.com/rest-live-v20/trade-ep/>

pub use crate::model;

use crate::client::Client;

use self::model::date_time::DateTimeFormat;

pub struct Trade<'a> {
    client: &'a Client,
    account_id: String,
}

impl<'a> Trade<'a> {
    pub fn new(client: &'a Client, account_id: String) -> Self {
        Self { client, account_id }
    }

    pub async fn open_trades(&self) -> OpenTradesRequest {
        OpenTradesRequest {
            accept_date_time_format: DateTimeFormat::default(),
            trade_endpoint: self,
        }
    }
}

pub struct OpenTradesRequest<'a> {
    trade_endpoint: &'a Trade<'a>,
    accept_date_time_format: DateTimeFormat,
}

impl<'a> OpenTradesRequest<'a> {
    pub fn accept_date_time_format(mut self, accept_date_time_format: DateTimeFormat) -> Self {
        self.accept_date_time_format = accept_date_time_format;
        todo!();
        self
    }
    // pub fn send(&self) -> Result<
}
