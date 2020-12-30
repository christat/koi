use std::error::Error;
//----------------------------------------------------------------------------------------------------------------------

use winit::event::{Event, WindowEvent};
use winit::event_loop::ControlFlow;
use winit::platform::desktop::EventLoopExtDesktop;
//----------------------------------------------------------------------------------------------------------------------

use crate::{core::window::Window, renderer::Renderer};
//----------------------------------------------------------------------------------------------------------------------

pub struct App {
    window: Window,
    renderer: Renderer,
}
//----------------------------------------------------------------------------------------------------------------------

impl App {
    pub fn init(name: &str) -> Self {
        info!("----- App::init -----");
        let window = Window::init(name, 800, 600);
        let renderer = Renderer::init(name, &window);

        Self { window, renderer }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        info!("----- App::run -----");

        let window = &mut self.window;
        let renderer = &mut self.renderer;

        window
            .get_event_loop_handle()
            .run_return(|event, _, control_flow| {
                match event {
                    Event::WindowEvent { event, .. } => match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        _ => *control_flow = ControlFlow::Poll,
                    },
                    _ => *control_flow = ControlFlow::Poll,
                }
                renderer.run();
            });

        Ok(())
    }
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------
