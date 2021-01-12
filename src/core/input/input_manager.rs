use std::{
    collections::{hash_map::Entry, HashMap},
    convert::TryFrom,
    hash::Hash,
    time::Instant,
};
//----------------------------------------------------------------------------------------------------------------------

//----------------------------------------------------------------------------------------------------------------------

use crate::core::input::types::{EvCode, MouseMotion, Stick, Trigger};
use crate::{
    core::{
        input::types::{
            AxisInfo, Button, Event, GamepadEvent, GamepadEventMeta, Gilrs, HwAxis, InputMode,
            InputState, Key, Mouse, ScrollDelta, IS_Y_AXIS_REVERSED,
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
    current_input_mode: InputMode,
    last_input_update: Instant,
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
            current_input_mode: InputMode::Gamepad,
            last_input_update: Instant::now(),
            hold_time_millis: hold_time_millis.unwrap_or(500),
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn update_gamepad_input(&mut self) -> Option<GamepadEvent> {
        // flush gamepad event queue. TODO Investigate if there's a better way to implement this...
        while let Some(gamepad_event) = self.gamepad_manager.next_event() {
            let GamepadEventMeta {
                event: r#type,
                id,
                time,
            } = gamepad_event;

            if self.current_input_mode != InputMode::Gamepad {
                if let Ok(duration) = time.elapsed() {
                    let now = Instant::now();
                    if duration < now.duration_since(self.last_input_update) {
                        self.current_input_mode = InputMode::Gamepad;
                        self.last_input_update = now;
                    }
                }
            }

            use GamepadEvent::*;
            match r#type {
                Connected => {
                    info!("Input - Gamepad connected (id: {})", id);
                }
                Disconnected => {
                    info!("Input - Gamepad disconnected (id: {})", id);
                    return Some(Disconnected);
                }
                AxisValueChanged(value, code) => {
                    if let Some(gamepad) = self.gamepad_manager.gamepad(id) {
                        if let Ok(axis) = HwAxis::try_from(code) {
                            if let Some(axis_info) = gamepad.axis_info(code) {
                                let v = clamp_hw_axis_value(axis, axis_info, value);
                                self.gamepad_axis.insert(axis, v);
                            }
                        }
                    }
                }
                ButtonPressed(code) => self.update_button_state(code, InputState::Down),
                ButtonReleased(code) => self.update_button_state(code, InputState::Up),
            };
        }

        None
    }
    //------------------------------------------------------------------------------------------------------------------

    fn update_button_state(&mut self, code: EvCode, new_state: InputState) {
        if let Ok(button) = Button::try_from(code) {
            match self.gamepad.entry(button) {
                Entry::Vacant(entry) => {
                    entry.insert((new_state, Instant::now()));
                }
                Entry::Occupied(mut entry) => match entry.get() {
                    (input_state, _) => {
                        if *input_state != new_state {
                            entry.insert((new_state, Instant::now()));
                        }
                    }
                },
            };
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn update_keyboard_mouse_input(&mut self, event: DevEvt) {
        // NB! takes advantage of the fact KBM gets spammed from Winit and skip duration checks.
        if self.current_input_mode != InputMode::KBM {
            self.last_input_update = Instant::now();
            self.current_input_mode = InputMode::KBM;
        }

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

    pub fn get_current_input_mode(&self) -> InputMode {
        self.current_input_mode
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

    pub fn get_key_value(&self, key: Key) -> f32 {
        match self.is_key_down(key) {
            true => 1.0,
            false => 0.0,
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn is_mouse_down(&self, button: Mouse) -> bool {
        use Mouse::*;
        match button {
            ScrollDown | ScrollUp => self.is_scroll_active(button),
            _ => match self.mouse.get(&button) {
                Some((state, _)) => match state {
                    InputState::Up => false,
                    _ => true,
                },
                None => false,
            },
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn is_mouse_hold(&self, button: Mouse) -> bool {
        use Mouse::*;
        match button {
            ScrollDown | ScrollUp => self.is_scroll_active(button),
            _ => match self.mouse.get(&button) {
                Some((state, _)) => match state {
                    InputState::Hold => true,
                    _ => false,
                },
                None => false,
            },
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn get_mouse_value(&self, button: Mouse) -> f32 {
        match self.is_mouse_down(button) {
            true => 1.0,
            false => 0.0,
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn is_scroll_active(&self, button: Mouse) -> bool {
        use Mouse::*;
        match button {
            ScrollUp => self.scroll_delta > 0.0,
            ScrollDown => self.scroll_delta < 0.0,
            _ => false,
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn is_mouse_in_motion(&self, mouse_motion: MouseMotion) -> bool {
        use MouseMotion::*;
        let (x_delta, y_delta) = self.mouse_deltas;
        match mouse_motion {
            XLeft => x_delta < 0.0,
            XRight => x_delta > 0.0,
            YUp => y_delta < 0.0,
            YDown => y_delta > 0.0,
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn get_mouse_motion(&self, mouse_motion: MouseMotion) -> f32 {
        use MouseMotion::*;
        let (x_delta, y_delta) = self.mouse_deltas;
        match mouse_motion {
            XLeft => (-x_delta as f32).max(0.0),
            XRight => (x_delta as f32).max(0.0),
            YUp => (-y_delta as f32).max(0.0),
            YDown => (y_delta as f32).max(0.0),
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

    pub fn get_button_value(&self, button: Button) -> f32 {
        match self.is_button_down(button) {
            true => 1.0,
            false => 0.0,
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

    pub fn is_stick_in_motion(&self, stick: Stick) -> bool {
        use HwAxis::*;
        use Stick::*;
        match stick {
            LSUp | LSDown => {
                let offset = self.get_axis_offset(LeftStickY);
                if stick == LSUp {
                    offset > 0.0
                } else {
                    offset < 0.0
                }
            }
            LSLeft | LSRight => {
                let offset = self.get_axis_offset(LeftStickX);
                if stick == LSRight {
                    offset > 0.0
                } else {
                    offset < 0.0
                }
            }
            RSUp | RSDown => {
                let offset = self.get_axis_offset(RightStickY);
                if stick == RSUp {
                    offset > 0.0
                } else {
                    offset < 0.0
                }
            }
            RSLeft | RSRight => {
                let offset = self.get_axis_offset(RightStickX);
                if stick == RSRight {
                    offset > 0.0
                } else {
                    offset < 0.0
                }
            }
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn get_stick_value(&self, stick: Stick) -> f32 {
        use HwAxis::*;
        use Stick::*;
        match stick {
            LSUp | LSDown => {
                let offset = self.get_axis_offset(LeftStickY);
                if stick == LSUp {
                    offset.max(0.0)
                } else {
                    (-offset).max(0.0)
                }
            }
            LSLeft | LSRight => {
                let offset = self.get_axis_offset(LeftStickX);
                if stick == LSRight {
                    offset.max(0.0)
                } else {
                    (-offset).max(0.0)
                }
            }
            RSUp | RSDown => {
                let offset = self.get_axis_offset(RightStickY);
                if stick == RSUp {
                    offset.max(0.0)
                } else {
                    (-offset).max(0.0)
                }
            }
            RSLeft | RSRight => {
                let offset = self.get_axis_offset(RightStickX);
                if stick == RSRight {
                    offset.max(0.0)
                } else {
                    (-offset).max(0.0)
                }
            }
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn is_trigger_active(&self, trigger: Trigger) -> bool {
        self.get_trigger_value(trigger) > 0.0
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn get_trigger_value(&self, trigger: Trigger) -> f32 {
        match trigger {
            Trigger::LT => self.get_axis_offset(HwAxis::LeftTrigger),
            Trigger::RT => self.get_axis_offset(HwAxis::RightTrigger),
        }
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
