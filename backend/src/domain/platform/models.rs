use strum_macros::{EnumString, IntoStaticStr, VariantNames};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, EnumString, IntoStaticStr, VariantNames)]
pub enum PlatformName {
    ChessCom,
}

#[derive(Debug, thiserror::Error)]
pub enum PlatformError {
    #[error("Adapter not implemented for platform: {0}")]
    PlatformNotFound(String),
    #[error("Request to platform failed: {0}")]
    NetworkError(#[from] anyhow::Error),
    #[error("Failed to parse response from platform: {0}")]
    ParseError(String),
    #[error("API error from platform: {0}")]
    ApiError(String),
}
