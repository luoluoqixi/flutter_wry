use std::sync::Mutex;

use crate::wry_webview::{
    window::RawWindowHandle, wry_webview_config::WryWebViewConfig,
    wry_webview_controller::WryWebViewController,
};
use anyhow::Result;

static WEBVIEW_CONTROLLER: Mutex<Option<WryWebViewController>> = Mutex::new(None);

#[flutter_rust_bridge::frb(sync)]
pub fn create_webview(config: WryWebViewConfig, handle: Option<isize>) -> Result<()> {
    let handle = match handle {
        Some(h) => {
            #[cfg(target_os = "windows")]
            let handle = RawWindowHandle::Hwnd(h);
            #[cfg(target_os = "macos")]
            let handle = RawWindowHandle::NsView(h);
            #[cfg(target_os = "linux")]
            let handle = RawWindowHandle::GtkWidget(h);

            handle
        }
        None => return Err(anyhow::anyhow!("Window handle is required")),
    };
    let mut controller = WEBVIEW_CONTROLLER.lock().unwrap();
    *controller = Some(WryWebViewController::create(config, handle)?);

    Ok(())
}
