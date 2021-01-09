//----------------------------------------------------------------------------------------------------------------------
// Copyright (c) 2021 Krzysztof Statkiewicz. All rights reserved.
// This work is licensed under the terms of the MIT license.
// For a copy, see <https://opensource.org/licenses/MIT>.
//----------------------------------------------------------------------------------------------------------------------

#[macro_use]
extern crate log;
//----------------------------------------------------------------------------------------------------------------------

pub mod core;
pub mod renderer;
pub mod utils;
//----------------------------------------------------------------------------------------------------------------------

use ultraviolet::Rotor3;
//----------------------------------------------------------------------------------------------------------------------

use crate::{
    core::{
        input::{Button, Input, Key},
        window::{init_window, DevEvt, Evt, Flow, WinEvt},
    },
    renderer::{entities::Camera, Renderer},
};
//----------------------------------------------------------------------------------------------------------------------

pub enum CoreEvent {
    CloseRequested,
}
//----------------------------------------------------------------------------------------------------------------------

const APP_NAME: &str = "Koi - WIP";
const WIDTH: usize = 1280;
const HEIGHT: usize = 720;
//----------------------------------------------------------------------------------------------------------------------

fn main() {
    info!("----- Logger::init -----");
    utils::Logger::init().unwrap();

    let (window_handle, event_loop_handle, event_proxy_handle, mut window_state) =
        init_window(APP_NAME, WIDTH, HEIGHT);
    let mut input_handle = Input::init(None);
    let mut renderer = Renderer::init(APP_NAME, &window_handle);

    info!("----- EventLoopHandle::run -----");
    event_loop_handle.run(move |event, _, control_flow| {
        let is_focused = window_state.is_focused();
        match event {
            Evt::UserEvent(event) => match event {
                CoreEvent::CloseRequested => {
                    renderer.await_device_idle();
                    *control_flow = Flow::Exit;
                }
            },
            Evt::WindowEvent { event, .. } => match event {
                WinEvt::Focused(state) => {
                    window_state.update(state);
                }
                WinEvt::CloseRequested => {
                    renderer.await_device_idle();
                    *control_flow = Flow::Exit;
                }
                _ => {}
            },
            Evt::DeviceEvent { event, .. } => {
                if is_focused {
                    match event {
                        DevEvt::Key(input) => {
                            if let Some(keycode) = input.virtual_keycode {
                                input_handle.update_keyboard_input(keycode, input.state);
                            }
                        }
                        DevEvt::Button { button, state, .. } => {
                            input_handle.update_mouse_input(button, state);
                        }
                        DevEvt::MouseMotion { delta: deltas, .. } => {
                            input_handle.update_mouse_deltas(deltas);
                        }
                        DevEvt::MouseWheel { delta, .. } => {
                            input_handle.update_mouse_scroll(delta);
                        }
                        _ => {}
                    }
                }
            }
            Evt::MainEventsCleared => {
                if is_focused {
                    input_handle.update_gamepad_input();

                    if input_handle.is_key_down(Key::Escape)
                        || (input_handle.is_button_down(Button::Start))
                    {
                        event_proxy_handle
                            .send_event(CoreEvent::CloseRequested)
                            .unwrap_or_else(|_| {
                                warn!("Failed to send CloseRequested event to event loop!")
                            });
                        return;
                    }

                    update_camera(renderer.camera_mut(), &input_handle);

                    renderer.draw();
                }
            }
            Evt::LoopDestroyed => renderer.await_device_idle(),
            _ => {}
        };
    });
}
//----------------------------------------------------------------------------------------------------------------------

fn update_camera(camera: &mut Camera, input_handle: &Input) {
    //camera.translate();
    //camera.rotate();
}
//----------------------------------------------------------------------------------------------------------------------
