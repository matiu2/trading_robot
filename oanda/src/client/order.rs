//! Anything order related. See <https://developer.oanda.com/rest-live-v20/order-ep/>
use crate::client::Client;

use self::order_request::MarketOrderRequest;
mod order_request;

// Sorry :(
type MarketOrderRequestBuilder<'a> = order_request::MarketOrderRequestBuilder<
    'a,
    ((&'a Order<'a>,), (), (), (), (), (), (), (), (), (), (), ()),
>;

#[derive(Debug)]
pub struct Order<'a> {
    pub client: &'a Client,
    pub account_id: String,
}

impl<'a> Order<'a> {
    pub fn new(client: &'a Client, account_id: String) -> Self {
        Self { client, account_id }
    }

    /// Buy or Sell an instrument at market price
    pub fn market_order(&self) -> MarketOrderRequestBuilder {
        MarketOrderRequest::builder().order_endpoint(self)
    }
}

// pub struct OrderRequest<'a> {  }

#[cfg(test)]
mod api_tests {
    // use crate::{client::test_utils::get_account_id, Client};
    // use std::env::var;

    // TODO: write this
    // #[tokio::test]
    // async fn make_market_order() {
    //     let api_key =
    //         var("OANDA_TOKEN").expect("expected OANDA_TOKEN environment variable to be set");
    //     let client = Client::new(api_key, crate::host::Host::Dev);
    //     let account_id = get_account_id(&client).await.unwrap();
    //     let result = client
    //         .order(account_id)
    //         .market_order()
    //         .instrument("EUR_USD")
    //         .units(1.0)
    //         .build()
    //         .send()
    //         .await
    //         .unwrap();
    //     // TODO: We have to actually make some trades and test that we can get them
    //     dbg!(result);
    // }
}
