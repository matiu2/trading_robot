use error_stack::{report, IntoReport, ResultExt};

use crate::{client::Client, error::Error};
pub use model::Account;

pub struct Accounts<'a> {
    pub(crate) client: &'a Client,
}

impl Accounts<'_> {
    pub async fn list(&self) -> error_stack::Result<Vec<Account>, Error> {
        let response = self
            .client
            .get("/v3/accounts")
            .send()
            .await
            .map_err(Error::from)?;

        if response.status().is_success() {
            let body = response.text().await.map_err(Error::from)?;
            serde_json::from_str::<model::Accounts>(&body)
                .map(|accounts| accounts.accounts)
                .map_err(move |err| Error::JsonParse { err, input: body })
                .into_report()
                .attach_printable("While listing accounts")
                .attach_printable(self.client.host)
        } else {
            // If we get a bad http status
            // try to get and add the body for more context
            let status = response.status();
            let body = response.text().await.map_err(Error::from);
            let mut err = report!(Error::Status(status));
            Err(match body {
                Ok(body) => err.attach_printable(format!("Body: {body}")),
                Err(body_err) => {
                    err.extend_one(report!(body_err));
                    err
                }
            })
        }
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
