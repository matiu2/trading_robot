use error_stack::{IntoReport, ResultExt};

use crate::{client::Client, error::Error};
pub use model::Account;

pub struct Accounts<'a> {
    pub(crate) client: &'a Client,
}

impl Accounts<'_> {
    pub async fn list(&self) -> error_stack::Result<Vec<Account>, Error> {
        let path = "/v3/accounts";
        let body = self
            .client
            .get(path)
            .await
            .attach_printable("While listing accounts")?;
        serde_json::from_str(&body)
            .map_err(|err| Error::JsonParse {
                err,
                input: body.to_owned(),
            })
            .map(|accounts: model::Accounts| accounts.accounts)
            .into_report()
            .attach_printable_lazy(|| format!("url path: {path}"))
    }
    pub async fn list_instruments(&self, account_id: &str) -> Vec<model::Instrument> {}
}

mod model {
    use serde::Deserialize;

    /// See https://developer.oanda.com/rest-live-v20/account-ep/
    #[derive(Debug, Deserialize)]
    pub struct Accounts {
        pub accounts: Vec<Account>,
    }

    /// See https://developer.oanda.com/rest-live-v20/account-ep/
    #[derive(Debug, Deserialize)]
    pub struct Account {
        pub id: String,
        pub tags: Vec<String>,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct InstrumentRaw {
        pub display_name: String,
        pub display_precision: i32,
        pub margin_rate: String,
        pub maximum_order_units: String,
        pub maximum_position_size: String,
        pub maximum_trailing_stop_distance: String,
        pub minimum_trade_size: String,
        pub minimum_trailing_stop_distance: String,
        pub name: String,
        pub pip_location: i32,
        pub r#type: String,
        pub trade_units_precision: i32,
    }

    #[derive(Debug)]
    pub struct Instrument {
        pub display_name: String,
        pub display_precision: i32,
        pub margin_rate: f32,
        pub maximum_order_units: u32,
        pub maximum_position_size: u32,
        pub maximum_trailing_stop_distance: f32,
        pub minimum_trade_size: u32,
        pub minimum_trailing_stop_distance: f32,
        pub name: String,
        pub pip_location: i32,
        pub r#type: String,
        pub trade_units_precision: i32,
    }

    impl TryFrom<InstrumentRaw> for Instrument {
        type Error = Box<dyn std::error::Error>;

        fn try_from(value: InstrumentRaw) -> Result<Self, Self::Error> {
            Ok(Instrument {
                display_name: value.display_name,
                display_precision: value.display_precision,
                margin_rate: value.margin_rate.parse()?,
                maximum_order_units: value.maximum_order_units.parse()?,
                maximum_position_size: value.maximum_position_size.parse()?,
                maximum_trailing_stop_distance: value.maximum_trailing_stop_distance.parse()?,
                minimum_trade_size: value.minimum_trade_size.parse()?,
                minimum_trailing_stop_distance: value.minimum_trailing_stop_distance.parse()?,
                name: value.name,
                pip_location: value.pip_location,
                r#type: value.r#type,
                trade_units_precision: value.trade_units_precision,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env::var;

    use super::*;

    #[tokio::test]
    async fn list_accounts() {
        let api_key =
            var("OANDA_TOKEN").expect("expected OANDA_TOKEN environment variable to be set");
        let client = Client::new(api_key, crate::host::Host::Dev);
        dbg!(client.accounts().list().await.unwrap());
    }
}
