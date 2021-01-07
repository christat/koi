extern crate winit;
//----------------------------------------------------------------------------------------------------------------------

use winit::{
    dpi::LogicalSize,
    event_loop::EventLoop,
    window::{Window as WinitWindow, WindowBuilder},
};
//----------------------------------------------------------------------------------------------------------------------

pub struct Window<T: 'static> {
    pub event_loop_handle: EventLoop<T>,
    pub window_handle: WinitWindow,
    //------------------------------------------------------------------------------------------------------------------
    pub focused: bool,
}
//----------------------------------------------------------------------------------------------------------------------

impl<T> Window<T> {
    pub fn init(title: &str, width: usize, height: usize) -> Self {
        info!("----- Window::init -----");

        let event_loop_handle = EventLoop::with_user_event();
        let window_handle = WindowBuilder::new()
            .with_inner_size(LogicalSize::new(width as f64, height as f64))
            .with_title(title)
            .build(&event_loop_handle)
            .expect("Window::init - Failed to build window!");

        // window_handle.set_cursor_grab(true);
        // window_handle.set_cursor_visible(false);

        Self {
            event_loop_handle,
            window_handle,
            focused: true,
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
    }
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------
