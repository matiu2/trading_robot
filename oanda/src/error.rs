use std::num::{ParseFloatError, ParseIntError};

use reqwest::StatusCode;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    // #[error("Conversion Error: {err:? }: {r#struct}.{field}: {value} ")]
    // Conversion {
    //     r#struct: String,
    //     field: String,
    //     value: String,
    //     err: Box<dyn std::error::Error>,
    // },
    #[error("https error")]
    Request(#[from] reqwest::Error),
    #[error("https status code error: {0}")]
    Status(StatusCode),
    #[error("Error parsing Json: {err:?}. Input: {input}")]
    JsonParse {
        err: serde_json::Error,
        input: String,
    },
    #[error("int conversion")]
    IntConversion(#[from] ParseIntError),
    #[error("float conversion")]
    FloatConversion(#[from] ParseFloatError),
    #[error("Other")]
    Other,
}
