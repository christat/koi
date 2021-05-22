//----------------------------------------------------------------------------------------------------------------------
// Copyright (c) 2021 Krzysztof Statkiewicz. All rights reserved.
// This work is licensed under the terms of the MIT license.
// For a copy, see <https://opensource.org/licenses/MIT>.
//----------------------------------------------------------------------------------------------------------------------

#[macro_use]
extern crate log;
//----------------------------------------------------------------------------------------------------------------------

mod input;
//----------------------------------------------------------------------------------------------------------------------

use std::time::Instant;
//----------------------------------------------------------------------------------------------------------------------

use shinzou::{
    core::{
        input::{
            types::{Button, GamepadEvent, Key},
            InputManager,
        },
        window::{init_window, Evt, Flow, WinEvt},
    },
    renderer::{entities::Camera, Renderer},
    utils::Logger,
};
use ultraviolet::{rotor::Rotor3, Vec3};
//----------------------------------------------------------------------------------------------------------------------

use input::{ActionContexts, InGameActions, InputActions};
//----------------------------------------------------------------------------------------------------------------------

#[derive(Copy, Clone, Debug)]
pub enum CoreEvent {
    CloseRequested,
    GamepadDisconnected,
}
//----------------------------------------------------------------------------------------------------------------------

const APP_NAME: &str = "IKE";
const WIDTH: usize = 1280;
const HEIGHT: usize = 720;
//----------------------------------------------------------------------------------------------------------------------

struct Timer {
    last_instant: Instant,
}
//----------------------------------------------------------------------------------------------------------------------

impl Timer {
    pub fn new() -> Self {
        Self {
            last_instant: Instant::now(),
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    fn tick(&mut self) -> f32 {
        let now = Instant::now();
        let delta = now.duration_since(self.last_instant).as_secs_f32();
        self.last_instant = now;
        delta
    }
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------

fn main() {
    info!("----- Logger::init -----");
    Logger::init().unwrap();

    let (window_handle, event_loop_handle, event_proxy, mut window_state) =
        init_window(APP_NAME, WIDTH, HEIGHT);
    let mut input_manager = InputManager::init(None);
    let mut input_actions = InputActions::init();

    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();

    let mut renderer = Renderer::init(APP_NAME, &window_handle);

    input_actions.set_active_context(ActionContexts::InGame);

    let mut timer = Timer::new();

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
                // WinEvt::Focused(state) => {
                //     window_state.update(state);
                // }
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
                    let frame_delta = timer.tick();

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

                    update_camera(
                        renderer.camera_mut(),
                        &input_actions,
                        &input_manager,
                        frame_delta,
                    );

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

fn update_camera(
    camera: &mut Camera,
    input_actions: &InputActions,
    mgr: &InputManager,
    frame_delta: f32,
) {
    let look_up = input_actions.get_in_game_action_value(mgr, InGameActions::LookUp);
    let look_down = input_actions.get_in_game_action_value(mgr, InGameActions::LookDown);
    let look_left = input_actions.get_in_game_action_value(mgr, InGameActions::LookLeft);
    let look_right = input_actions.get_in_game_action_value(mgr, InGameActions::LookRight);

    const ROT_MULTIPLIER: f32 = 10.0;

    let rotor = Rotor3::from_euler_angles(
        0.0,
        (look_down - look_up) * ROT_MULTIPLIER * frame_delta,
        (look_left - look_right) * ROT_MULTIPLIER * frame_delta,
    );
    camera.rotate(rotor);

    let fwd = input_actions.get_in_game_action_value(mgr, InGameActions::Forward);
    let bwd = input_actions.get_in_game_action_value(mgr, InGameActions::Backward);
    let left = input_actions.get_in_game_action_value(mgr, InGameActions::Left);
    let right = input_actions.get_in_game_action_value(mgr, InGameActions::Right);
    let lb_down = input_actions.is_in_game_action_down(mgr, InGameActions::Sprint);

    const WALK_MULTIPLIER: f32 = 10.0;
    const SPRINT_MULTIPLIER: f32 = 20.0;
    let multiplier = if lb_down {
        SPRINT_MULTIPLIER
    } else {
        WALK_MULTIPLIER
    };

    camera.translate(Vec3::new(
        (left - right) * multiplier * frame_delta,
        0.0,
        (fwd - bwd) * multiplier * frame_delta,
    ));
}
//----------------------------------------------------------------------------------------------------------------------
