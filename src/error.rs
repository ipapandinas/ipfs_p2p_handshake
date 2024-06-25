use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConnectionError {
    #[error("EOF")]
    EOF,
    #[error("Invalid response: {0}")]
    InvalidResponse(String),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

#[derive(Error, Debug)]
pub enum NoiseError {
    #[error("Handshake failed: {0}")]
    HandshakeFailed(String),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    SnowError(#[from] snow::Error),
    #[error(transparent)]
    ProstError(#[from] prost::EncodeError),
}

#[derive(Error, Debug)]
pub enum MainError {
    #[error(transparent)]
    ConnectionError(#[from] ConnectionError),
    #[error(transparent)]
    NoiseError(#[from] NoiseError),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}
