use std::error::Error;
//----------------------------------------------------------------------------------------------------------------------

use winit::{
    event::{DeviceEvent, Event, VirtualKeyCode as Key, WindowEvent},
    event_loop::ControlFlow,
    platform::desktop::EventLoopExtDesktop,
};
//----------------------------------------------------------------------------------------------------------------------

use crate::{
    core::{Input, Window},
    renderer::Renderer,
};
//----------------------------------------------------------------------------------------------------------------------

pub enum CoreEvent {
    CloseRequested,
}
//----------------------------------------------------------------------------------------------------------------------

pub struct App {
    window: Window<CoreEvent>,
    renderer: Renderer,
    input: Input,
}
//----------------------------------------------------------------------------------------------------------------------

impl App {
    pub fn init(name: &str) -> Self {
        info!("----- App::init -----");
        let window = Window::init(name, 1280, 720);
        let renderer = Renderer::init(name, &window);
        let input = Input::init(None);

        Self {
            window,
            renderer,
            input,
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        info!("----- App::run -----");

        let Self {
            renderer,
            window,
            input: input_handle,
        } = self;

        let Window {
            event_loop_handle,
            focused,
            ..
        } = window;

        let event_loop_proxy = event_loop_handle.create_proxy();

        event_loop_handle.run_return(|event, _, control_flow| {
            match event {
                Event::UserEvent(event) => match event {
                    CoreEvent::CloseRequested => {
                        renderer.await_device_idle();
                        *control_flow = ControlFlow::Exit;
                    }
                },
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::Focused(state) => {
                        *focused = state;
                    }
                    WindowEvent::CloseRequested => {
                        renderer.await_device_idle();
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => {}
                },
                Event::DeviceEvent { event, .. } => {
                    if *focused {
                        match event {
                            DeviceEvent::Key(input) => {
                                if let Some(keycode) = input.virtual_keycode {
                                    input_handle.update_keyboard_input(keycode, input.state);
                                }
                            }
                            DeviceEvent::Button { button, state, .. } => {
                                info!("button event sent for: {}", button);
                                input_handle.update_mouse_input(button, state);
                            }
                            DeviceEvent::MouseMotion { delta: deltas, .. } => {
                                input_handle.update_mouse_deltas(deltas);
                            }
                            DeviceEvent::MouseWheel { delta, .. } => {
                                input_handle.update_mouse_scroll(delta);
                            }
                            _ => {}
                        }
                    }
                }
                Event::MainEventsCleared => {
                    if *focused {
                        if input_handle.is_key_down(Key::Escape) {
                            event_loop_proxy
                                .send_event(CoreEvent::CloseRequested)
                                .unwrap_or_else(|_| {
                                    warn!("Failed to send CloseRequested event to event loop!")
                                });
                            return;
                        }

                        renderer.draw();
                    }
                }
                Event::LoopDestroyed => renderer.await_device_idle(),
                _ => {}
            };
        });

        Ok(())
    }
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------
