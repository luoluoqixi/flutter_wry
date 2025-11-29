use anyhow::Result;
use wry::raw_window_handle::{HandleError, HasWindowHandle, WindowHandle};

unsafe impl Send for RawWindow {}
unsafe impl Sync for RawWindow {}

pub enum RawWindowHandle {
    #[cfg(target_os = "windows")]
    Windows(isize),
    #[cfg(target_os = "macos")]
    MacOS(isize),
    #[cfg(target_os = "linux")]
    Linux(isize),
    #[cfg(target_os = "android")]
    Android(jobject),
    #[cfg(target_os = "ios")]
    Ios(*mut objc::runtime::Object),
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
    #[cfg(target_os = "windows")]
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        use std::num::{NonZero, NonZeroIsize};
        use wry::raw_window_handle::Win32WindowHandle;

        let RawWindowHandle::Windows(raw_handle) = self.window;

        let non_zero_window_handle = NonZero::new(raw_handle).ok_or(HandleError::Unavailable)?;
        let nz =
            NonZeroIsize::try_from(non_zero_window_handle).map_err(|_| HandleError::Unavailable)?;
        let handle = Win32WindowHandle::new(nz);

        unsafe {
            Ok(WindowHandle::borrow_raw(
                wry::raw_window_handle::RawWindowHandle::Win32(handle),
            ))
        }
    }

    #[cfg(target_os = "macos")]
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        use core::ptr::NonNull;
        use wry::raw_window_handle::{AppKitWindowHandle, RawWindowHandle};

        let RawWindowHandle::MacOS(raw_handle) = self.window;

        let ptr = raw_handle as *mut std::ffi::c_void;
        let non_null = NonNull::new(ptr).ok_or(HandleError::Unavailable)?;

        let handle = AppKitWindowHandle::new(non_null.cast());
        Ok(unsafe { WindowHandle::borrow_raw(RawWindowHandle::AppKit(handle)) })
    }
}
