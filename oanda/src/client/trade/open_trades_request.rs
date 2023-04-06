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
            self.accept_date_time_format.header_name(),
            self.accept_date_time_format.header_value(),
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
    use crate::client::test_utils::get_account_id;
    use crate::model::date_time::DateTimeFormat;
    use crate::Client;
    use std::env::var;

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
