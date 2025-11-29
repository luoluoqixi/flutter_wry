#[cfg(target_os = "macos")]
use tao::platform::macos::WindowExtMacOS;
#[cfg(target_os = "windows")]
use tao::platform::windows::WindowExtWindows;
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use flutter_wry::wry_webview::{
    window::RawWindowHandle,
    wry_webview_config::{WryWebViewConfig, WryWebViewSize},
    wry_webview_controller::WryWebViewController,
};

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let config = WryWebViewConfig::new()
        .with_initial_url("https://baidu.com")
        .with_initial_devtools(true)
        .with_initial_size(WryWebViewSize::new(800.0, 400.0));

    #[cfg(target_os = "windows")]
    let handle = RawWindowHandle::Windows(window.hwnd());
    #[cfg(target_os = "macos")]
    let handle = RawWindowHandle::MacOS(window.ns_view() as isize);

    let _controller = WryWebViewController::create(config, handle).unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = ControlFlow::Exit;
        }
    });
}
