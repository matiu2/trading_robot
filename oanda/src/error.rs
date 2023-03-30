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
    #[error("https error: {0}")]
    Request(#[from] reqwest::Error),
    #[error("https status code error: {0}")]
    Status(StatusCode),
    #[error("Error parsing Json: {err:?}. Input: {input}")]
    JsonParse {
        err: serde_json::Error,
        input: String,
    },
    #[error("int conversion: {0}")]
    IntConversion(#[from] ParseIntError),
    #[error("float conversion: {0}")]
    FloatConversion(#[from] ParseFloatError),
    #[error("Json conversion error")]
    JsonConversion,
    #[error("Other")]
    Other,
}
