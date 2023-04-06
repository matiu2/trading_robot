use error_stack::{Result, ResultExt};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use typed_builder::TypedBuilder;

use crate::{
    model::{date_time::DateTimeFormat, trade::TradesResponse},
    Error,
};

use super::Trade;

#[derive(TypedBuilder, Serialize)]
#[serde_as]
/// A struct representing a request for trades.
pub struct TradesRequest<'a> {
    #[serde(skip)]
    trade_endpoint: &'a Trade<'a>,

    #[builder(default)]
    #[serde(skip)] // Skip because it's transmitted in the headers not in the url params
    /// Format of DateTime fields in the request and response.
    accept_date_time_format: DateTimeFormat,

    #[builder(setter(strip_option), default)]
    #[serde_as(as = "Option<Vec<String>>")]
    /// List of Trade IDs to retrieve.
    pub ids: Option<Vec<u32>>,

    #[builder(setter(strip_option), default)]
    /// The state to filter the requested Trades by. [default=OPEN]
    pub state: Option<TradeStateFilter>,

    #[builder(setter(strip_option), default)]
    /// The instrument to filter the requested Trades by.
    pub instrument: Option<String>,

    #[builder(setter(strip_option), default)]
    /// The maximum number of Trades to return. [default=50, maximum=500]
    pub count: Option<usize>,

    #[builder(setter(strip_option), default)]
    #[serde_as(as = "Option<String>")]
    /// The maximum Trade ID to return. If not provided the most recent Trades in the Account are returned.
    pub before_id: Option<u32>,
}

impl<'a> TradesRequest<'a> {
    pub async fn send(&self) -> Result<TradesResponse, Error> {
        let path = format!("/v3/accounts/{}/trades", self.trade_endpoint.account_id);
        let url = self.trade_endpoint.client.url(&path);
        let request = self
            .trade_endpoint
            .client
            .start_get(&url)
            .header(
                self.accept_date_time_format.header_name(),
                self.accept_date_time_format.header_value(),
            )
            .query(self);
        self.trade_endpoint
            .client
            .get(request)
            .await
            .change_context(Error::ListTrades)
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TradeStateFilter {
    Open,
    Closed,
    CloseWhenTradeable,
    All,
}

#[cfg(test)]
mod api_tests {
    use super::super::test_utils::get_account_id;
    use crate::model::date_time::DateTimeFormat;
    use crate::Client;
    use std::env::var;

    #[tokio::test]
    async fn list_trades() {
        let api_key =
            var("OANDA_TOKEN").expect("expected OANDA_TOKEN environment variable to be set");
        let client = Client::new(api_key, crate::host::Host::Dev);
        let account_id = get_account_id(&client).await.unwrap();
        let result = client
            .trade(account_id)
            .trades()
            .accept_date_time_format(DateTimeFormat::Rfc3339)
            .count(1)
            .build()
            .send()
            .await
            .unwrap();
        // TODO: We have to actually make some trades and test that we can get them
        dbg!(result);
    }
}
