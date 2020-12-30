extern crate winit;
//----------------------------------------------------------------------------------------------------------------------

use winit::{
    dpi::LogicalSize,
    event_loop::EventLoop,
    window::{Window as WinitWindow, WindowBuilder},
};
//----------------------------------------------------------------------------------------------------------------------

pub struct Window {
    pub event_loop_handle: EventLoop<()>,
    pub window_handle: WinitWindow,
}
//----------------------------------------------------------------------------------------------------------------------

impl Window {
    pub fn init(title: &str, width: usize, height: usize) -> Self {
        info!("----- Window::init -----");

        let event_loop_handle = EventLoop::new();
        let window_handle = WindowBuilder::new()
            .with_inner_size(LogicalSize::new(width as f64, height as f64))
            .with_title(title)
            .build(&event_loop_handle)
            .expect("Window::init - Failed to build window!");

        Self {
            event_loop_handle,
            window_handle,
        }
    }
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------
