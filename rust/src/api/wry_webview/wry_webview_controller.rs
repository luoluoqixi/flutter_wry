use super::{
    window::{FlutterWindow, FlutterWindowHandle},
    wry_webview_config::WryWebViewConfig,
    wry_webview_error::WryWebViewError,
};

pub struct WryWebViewController {
    webview: wry::WebView,
}

impl WryWebViewController {
    pub fn new(
        config: WryWebViewConfig,
        handle: FlutterWindowHandle,
    ) -> Result<Self, WryWebViewError> {
        let window = FlutterWindow::new(handle);

        let mut webview_builder = wry::WebViewBuilder::new();
        if let Some(url) = config.initial_url {
            webview_builder = webview_builder.with_url(&url);
        }
        if let Some(html) = config.initial_html {
            webview_builder = webview_builder.with_html(&html);
        }
        if let Some(devtools) = config.initial_devtools {
            webview_builder = webview_builder.with_devtools(devtools);
        }
        #[cfg(not(all(target_os = "android", target_os = "ios", target_os = "macos")))]
        if let Some(focused) = config.initial_focused {
            webview_builder = webview_builder.with_focused(focused);
        }

        #[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
        let webview = webview_builder
            .build(&window)
            .map_err(|err| WryWebViewError::CreationError(err.to_string()))?;

        Ok(Self { webview })
    }
}
