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

use ultraviolet::{rotor::Rotor3, Vec3};
//----------------------------------------------------------------------------------------------------------------------

use crate::{
    core::{
        input::{
            types::{Button, GamepadEvent, Key},
            ActionContexts, InGameActions, InputActions, InputManager,
        },
        window::{init_window, Evt, Flow, WinEvt},
    },
    renderer::{entities::Camera, Renderer},
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
    let mut input_manager = InputManager::init(None);
    let mut input_actions = InputActions::init();
    let mut renderer = Renderer::init(APP_NAME, &window_handle);

    input_actions.set_active_context(ActionContexts::InGame);

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
                    input_manager.update_keyboard_mouse_input(event);
                }
            }
            Evt::MainEventsCleared => {
                if is_focused {
                    if let Some(evt) = input_manager.update_gamepad_input() {
                        if evt == GamepadEvent::Disconnected {
                            event_proxy.emit(CoreEvent::GamepadDisconnected);
                        }
                    }

                    if input_manager.is_key_down(Key::Escape)
                        || (input_manager.is_button_down(Button::Start))
                    {
                        event_proxy.emit(CoreEvent::CloseRequested);
                    }

                    update_camera(renderer.camera_mut(), &input_actions, &input_manager);

                    renderer.draw();

                    input_manager.post_update();
                }
            }
            Evt::LoopDestroyed => renderer.await_device_idle(),
            _ => {}
        };
    });
}
//----------------------------------------------------------------------------------------------------------------------

fn update_camera(camera: &mut Camera, input_actions: &InputActions, mgr: &InputManager) {
    let look_up = input_actions.get_in_game_action_value(mgr, InGameActions::LookUp);
    let look_down = input_actions.get_in_game_action_value(mgr, InGameActions::LookDown);
    let look_left = input_actions.get_in_game_action_value(mgr, InGameActions::LookLeft);
    let look_right = input_actions.get_in_game_action_value(mgr, InGameActions::LookRight);

    const ROT_MULTIPLIER: f32 = 0.3;

    let rotor = Rotor3::from_euler_angles(
        0.0,
        (look_down - look_up) * ROT_MULTIPLIER,
        (look_left - look_right) * ROT_MULTIPLIER,
    );
    camera.rotate(rotor);

    let fwd = input_actions.get_in_game_action_value(mgr, InGameActions::Forward);
    let bwd = input_actions.get_in_game_action_value(mgr, InGameActions::Backward);
    let left = input_actions.get_in_game_action_value(mgr, InGameActions::Left);
    let right = input_actions.get_in_game_action_value(mgr, InGameActions::Right);
    let lb_down = input_actions.is_in_game_action_down(mgr, InGameActions::Sprint);

    const WALK_MULTIPLIER: f32 = 0.2;
    const SPRINT_MULTIPLIER: f32 = 0.8;
    let multiplier = if lb_down {
        SPRINT_MULTIPLIER
    } else {
        WALK_MULTIPLIER
    };

    camera.translate(Vec3::new(
        (left - right) * multiplier,
        0.0,
        (fwd - bwd) * multiplier,
    ));
}
//----------------------------------------------------------------------------------------------------------------------
