use std::fmt::Debug;
//----------------------------------------------------------------------------------------------------------------------

use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    event_loop::EventLoopProxy,
    window::WindowBuilder,
};
//----------------------------------------------------------------------------------------------------------------------

pub use winit::{
    event::{DeviceEvent as DevEvt, VirtualKeyCode as Key},
    event_loop::ControlFlow as Flow,
    window::Window as WindowHandle,
};

pub type EventLoopHandle<T> = EventLoop<T>;
pub type Evt<'a, T> = Event<'a, T>;
pub type WinEvt<'a> = WindowEvent<'a>;
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

pub struct EventProxy<T: 'static + Debug + Clone> {
    proxy: EventLoopProxy<T>,
}
//----------------------------------------------------------------------------------------------------------------------

impl<T: 'static + Debug + Clone> EventProxy<T> {
    pub fn init(event_loop_handle: &EventLoop<T>) -> Self {
        Self {
            proxy: event_loop_handle.create_proxy(),
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn emit(&self, event: T) {
        self.proxy
            .send_event(event.clone())
            .unwrap_or_else(|_| warn!("Failed to send event {:?} to event loop!", event));
    }
}
//----------------------------------------------------------------------------------------------------------------------

pub fn init_window<T: 'static + Debug + Clone>(
    title: &str,
    width: usize,
    height: usize,
) -> (WindowHandle, EventLoopHandle<T>, EventProxy<T>, WindowState) {
    info!("----- Window::init -----");
    let event_loop_handle = EventLoop::with_user_event();
    let event_proxy = EventProxy::init(&event_loop_handle);

    let window_handle = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(width as f64, height as f64))
        .with_title(title)
        .build(&event_loop_handle)
        .expect("Window::init_window - Failed to build window!");

    (
        window_handle,
        event_loop_handle,
        event_proxy,
        WindowState { focused: true },
    )
}
//----------------------------------------------------------------------------------------------------------------------
