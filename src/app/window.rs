extern crate winit;
//-----------------------------------------------------------------------------

use std::error::Error;
//----------------------------------------------------------------------------------------------------------------------

use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::desktop::EventLoopExtDesktop,
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

    pub fn run_loop(&mut self) -> Result<(), Box<dyn Error>> {
        self.event_loop
            .run_return(|event, _, control_flow| match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::Resized(resolution) => {
                        let (width, height): (u32, u32) = resolution.into();
                        if width == 0 || height == 0 {
                            *control_flow = ControlFlow::Wait;
                        } else {
                            *control_flow = ControlFlow::Poll;
                        }
                    }
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    _ => *control_flow = ControlFlow::Poll,
                },
                _ => *control_flow = ControlFlow::Poll,
            });

        Ok(())
    }
}
//----------------------------------------------------------------------------------------------------------------------
