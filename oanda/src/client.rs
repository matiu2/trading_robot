pub mod account;

use std::borrow::ToOwned;

use error_stack::{report, IntoReport, ResultExt};
use serde::de::DeserializeOwned;

use crate::{error::Error, host::Host};

use self::account::Accounts;

pub struct Client {
    token: String,
    pub(crate) host: Host,
    rest_client: reqwest::Client,
}

impl Client {
    /// Creates a new [`Client`].
    ///
    /// `token` is your API Token
    /// `host` is the host to use
    pub fn new(token: String, host: Host) -> Client {
        let rest_client = reqwest::Client::new();
        Client {
            token,
            host,
            rest_client,
        }
    }
    /// Makes an authenticated get request to a path in the rest api
    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> error_stack::Result<T, Error> {
        let url = self.host.rest_url(path);

        let response = self
            .rest_client
            .get(&url)
            .header("Authorization", format!("Bearer {}", &self.token))
            .send()
            .await
            .map_err(Error::from)
            .into_report()
            .attach_printable_lazy(|| format!("URL: {url}"))?;

        let status = response.status();
        if status.is_success() {
            let body: String = response
                .text()
                .await
                .map_err(Error::from)
                .into_report()
                .attach_printable_lazy(|| format!("URL: {url}"))
                .attach_printable_lazy(|| format!("HTTP status code: {status}"))?;
            serde_json::from_str(&body)
                .map_err(|err| Error::JsonParse {
                    err,
                    input: body.to_owned(),
                })
                .into_report()
                .attach_printable_lazy(|| format!("url path: {path}"))
        } else {
            // If we get a bad http status
            // try to get and add the body for more context
            let body = response.text().await.map_err(Error::from);
            let mut err = report!(Error::Status(status)).attach_printable(format!("URL: {url}"));
            Err(match body {
                Ok(body) => err.attach_printable(format!("Body: {body}")),
                Err(body_err) => {
                    err.extend_one(report!(body_err));
                    err
                }
            })
        }
    }

    /// Rest API for anything account related
    pub fn accounts(&self) -> Accounts {
        Accounts { client: self }
    }
}
