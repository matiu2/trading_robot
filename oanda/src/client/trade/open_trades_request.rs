use self::model::{date_time::DateTimeFormat, trade::TradesResponse};
use super::Trade;
use crate::Error;
use error_stack::{Result, ResultExt};
use tracing::debug;
use typed_builder::TypedBuilder;

pub use crate::model;

#[derive(Debug, TypedBuilder)]
pub struct OpenTradesRequest<'a> {
    trade_endpoint: &'a Trade<'a>,
    #[builder(default)]
    accept_date_time_format: DateTimeFormat,
}

impl<'a> OpenTradesRequest<'a> {
    pub async fn send(&self) -> Result<TradesResponse, Error> {
        let path = format!("/v3/accounts/{}/openTrades", self.trade_endpoint.account_id);
        let url = self.trade_endpoint.client.url(&path);
        let request = self.trade_endpoint.client.start_get(&url);
        debug!("Get open trades request: {request:#?}");
        let request = self.trade_endpoint.client.start_get(&url).header(
            "Accept-Datetime-Format",
            self.accept_date_time_format.to_string(),
        );
        self.trade_endpoint
            .client
            .get(request)
            .await
            .change_context(Error::ListOpenTrades)
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
            .build()
            .send()
            .await
            .unwrap();
        dbg!(result);
    }
}
