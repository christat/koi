use std::error::Error;
//----------------------------------------------------------------------------------------------------------------------

use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
    platform::desktop::EventLoopExtDesktop,
};
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

        let Window {
            window_handle,
            event_loop_handle,
        } = &mut self.window;

        let renderer = &mut self.renderer;

        event_loop_handle.run_return(|event, _, control_flow| {
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        renderer.await_device_idle();
                        *control_flow = ControlFlow::Exit;
                    }
                    WindowEvent::KeyboardInput { input, .. } => match input {
                        KeyboardInput {
                            virtual_keycode,
                            state,
                            ..
                        } => match (virtual_keycode, state) {
                            (Some(VirtualKeyCode::Escape), ElementState::Pressed) => {
                                renderer.await_device_idle();
                                *control_flow = ControlFlow::Exit;
                            }
                            (Some(VirtualKeyCode::Space), ElementState::Pressed) => {
                                renderer.swap_pipelines();
                            }
                            _ => {}
                        },
                    },
                    _ => *control_flow = ControlFlow::Poll,
                },
                Event::MainEventsCleared => window_handle.request_redraw(),
                Event::RedrawRequested(_window_id) => renderer.run(),
                Event::LoopDestroyed => renderer.await_device_idle(),
                _ => *control_flow = ControlFlow::Poll,
            };
        });

        Ok(())
    }
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------
