extern crate winit;
//----------------------------------------------------------------------------------------------------------------------

use winit::{
    dpi::LogicalSize,
    event::{DeviceEvent, Event, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
    event_loop::EventLoop,
    event_loop::EventLoopProxy,
    window::{Window as WinitWindow, WindowBuilder},
};
//----------------------------------------------------------------------------------------------------------------------

pub type WindowHandle = WinitWindow;
pub type EventLoopHandle<T> = EventLoop<T>;
pub type EventProxyHandle<T> = EventLoopProxy<T>;
pub type Key = VirtualKeyCode;
pub type Evt<'a, T> = Event<'a, T>;
pub type DevEvt = DeviceEvent;
pub type WinEvt<'a> = WindowEvent<'a>;
pub type Flow = ControlFlow;
//----------------------------------------------------------------------------------------------------------------------

pub struct WindowState {
    focused: bool,
}
//----------------------------------------------------------------------------------------------------------------------

impl WindowState {
    pub fn update(&mut self, focused: bool) {
        self.focused = focused;
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn is_focused(&self) -> bool {
        self.focused
    }
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------

pub fn init_window<T>(
    title: &str,
    width: usize,
    height: usize,
) -> (
    WindowHandle,
    EventLoopHandle<T>,
    EventProxyHandle<T>,
    WindowState,
) {
    info!("----- Window::init -----");
    let event_loop_handle = EventLoop::with_user_event();
    let event_proxy_handle = event_loop_handle.create_proxy();

    let window_handle = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(width as f64, height as f64))
        .with_title(title)
        .build(&event_loop_handle)
        .expect("Window::init_window - Failed to build window!");

    (
        window_handle,
        event_loop_handle,
        event_proxy_handle,
        WindowState { focused: true },
    )
}
//----------------------------------------------------------------------------------------------------------------------
