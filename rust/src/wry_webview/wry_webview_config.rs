#[derive(Debug, Default)]
pub struct WryWebViewConfig {
    /// Load the provided URL when the builder calling [`WebViewBuilder::build`] to create the [`WebView`].
    /// The provided URL must be valid.
    ///
    /// ## Note
    ///
    /// Data URLs are not supported, use [`html`](Self::with_html) option instead.
    pub initial_url: Option<String>,
    /// Load the provided HTML string when the builder calling [`WebViewBuilder::build`] to create the [`WebView`].
    /// This will be ignored if `url` is provided.
    ///
    /// # Warning
    ///
    /// The Page loaded from html string will have `null` origin.
    ///
    /// ## PLatform-specific:
    ///
    /// - **Windows:** the string can not be larger than 2 MB (2 * 1024 * 1024 bytes) in total size
    pub initial_html: Option<String>,
    /// Set whether the webview should be focused when created.
    ///
    /// ## Platform-specific:
    ///
    /// - **macOS / Android / iOS:** Unsupported.
    pub initial_focused: Option<bool>,
    /// Enable or disable web inspector which is usually called devtools.
    ///
    /// Note this only enables devtools to the webview. To open it, you can call
    /// [`WebView::open_devtools`], or right click the page and open it from the context menu.
    ///
    /// ## Platform-specific
    ///
    /// - macOS: This will call private functions on **macOS**. It is enabled in **debug** builds,
    ///   but requires `devtools` feature flag to actually enable it in **release** builds.
    /// - Android: Open `chrome://inspect/#devices` in Chrome to get the devtools window. Wry's `WebView` devtools API isn't supported on Android.
    /// - iOS: Open Safari > Develop > [Your Device Name] > [Your WebView] to get the devtools window.
    pub initial_devtools: Option<bool>,
}

impl WryWebViewConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_initial_url(mut self, url: impl AsRef<str>) -> Self {
        self.initial_url = Some(url.as_ref().to_string());
        self
    }

    pub fn with_initial_html(mut self, html: impl AsRef<str>) -> Self {
        self.initial_html = Some(html.as_ref().to_string());
        self
    }

    pub fn with_initial_focused(mut self, focused: bool) -> Self {
        self.initial_focused = Some(focused);
        self
    }

    pub fn with_initial_devtools(mut self, devtools: bool) -> Self {
        self.initial_devtools = Some(devtools);
        self
    }
}
