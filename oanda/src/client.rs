pub mod account;
pub mod instrument;
pub mod trade;

use std::borrow::ToOwned;

use error_stack::{report, IntoReport, ResultExt};
use reqwest::RequestBuilder;
use serde::de::DeserializeOwned;

use crate::{error::Error, host::Host};

use self::account::Accounts;
use self::instrument::Instrument;
use self::trade::Trade;

#[derive(Debug, Clone)]
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
        let rest_client = reqwest::Client::builder()
            .deflate(true)
            .gzip(true)
            .brotli(true)
            .build()
            .into_report()
            .unwrap();
        Client {
            token,
            host,
            rest_client,
        }
    }
    /// Given a URL path, inserts the part before it
    pub fn url(&self, path: &str) -> String {
        self.host.rest_url(path)
    }
    /// Given a URL path, creates a Get request builder with the correct
    /// host and authentication token
    pub fn start_get(&self, url: &str) -> RequestBuilder {
        use reqwest::header::{ACCEPT, AUTHORIZATION};
        self.rest_client
            .get(url)
            .header(AUTHORIZATION, format!("Bearer {}", &self.token))
            .header(ACCEPT, "application/json")
    }
    /// Makes an authenticated get request to a path in the rest api
    pub async fn get<T: DeserializeOwned>(
        &self,
        request: RequestBuilder,
    ) -> error_stack::Result<T, Error> {
        let request = request.build().map_err(Error::from).into_report()?;
        let url = request.url().to_owned();

        let response = self
            .rest_client
            .execute(request)
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
                .attach_printable_lazy(|| format!("url: {url}"))
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

    /// Rest API for anything instrument related
    pub fn instrument(&self, instrument: impl ToString) -> Instrument {
        Instrument {
            client: self,
            instrument: instrument.to_string(),
        }
    }

    /// Rest API for anything trade related
    pub fn trade(&self, account_id: impl ToString) -> Trade {
        Trade::new(self, account_id.to_string())
    }
}
