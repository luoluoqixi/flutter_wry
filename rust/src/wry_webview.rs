pub mod raw_window;
pub mod wry_webview_config;
pub mod wry_webview_error;

use raw_window::{RawWindow, RawWindowHandle};
use wry_webview_config::WryWebViewConfig;
use wry_webview_error::WryWebViewError;

unsafe impl Send for WryWebView {}
unsafe impl Sync for WryWebView {}

pub struct WryWebView {
    webview: wry::WebView,
    config: WryWebViewConfig,
    parent_window: RawWindow,
    webview_window: RawWindow,
}

impl WryWebView {
    pub fn create(
        config: WryWebViewConfig,
        handle: RawWindowHandle,
    ) -> Result<Self, WryWebViewError> {
        let parent_window = RawWindow::new(handle);
        let child_window = parent_window.create_child_window()?;

        let webview_builder = wry::WebViewBuilder::new_with_attributes(wry::WebViewAttributes {
            url: config.url.clone(),
            html: config.html.clone(),
            devtools: config.devtools.unwrap_or(false),
            bounds: Some(wry::Rect {
                position: wry::dpi::Position::Logical(wry::dpi::LogicalPosition { x: 0.0, y: 0.0 }),
                size: wry::dpi::Size::Logical(wry::dpi::LogicalSize {
                    width: 800.0,
                    height: 600.0,
                }),
            }),

            #[cfg(not(all(target_os = "android", target_os = "ios", target_os = "macos")))]
            focused: config.focused.unwrap_or(true),

            ..Default::default()
        });

        #[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
        let webview = webview_builder
            .build(&child_window)
            .map_err(|err| WryWebViewError::CreationError(err.to_string()))?;

        Ok(Self {
            webview,
            config,
            parent_window,
            webview_window: child_window,
        })
    }

    pub fn inner(&self) -> &wry::WebView {
        &self.webview
    }
    pub fn inner_mut(&mut self) -> &mut wry::WebView {
        &mut self.webview
    }
    pub fn config(&self) -> &WryWebViewConfig {
        &self.config
    }
    pub fn parent_window_handle(&self) -> &RawWindowHandle {
        &self.parent_window.window
    }
    pub fn window_handle(&self) -> &RawWindowHandle {
        &self.webview_window.window
    }
}
