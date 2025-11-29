use anyhow::Result;
use wry::raw_window_handle::{HandleError, HasWindowHandle, WindowHandle};

use super::WryWebViewError;

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
    #[cfg(target_os = "windows")]
    pub fn get_hwnd(&self) -> Result<windows::Win32::Foundation::HWND, WryWebViewError> {
        use windows::Win32::Foundation::HWND;
        use wry::raw_window_handle::HasWindowHandle;

        let handle = self
            .window_handle()
            .map_err(|e| WryWebViewError::WindowHandleError(e.to_string()))?
            .as_raw();
        let hwnd = match handle {
            wry::raw_window_handle::RawWindowHandle::Win32(win32) => HWND(win32.hwnd.get() as _),
            _ => return Err(WryWebViewError::UnsupportedPlatform),
        };

        Ok(hwnd)
    }

    #[cfg(target_os = "windows")]
    fn create_child_hwnd(&self) -> Result<windows::Win32::Foundation::HWND, WryWebViewError> {
        use windows::core::*;
        use windows::Win32::Foundation::*;
        use windows::Win32::Graphics::Gdi::ValidateRect;
        use windows::Win32::System::LibraryLoader::GetModuleHandleW;
        use windows::Win32::UI::HiDpi::GetDpiForWindow;
        use windows::Win32::UI::WindowsAndMessaging::*;

        let logical_width = 800.0;
        let logical_height = 600.0;

        let parent_hwnd = self.get_hwnd()?;
        let class_name = w!("WINDOW_CLASS_FLUTTER_WRY_CHILD");

        let scale_factor = unsafe { GetDpiForWindow(parent_hwnd) as f64 / 96.0 };

        let physical_width = (logical_width * scale_factor).round() as i32;
        let physical_height = (logical_height * scale_factor).round() as i32;

        let h_instance: HINSTANCE = unsafe { GetModuleHandleW(None) }
            .map_err(|_| {
                WryWebViewError::CreateChildWindowError("Failed to get module handle".to_string())
            })?
            .into();

        extern "system" fn wndproc(
            window: HWND,
            message: u32,
            wparam: WPARAM,
            lparam: LPARAM,
        ) -> LRESULT {
            unsafe {
                match message {
                    WM_PAINT => {
                        _ = ValidateRect(Some(window), None);
                        LRESULT(0)
                    }
                    WM_DESTROY => {
                        PostQuitMessage(0);
                        LRESULT(0)
                    }
                    _ => DefWindowProcA(window, message, wparam, lparam),
                }
            }
        }

        let wnd_class = WNDCLASSW {
            lpfnWndProc: Some(wndproc),
            hInstance: h_instance,
            lpszClassName: class_name,
            // style: CS_HREDRAW | CS_VREDRAW,
            style: Default::default(),
            ..Default::default()
        };

        unsafe {
            if RegisterClassW(&wnd_class) == 0 {
                return Err(WryWebViewError::CreateChildWindowError(
                    "Failed to register window class".to_string(),
                ));
            }
        }

        let child_hwnd = unsafe {
            CreateWindowExW(
                Default::default(),
                class_name,
                w!("flutter_wry_child_window"),
                WS_CHILD | WS_VISIBLE,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                physical_width,
                physical_height,
                Some(parent_hwnd),
                None,
                Some(h_instance),
                None,
            )
            .map_err(|err| {
                WryWebViewError::CreateChildWindowError(format!(
                    "Failed to create child window, CreateWindowExW Error: {err}"
                ))
            })?
        };

        Ok(child_hwnd)
    }

    #[allow(dead_code)]
    pub fn create_child_window(&self) -> Result<Self, WryWebViewError> {
        #[cfg(target_os = "windows")]
        {
            let hwnd = self.create_child_hwnd()?;
            let window = Self::new(RawWindowHandle::Windows(hwnd.0 as isize));

            Ok(window)
        }
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
