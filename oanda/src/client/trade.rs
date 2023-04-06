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

#[cfg(test)]
mod test_utils {
    use crate::{Client, Error};
    use error_stack::{IntoReport, Result, ResultExt};
    use lazy_static::lazy_static;
    use std::sync::Mutex;

    lazy_static! {
        static ref ACCOUNT_ID: Mutex<Option<String>> = Mutex::new(None);
    }

    pub async fn get_account_id(client: &Client) -> Result<String, Error> {
        let mut account_id = ACCOUNT_ID.lock().unwrap();
        if let Some(account_id) = account_id.as_ref() {
            Ok(account_id.clone())
        } else {
            let accounts = client.accounts().list().await?;
            let out = accounts
                .into_iter()
                .next()
                .ok_or_else(|| Error::Other)
                .into_report()
                .attach_printable_lazy(|| "No oanda accounts found")?
                .id;
            *account_id = Some(out.clone());
            Ok(out)
        }
    }
}
