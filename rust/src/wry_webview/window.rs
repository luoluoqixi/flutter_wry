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
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        match self.window {
            #[cfg(target_os = "windows")]
            RawWindowHandle::Windows(raw_handle) => {
                use std::num::NonZeroIsize;
                use wry::raw_window_handle::{RawWindowHandle, Win32WindowHandle};

                let nz = NonZeroIsize::new(raw_handle).ok_or(HandleError::Unavailable)?;
                let handle = Win32WindowHandle::new(nz);
                Ok(unsafe { WindowHandle::borrow_raw(RawWindowHandle::Win32(handle)) })
            }
            #[cfg(target_os = "macos")]
            RawWindowHandle::MacOS(raw_handle) => {
                use core::ptr::NonNull;
                use wry::raw_window_handle::{AppKitWindowHandle, RawWindowHandle};

                let ptr = raw_handle as *mut std::ffi::c_void;
                let non_null = NonNull::new(ptr).ok_or(HandleError::Unavailable)?;

                let handle = AppKitWindowHandle::new(non_null.cast());
                Ok(unsafe { WindowHandle::borrow_raw(RawWindowHandle::AppKit(handle)) })
            }
            #[allow(unreachable_patterns)]
            _ => Err(HandleError::NotSupported),
        }
    }
}
