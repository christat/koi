extern crate winit;
//----------------------------------------------------------------------------------------------------------------------

use winit::{
    dpi::LogicalSize,
    event_loop::EventLoop,
    window::{Window as WinitWindow, WindowBuilder},
};
//----------------------------------------------------------------------------------------------------------------------

pub struct Window {
    event_loop: EventLoop<()>,
    window: WinitWindow,
}
//----------------------------------------------------------------------------------------------------------------------

impl Window {
    pub fn init(title: &str, width: usize, height: usize) -> Self {
        info!("----- Window::init -----");

        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_inner_size(LogicalSize::new(width as f64, height as f64))
            .with_title(title)
            .build(&event_loop)
            .expect("Window::init - Failed to build window!");

        Self { event_loop, window }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn get_window_handle(&self) -> &WinitWindow {
        &self.window
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn get_event_loop_handle(&mut self) -> &mut EventLoop<()> {
        &mut self.event_loop
    }
}
//----------------------------------------------------------------------------------------------------------------------
