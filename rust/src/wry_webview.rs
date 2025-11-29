pub mod raw_window;
pub mod wry_webview_config;
pub mod wry_webview_error;

use raw_window::{RawWindow, RawWindowHandle};
use wry_webview_config::{WryWebViewConfig, WryWebViewSize};
use wry_webview_error::WryWebViewError;

unsafe impl Send for WryWebView {}
unsafe impl Sync for WryWebView {}

pub struct WryWebView {
    webview: wry::WebView,
}

impl WryWebView {
    pub fn create(
        config: WryWebViewConfig,
        handle: RawWindowHandle,
    ) -> Result<Self, WryWebViewError> {
        let window = RawWindow::new(handle);

        let bounds = if config.initial_position.is_some() || config.initial_size.is_some() {
            let position = config.initial_position.unwrap_or_default();
            let size = config.initial_size.unwrap_or(WryWebViewSize {
                width: 200.0,
                height: 200.0,
            });
            Some(wry::Rect {
                position: wry::dpi::LogicalPosition::new(position.x, position.y).into(),
                size: wry::dpi::LogicalSize::new(size.width, size.height).into(),
            })
        } else {
            None
        };

        let webview_builder = wry::WebViewBuilder::new_with_attributes(wry::WebViewAttributes {
            bounds,
            url: config.initial_url,
            html: config.initial_html,
            devtools: config.initial_devtools.unwrap_or(false),

            #[cfg(not(all(target_os = "android", target_os = "ios", target_os = "macos")))]
            focused: config.initial_focused.unwrap_or(true),

            ..Default::default()
        });

        #[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
        let webview = webview_builder
            .build_as_child(&window)
            .map_err(|err| WryWebViewError::CreationError(err.to_string()))?;

        Ok(Self { webview })
    }

    pub fn inner(&self) -> &wry::WebView {
        &self.webview
    }
    pub fn inner_mut(&mut self) -> &mut wry::WebView {
        &mut self.webview
    }
}
