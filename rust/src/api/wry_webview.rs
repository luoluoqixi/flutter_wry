use anyhow::Result;
use std::sync::Mutex;

use crate::wry_webview::{
    raw_window::RawWindowHandle, wry_webview_config::WryWebViewConfig, WryWebView,
};

static WEBVIEW_CONTROLLER: Mutex<Option<WryWebView>> = Mutex::new(None);

#[flutter_rust_bridge::frb(sync)]
pub fn create_webview(config: WryWebViewConfig, handle: Option<isize>) -> Result<()> {
    #[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
    let handle = handle.ok_or(anyhow::anyhow!("Window handle is required"))?;

    #[cfg(target_os = "windows")]
    let handle = RawWindowHandle::Windows(handle);
    #[cfg(target_os = "macos")]
    let handle = RawWindowHandle::MacOS(handle);

    let mut controller = WEBVIEW_CONTROLLER.lock().unwrap();
    *controller = Some(WryWebView::create(config, handle)?);

    Ok(())
}
