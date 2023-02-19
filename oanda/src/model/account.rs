use serde::Deserialize;

/// See <https://developer.oanda.com/rest-live-v20/account-ep/>
#[derive(Debug, Deserialize)]
pub struct Accounts {
    pub accounts: Vec<Account>,
}

/// See <https://developer.oanda.com/rest-live-v20/account-ep/>
#[derive(Debug, Deserialize)]
pub struct Account {
    pub id: String,
    pub tags: Vec<String>,
}
