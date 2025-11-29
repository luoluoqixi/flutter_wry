pub mod raw_window;
pub mod wry_webview_config;
pub mod wry_webview_error;

use raw_window::{RawWindow, RawWindowHandle};
use wry_webview_config::{WryWebViewConfig, WryWebViewPosition, WryWebViewSize};
use wry_webview_error::WryWebViewError;

unsafe impl Send for WryWebView {}
unsafe impl Sync for WryWebView {}

pub struct WryWebView {
    webview: wry::WebView,
    config: WryWebViewConfig,
    window: RawWindow,
}

impl WryWebView {
    pub fn create(
        config: WryWebViewConfig,
        handle: RawWindowHandle,
    ) -> Result<Self, WryWebViewError> {
        let parent_window = RawWindow::new(handle);

        let rect = if config.position.is_some() || config.size.is_some() {
            let default_position = WryWebViewPosition::new(0.0, 0.0);
            let default_size = WryWebViewSize::new(200.0, 200.0);
            let position = config.position.as_ref().unwrap_or(&default_position);
            let size = config.size.as_ref().unwrap_or(&default_size);
            Some(wry::Rect {
                position: wry::dpi::Position::Logical(wry::dpi::LogicalPosition {
                    x: position.x,
                    y: position.y,
                }),
                size: wry::dpi::Size::Logical(wry::dpi::LogicalSize {
                    width: size.width,
                    height: size.height,
                }),
            })
        } else {
            None
        };

        let webview_builder = wry::WebViewBuilder::new_with_attributes(wry::WebViewAttributes {
            url: config.url.clone(),
            html: config.html.clone(),
            devtools: config.devtools.unwrap_or(false),
            bounds: rect,

            #[cfg(not(all(target_os = "android", target_os = "ios", target_os = "macos")))]
            focused: config.focused.unwrap_or(true),

            ..Default::default()
        });

        #[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
        let webview = webview_builder
            .build_as_child(&parent_window)
            .map_err(|err| WryWebViewError::CreationError(err.to_string()))?;

        Ok(Self {
            webview,
            config,
            window: parent_window,
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
    pub fn window_handle(&self) -> &RawWindowHandle {
        &self.window.window
    }
}
