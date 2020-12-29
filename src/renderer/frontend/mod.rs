use crate::app::window::Window;
use crate::renderer::RendererBackend;
use std::error::Error;
use winit::event::{Event, WindowEvent};
use winit::event_loop::ControlFlow;
use winit::platform::desktop::EventLoopExtDesktop;
//----------------------------------------------------------------------------------------------------------------------

pub struct Renderer {
    backend: RendererBackend,
}
//----------------------------------------------------------------------------------------------------------------------

impl Renderer {
    pub fn init(app_name: &str, window: &Window) -> Self {
        info!("----- Renderer::init -----");

        let backend = RendererBackend::init(app_name, window.get_window_handle());

        Self { backend }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn draw(&mut self) {
        println!("CALLED DRAW!");
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn run(&mut self, window: &mut Window) -> Result<(), Box<dyn Error>> {
        info!("----- Renderer::run -----");

        window
            .get_event_loop_handle()
            .run_return(|event, _, control_flow| {
                match event {
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
                };
                self.draw();
            });

        Ok(())
    }
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------
