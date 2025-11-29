use thiserror::Error;

#[derive(Error, Debug)]
pub enum WryWebViewError {
    #[error("Failed to create webview: {0}")]
    CreationError(String),
    #[error("Failed to get window handle: {0}")]
    WindowHandleError(String),
    #[error("Failed to create child window: {0}")]
    CreateChildWindowError(String),

    #[error("Unsupported platform or handle type")]
    UnsupportedPlatform,
}
