use thiserror::Error;

#[derive(Error, Debug)]
pub enum WryWebViewError {
    #[error("Failed to create webview: {0}")]
    CreationError(String),

    #[error("Unsupported platform or handle type")]
    UnsupportedPlatform,
}
