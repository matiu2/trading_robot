use error_stack::{Result, ResultExt};
use serde::Serialize;

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
    pub fn list_instruments<'a>(&'a self, account_id: &'a str) -> ListInstrumentsRequest {
        ListInstrumentsRequest {
            accounts: self,
            account_id,
            instruments: None,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListInstrumentsRequest<'a> {
    #[serde(skip)]
    accounts: &'a Accounts<'a>,
    /// The Id of the account for which to list instruments
    #[serde(skip)]
    account_id: &'a str,
    /// List of instruments to query specifically.
    #[serde(
        serialize_with = "serialize_csv",
        skip_serializing_if = "Option::is_none"
    )]
    instruments: Option<Vec<String>>,
}

fn serialize_csv<S>(
    value: &Option<Vec<String>>,
    serializer: S,
) -> std::result::Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&value.as_ref().map(|vec| vec.join(",")).unwrap_or_default())
}

impl<'a> ListInstrumentsRequest<'a> {
    pub fn add_instrument(mut self, instrument: impl ToString) -> Self {
        self.instruments
            .get_or_insert_with(|| Vec::new())
            .push(instrument.to_string());
        self
    }
    pub fn add_instruments<T: ToString>(
        mut self,
        instruments: impl IntoIterator<Item = T>,
    ) -> Self {
        self.instruments.get_or_insert_with(|| Vec::new()).extend(
            instruments
                .into_iter()
                .map(|instrument| instrument.to_string()),
        );
        self
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
    pub async fn send(&self) -> Result<Vec<model::Instrument>, Error> {
        let path = format!("/v3/accounts/{}/instruments", self.account_id);
        let url = self.accounts.client.url(&path);
        let request = self.accounts.client.start_get(&url).query(self);
        self.accounts
            .client
            .get(request)
            .await
            .map(|instruments: model::Instruments| instruments.instruments)
            .attach_printable_lazy(|| {
                format!(
                    "While getting the instrument list for account_id {}",
                    self.account_id
                )
            })
    }
}

#[cfg(test)]
mod tests {
    use lazy_static::lazy_static;
    use std::env::var;
    use std::sync::RwLock;

    use super::*;

    fn client() -> Client {
        let api_key =
            var("OANDA_TOKEN").expect("expected OANDA_TOKEN environment variable to be set");
        let client = Client::new(api_key, crate::host::Host::Dev);
        client
    }

    #[tokio::test]
    async fn list_accounts() {
        dbg!(client().accounts().list().await.unwrap());
    }

    lazy_static! {
        static ref ACCOUNT_ID: RwLock<Option<String>> = RwLock::new(None);
    }

    async fn account_id(client: &Client) -> String {
        // Try to get the account ID from the cache
        if let Some(account_id) = ACCOUNT_ID.read().unwrap().as_ref() {
            return account_id.to_owned();
        }

        // If the account ID is not in the cache, fetch it from the API
        let account_id = client
            .accounts()
            .list()
            .await
            .unwrap()
            .first()
            .expect("You should set up some Oanda accounts bro")
            .id
            .to_owned();

        // Store the account ID in the cache for future calls
        *ACCOUNT_ID.write().unwrap() = Some(account_id.to_owned());

        account_id
    }

    #[tokio::test]
    async fn list_instruments_simple() {
        let client = client();
        let account_id = account_id(&client).await;
        let simple = client
            .accounts()
            .list_instruments(&account_id)
            .send()
            .await
            .unwrap();
        assert!(simple.len() > 1);
    }

    #[tokio::test]
    async fn list_instruments_add_instrument() {
        let client = client();
        let account_id = account_id(&client).await;
        let instruments = client
            .accounts()
            .list_instruments(&account_id)
            .add_instrument("EUR_USD")
            .send()
            .await
            .unwrap();
        assert!(instruments.len() == 1);
        assert!(instruments[0].name == "EUR_USD");
        dbg!(instruments);
    }

    #[tokio::test]
    async fn list_instruments_add_instruments() {
        let client = client();
        let account_id = account_id(&client).await;
        let instruments = client
            .accounts()
            .list_instruments(&account_id)
            .add_instruments(["EUR_USD"])
            .send()
            .await
            .unwrap();
        assert!(instruments.len() == 1);
        assert!(instruments[0].name == "EUR_USD");
        dbg!(instruments);
    }
}
