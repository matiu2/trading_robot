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
