use reqwest::StatusCode;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("https error")]
    Request(#[from] reqwest::Error),
    #[error("https status code error: {0}")]
    Status(StatusCode),
    #[error("Error parsing Json: {err:?}. Input: {input}")]
    JsonParse {
        err: serde_json::Error,
        input: String,
    },
}
