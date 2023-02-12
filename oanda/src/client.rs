use std::fmt::Display;

use reqwest::RequestBuilder;

use crate::{account::Accounts, host::Host};

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
    /// Starts an authenticated request to a path in the rest api
    pub fn get(&self, path: impl Display) -> RequestBuilder {
        let url = self.host.rest_url(path);
        self.rest_client
            .get(url)
            .header("Authorization", format!("Bearer {}", &self.token))
    }
    /// Rest API for anything account related
    pub fn accounts(&self) -> Accounts {
        Accounts { client: self }
    }
}
