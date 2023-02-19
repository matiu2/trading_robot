use error_stack::{Result, ResultExt};

pub use crate::model;

use crate::{client::Client, error::Error};

pub struct Accounts<'a> {
    pub(crate) client: &'a Client,
}

impl Accounts<'_> {
    /// Returns the list of accounts associated with this Oanda account.
    ///
    /// # Errors
    ///
    /// This function will return an error if the http request fails or the Json deseralization fails
    pub async fn list(&self) -> Result<Vec<model::Account>, Error> {
        let url = self.client.url("/v3/accounts");
        let request = self.client.start_get(&url);
        self.client
            .get(request)
            .await
            .map(|accounts: model::Accounts| accounts.accounts)
            .attach_printable("While listing accounts")
    }
    /// Returns the list of instruments ( things to trade like EUR/USD) available to an account
    ///
    /// See [the docs](https://developer.oanda.com/rest-live-v20/account-ep/)
    ///
    /// # Errors
    ///
    /// This function will return an error if any of these happen:
    ///  * The http request fails
    ///  * The JSON deserialization fails
    ///  * Any of the data fields fail to convert to f32s
    pub async fn list_instruments(
        &self,
        account_id: &str,
    ) -> Result<Vec<model::Instrument>, Error> {
        let path = format!("/v3/accounts/{account_id}/instruments");
        let url = self.client.url(&path);
        let request = self.client.start_get(&url);
        self.client
            .get(request)
            .await
            .map(|instruments: model::Instruments| instruments.instruments)
            .attach_printable_lazy(|| {
                format!("While getting the instrument list for account_id {account_id}")
            })
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

    #[tokio::test]
    async fn list_instruments() {
        let api_key =
            var("OANDA_TOKEN").expect("expected OANDA_TOKEN environment variable to be set");
        let client = Client::new(api_key, crate::host::Host::Dev);
        let accounts = client.accounts().list().await.unwrap();
        let account_1 = accounts
            .first()
            .expect("You should set up some Oanda accounts bro");
        dbg!(client
            .accounts()
            .list_instruments(&account_1.id)
            .await
            .unwrap());
    }
}
