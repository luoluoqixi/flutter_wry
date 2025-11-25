use wry::raw_window_handle::{HandleError, HasWindowHandle, WindowHandle};

pub enum RawWindowHandle {
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

pub(crate) struct RawWindow {
    pub window: RawWindowHandle,
}

impl RawWindow {
    pub fn new(window: RawWindowHandle) -> Self {
        Self { window }
    }
}

impl HasWindowHandle for RawWindow {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        match self.window {
            #[cfg(target_os = "windows")]
            RawWindowHandle::Hwnd(raw_handle) => {
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
