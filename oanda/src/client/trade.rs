//! List open trades - see <https://developer.oanda.com/rest-live-v20/trade-ep/>

use error_stack::{Result, ResultExt};
use serde::Serialize;
use tracing::debug;

pub use crate::model;

use crate::{client::Client, Error};

use self::model::{date_time::DateTimeFormat, trade::TradesResponse};

#[derive(Debug)]
pub struct Trade<'a> {
    client: &'a Client,
    account_id: String,
}

impl<'a> Trade<'a> {
    pub fn new(client: &'a Client, account_id: String) -> Self {
        Self { client, account_id }
    }

    pub fn open_trades(&self) -> OpenTradesRequest {
        OpenTradesRequest {
            accept_date_time_format: DateTimeFormat::default(),
            trade_endpoint: self,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct OpenTradesRequest<'a> {
    #[serde(skip)]
    trade_endpoint: &'a Trade<'a>,
    accept_date_time_format: DateTimeFormat,
}

impl<'a> OpenTradesRequest<'a> {
    pub fn accept_date_time_format(mut self, accept_date_time_format: DateTimeFormat) -> Self {
        self.accept_date_time_format = accept_date_time_format;
        self
    }
    pub async fn send(&self) -> Result<TradesResponse, Error> {
        let path = format!("/v3/accounts/{}/openTrades", self.trade_endpoint.account_id);
        let url = self.trade_endpoint.client.url(&path);
        let request = self.trade_endpoint.client.start_get(&url).query(self);
        debug!("Get open trades request: {request:#?}");
        self.trade_endpoint
            .client
            .get(request)
            .await
            .attach_printable_lazy(|| format!("With these params: {:?}", self))
    }
}

#[cfg(test)]
mod api_tests {
    use crate::model::date_time::DateTimeFormat;
    use crate::{Client, Error};
    use error_stack::{IntoReport, Result, ResultExt};
    use lazy_static::lazy_static;
    use std::env::var;
    use std::sync::Mutex;

    lazy_static! {
        static ref ACCOUNT_ID: Mutex<Option<String>> = Mutex::new(None);
    }

    async fn get_account_id(client: &Client) -> Result<String, Error> {
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

    #[tokio::test]
    async fn list_open_trades() {
        let api_key =
            var("OANDA_TOKEN").expect("expected OANDA_TOKEN environment variable to be set");
        let client = Client::new(api_key, crate::host::Host::Dev);
        let account_id = get_account_id(&client).await.unwrap();
        let result = client
            .trade(account_id)
            .open_trades()
            .accept_date_time_format(DateTimeFormat::Rfc3339)
            .send()
            .await
            .unwrap();
        dbg!(result);
    }
}
