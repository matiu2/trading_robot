//! Anything order related. See <https://developer.oanda.com/rest-live-v20/order-ep/>
use crate::client::Client;

// use self::model::date_time::DateTimeFormat;

pub struct Order<'a> {
    pub client: &'a Client,
    pub account_id: String,
}

impl<'a> Order<'a> {
    pub fn new(client: &'a Client, account_id: String) -> Self {
        Self { client, account_id }
    }

    // pub async fn order(&self) -> OrderRequest {
    //     todo!()
    // }
}

// pub struct OrderRequest<'a> {  }
