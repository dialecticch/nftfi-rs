use thiserror::Error;

#[derive(Error, Debug)]
pub enum NFTFiError {
    #[error("reqwest error")]
    ReqwestError(#[from] reqwest::Error),
    #[error("url error")]
    URLError(#[from] url::ParseError),
    #[error("decode error")]
    DecodeError(#[from] serde_path_to_error::Error<serde_json::Error>),
}
