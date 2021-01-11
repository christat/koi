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

use crate::{
    core::{
        input::{
            types::{Button, GamepadEvent, Key},
            InputManager,
        },
        window::{init_window, Evt, Flow, WinEvt},
    },
    renderer::Renderer,
};
//----------------------------------------------------------------------------------------------------------------------

#[derive(Copy, Clone, Debug)]
pub enum CoreEvent {
    CloseRequested,
    GamepadDisconnected,
}
//----------------------------------------------------------------------------------------------------------------------

const APP_NAME: &str = "Koi - WIP";
const WIDTH: usize = 1280;
const HEIGHT: usize = 720;
//----------------------------------------------------------------------------------------------------------------------

fn main() {
    info!("----- Logger::init -----");
    utils::Logger::init().unwrap();

    let (window_handle, event_loop_handle, event_proxy, mut window_state) =
        init_window(APP_NAME, WIDTH, HEIGHT);
    let mut input_handle = InputManager::init(None);
    let mut renderer = Renderer::init(APP_NAME, &window_handle);

    info!("----- EventLoopHandle::run -----");
    event_loop_handle.run(move |event, _, control_flow| {
        let is_focused = window_state.is_focused();
        match event {
            Evt::UserEvent(event) => match event {
                CoreEvent::GamepadDisconnected => {
                    window_state.update(false);
                    warn!("TODO IMPLEMENT - Gamepad disconnected!")
                }
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
                    input_handle.update_keyboard_mouse_input(event);
                }
            }
            Evt::MainEventsCleared => {
                if is_focused {
                    if let Some(evt) = input_handle.update_gamepad_input() {
                        if evt == GamepadEvent::Disconnected {
                            event_proxy.emit(CoreEvent::GamepadDisconnected);
                        }
                    }

                    if input_handle.is_key_down(Key::Escape)
                        || (input_handle.is_button_down(Button::Start))
                    {
                        event_proxy.emit(CoreEvent::CloseRequested);
                    }

                    // update_camera(renderer.camera_mut(), &input_handle);

                    renderer.draw();
                }
            }
            Evt::LoopDestroyed => renderer.await_device_idle(),
            _ => {}
        };
    });
}
//----------------------------------------------------------------------------------------------------------------------

// fn update_camera(camera: &mut Camera, input_handle: &InputManager) {
//      camera.translate();
//      camera.rotate();
// }
//----------------------------------------------------------------------------------------------------------------------
