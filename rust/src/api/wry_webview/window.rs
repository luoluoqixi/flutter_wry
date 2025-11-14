use wry::raw_window_handle::{HandleError, HasWindowHandle, WindowHandle};

pub enum FlutterWindowHandle {
    #[cfg(target_os = "windows")]
    Hwnd(isize),
    #[cfg(target_os = "macos")]
    NsView(isize),
    #[cfg(target_os = "linux")]
    GtkWidget(isize),
    #[cfg(target_os = "android")]
    AndroidNativeView(jobject),
    #[cfg(target_os = "ios")]
    UiView(*mut objc::runtime::Object),
}

pub struct FlutterWindow {
    pub window: FlutterWindowHandle,
}

impl FlutterWindow {
    pub fn new(window: FlutterWindowHandle) -> Self {
        Self { window }
    }
}

impl HasWindowHandle for FlutterWindow {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        match self.window {
            #[cfg(target_os = "windows")]
            FlutterWindowHandle::Hwnd(raw_handle) => {
                use std::num::NonZeroIsize;
                use wry::raw_window_handle::{RawWindowHandle, Win32WindowHandle};

                let nz = NonZeroIsize::new(raw_handle).ok_or(HandleError::Unavailable)?;
                let handle = Win32WindowHandle::new(nz);
                Ok(unsafe { WindowHandle::borrow_raw(RawWindowHandle::Win32(handle)) })
            }
            #[allow(unreachable_patterns)]
            _ => Err(HandleError::NotSupported),
        }
    }
}
