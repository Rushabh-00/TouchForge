use thiserror::Error;

#[derive(Debug, Error)]
pub enum TouchForgeError {
    #[error("unsupported profile format")]
    UnsupportedProfileFormat,

    #[error("invalid control data: {0}")]
    InvalidControlData(String),
}
