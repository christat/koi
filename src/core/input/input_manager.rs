use std::{
    collections::{hash_map::Entry, HashMap},
    convert::TryFrom,
    hash::Hash,
    time::Instant,
};
//----------------------------------------------------------------------------------------------------------------------

//----------------------------------------------------------------------------------------------------------------------

use crate::core::input::types::ActionBindings;
use crate::{
    core::{
        input::types::{
            AxisInfo, Button, Event, GamepadEvent, GamepadEventMeta, Gilrs, HwAxis, InputState,
            Key, Mouse, ScrollDelta, IS_Y_AXIS_REVERSED,
        },
        window::DevEvt,
    },
    utils::math,
};
//----------------------------------------------------------------------------------------------------------------------

pub struct InputManager {
    keyboard: HashMap<Key, (InputState, Instant)>,
    mouse: HashMap<Mouse, (InputState, Instant)>,
    mouse_deltas: (f64, f64),
    scroll_delta: f64,
    //------------------------------------------------------------------------------------------------------------------
    gamepad_manager: Gilrs,
    gamepad: HashMap<Button, (InputState, Instant)>,
    gamepad_axis: HashMap<HwAxis, f32>,
    //------------------------------------------------------------------------------------------------------------------
    hold_time_millis: u128,
}
//----------------------------------------------------------------------------------------------------------------------

impl InputManager {
    pub fn init(hold_time_millis: Option<u128>) -> Self {
        info!("----- Input::init -----");

        let gamepad_manager = Gilrs::new().expect("Input::init - Failed to instantiate Gilrs!");

        Self {
            keyboard: HashMap::new(),
            mouse: HashMap::new(),
            mouse_deltas: (0.0, 0.0),
            scroll_delta: 0.0,
            //----------------------------------------------------------------------------------------------------------
            gamepad_manager,
            gamepad: HashMap::new(),
            gamepad_axis: HashMap::new(),
            //----------------------------------------------------------------------------------------------------------
            hold_time_millis: hold_time_millis.unwrap_or(500),
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn update_keyboard_mouse_input(&mut self, event: DevEvt) {
        match event {
            DevEvt::Key(input) => {
                if let Some(keycode) = input.virtual_keycode {
                    self.update_keyboard_input(keycode, input.state);
                }
            }
            DevEvt::Button { button, state, .. } => {
                self.update_mouse_input(button, state);
            }
            DevEvt::MouseMotion { delta: deltas, .. } => {
                self.update_mouse_deltas(deltas);
            }
            DevEvt::MouseWheel { delta, .. } => {
                self.update_mouse_scroll(delta);
            }
            _ => {}
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    fn update_keyboard_input(&mut self, key: Key, event: Event) {
        update_input_entry(
            &mut self.keyboard,
            key,
            InputState::from(event),
            self.hold_time_millis,
        );
    }
    //------------------------------------------------------------------------------------------------------------------

    fn update_mouse_input(&mut self, code: u32, event: Event) {
        if let Ok(button) = Mouse::try_from(code) {
            update_input_entry(
                &mut self.mouse,
                button,
                InputState::from(event),
                self.hold_time_millis,
            );
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    fn update_mouse_deltas(&mut self, delta: (f64, f64)) {
        self.mouse_deltas = delta;
    }
    //------------------------------------------------------------------------------------------------------------------

    fn update_mouse_scroll(&mut self, delta: ScrollDelta) {
        match delta {
            ScrollDelta::LineDelta(delta, _) => self.scroll_delta = delta as f64,
            ScrollDelta::PixelDelta(logical_pos) => self.scroll_delta = logical_pos.y,
        };
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn update_gamepad_input(&mut self) -> Option<GamepadEvent> {
        // flush gamepad event queue. TODO Investigate if there's a better way to implement this...
        while let Some(gamepad_event) = self.gamepad_manager.next_event() {
            let GamepadEventMeta {
                event: r#type, id, ..
            } = gamepad_event;

            match r#type {
                GamepadEvent::Connected => {
                    info!("Input - Gamepad connected (id: {})", id);
                }
                GamepadEvent::Disconnected => {
                    info!("Input - Gamepad disconnected (id: {})", id);
                    return Some(GamepadEvent::Disconnected);
                }
                GamepadEvent::AxisValueChanged(value, code) => {
                    if let Some(gamepad) = self.gamepad_manager.gamepad(id) {
                        if let Ok(axis) = HwAxis::try_from(code) {
                            if let Some(axis_info) = gamepad.axis_info(code) {
                                let v = clamp_hw_axis_value(axis, axis_info, value);
                                self.gamepad_axis.insert(axis, v);
                            }
                        }
                    }
                }
                GamepadEvent::ButtonPressed(code) => {
                    if let Ok(button) = Button::try_from(code) {
                        match self.gamepad.entry(button) {
                            Entry::Vacant(entry) => {
                                entry.insert((InputState::Down, Instant::now()));
                            }
                            Entry::Occupied(mut entry) => match entry.get() {
                                (InputState::Up, _) => {
                                    entry.insert((InputState::Down, Instant::now()));
                                }
                                (_, _) => {}
                            },
                        };
                    }
                }
                GamepadEvent::ButtonReleased(code) => {
                    if let Ok(button) = Button::try_from(code) {
                        match self.gamepad.entry(button) {
                            Entry::Vacant(entry) => {
                                entry.insert((InputState::Up, Instant::now()));
                            }
                            Entry::Occupied(mut entry) => {
                                match entry.get() {
                                    (InputState::Down, _) => {}
                                    (_, _) => {
                                        entry.insert((InputState::Up, Instant::now()));
                                    }
                                };
                            }
                        };
                    }
                }
            };
        }

        None
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn is_key_down(&self, key: Key) -> bool {
        match self.keyboard.get(&key) {
            Some((state, _)) => match state {
                InputState::Up => false,
                _ => true,
            },
            None => false,
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn is_key_hold(&self, key: Key) -> bool {
        match self.keyboard.get(&key) {
            Some((state, _)) => match state {
                InputState::Hold => true,
                _ => false,
            },
            None => false,
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn is_mouse_down(&self, button: Mouse) -> bool {
        match self.mouse.get(&button) {
            Some((state, _)) => match state {
                InputState::Up => false,
                _ => true,
            },
            None => false,
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn is_mouse_hold(&self, button: Mouse) -> bool {
        match self.mouse.get(&button) {
            Some((state, _)) => match state {
                InputState::Hold => true,
                _ => false,
            },
            None => false,
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn get_mouse_deltas(&self) -> (f64, f64) {
        self.mouse_deltas
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn get_mouse_scroll(&self) -> f64 {
        self.scroll_delta
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn is_button_down(&self, button: Button) -> bool {
        match self.gamepad.get(&button) {
            Some((state, _)) => match state {
                InputState::Up => false,
                _ => true,
            },
            None => false,
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn is_button_hold(&self, button: Button) -> bool {
        // We need to derive the hold state as Gilrs only sends Up/Down events once. TODO revisit
        let now = Instant::now();
        match self.gamepad.get(&button) {
            Some((state, change_instant)) => match state {
                InputState::Down => {
                    if now.duration_since(*change_instant).as_millis() > self.hold_time_millis {
                        true
                    } else {
                        false
                    }
                }
                _ => false,
            },
            None => false,
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn get_axis_offset(&self, axis: HwAxis) -> f32 {
        match self.gamepad_axis.get(&axis) {
            Some(offset) => offset.clone(),
            None => 0.0,
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn is_binding_down(&self, binding: &ActionBindings) -> bool {
        unimplemented!()
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn is_binding_hold(&self, binding: &ActionBindings) -> bool {
        unimplemented!()
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn get_binding_value(&self, binding: &ActionBindings) -> f32 {
        unimplemented!()
    }
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------

fn update_input_entry<T>(
    hash_map: &mut HashMap<T, (InputState, Instant)>,
    key: T,
    state: InputState,
    hold_time_millis: u128,
) where
    T: Eq + Hash,
{
    match hash_map.entry(key) {
        Entry::Vacant(entry) => {
            entry.insert((state, Instant::now()));
        }
        Entry::Occupied(mut entry) => match (entry.get(), state) {
            ((InputState::Hold, _), InputState::Down) => {}
            ((InputState::Down, start), InputState::Down) => {
                let now = Instant::now();
                if now.duration_since(*start).as_millis() > hold_time_millis {
                    entry.insert((InputState::Hold, now));
                }
            }
            _ => {
                entry.insert((state.clone(), Instant::now()));
            }
        },
    };
}
//----------------------------------------------------------------------------------------------------------------------

fn clamp_hw_axis_value(hw_axis: HwAxis, info: &AxisInfo, value: i32) -> f32 {
    let AxisInfo { min, max, deadzone } = *info;
    let range = max as f32 - min as f32;
    let mut val = value as f32 - min as f32;

    if hw_axis == HwAxis::LeftTrigger || hw_axis == HwAxis::RightTrigger {
        val = val / range;
    } else {
        val = val / range * 2.0 - 1.0;
    }

    if IS_Y_AXIS_REVERSED
        && (hw_axis == HwAxis::LeftStickY || hw_axis == HwAxis::RightStickY)
        && val != 0.0
    {
        val = -val;
    }

    val = math::clamp_f(val, -1.0, 1.0);

    match deadzone {
        None => val,
        Some(dz) => {
            if val.abs() < (dz as f32 / range * 2.0) {
                return 0.0;
            } else {
                val
            }
        }
    }
}
//----------------------------------------------------------------------------------------------------------------------
