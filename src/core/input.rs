use std::{
    collections::{hash_map::Entry, HashMap},
    convert::TryFrom,
    fmt,
    hash::Hash,
    time::Instant,
};
//----------------------------------------------------------------------------------------------------------------------

use gilrs_core::{
    AxisInfo, EvCode, Event as GamepadEvent, EventType as GamepadEventType, Gilrs,
    IS_Y_AXIS_REVERSED,
};
use winit::event::{ElementState as Event, MouseScrollDelta as ScrollDelta, VirtualKeyCode};
//----------------------------------------------------------------------------------------------------------------------

use crate::utils::math;
//----------------------------------------------------------------------------------------------------------------------

pub type Key = VirtualKeyCode;
//----------------------------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
pub enum InputState {
    Up,
    Down,
    Hold,
}
//----------------------------------------------------------------------------------------------------------------------

impl From<Event> for InputState {
    fn from(event: Event) -> Self {
        match event {
            Event::Pressed => InputState::Down,
            Event::Released => InputState::Up,
        }
    }
}
//----------------------------------------------------------------------------------------------------------------------

// TODO investigate how these codes translate across OS-es
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Mouse {
    Left,
    Middle,
    Right,
}
//----------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct MouseFromU32Error;
//----------------------------------------------------------------------------------------------------------------------

impl fmt::Display for MouseFromU32Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to obtain MouseButton code from u32!")
    }
}
//----------------------------------------------------------------------------------------------------------------------

impl TryFrom<u32> for Mouse {
    type Error = MouseFromU32Error;

    fn try_from(code: u32) -> Result<Self, Self::Error> {
        match code {
            1 => Ok(Mouse::Left),
            2 => Ok(Mouse::Middle),
            3 => Ok(Mouse::Right),
            _ => Err(Self::Error {}),
        }
    }
}
//----------------------------------------------------------------------------------------------------------------------

// TODO investigate how these codes translate across OS-es
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Axis {
    LeftStickX,
    LeftStickY,
    RightStickX,
    RightStickY,
    LeftTrigger,
    RightTrigger,
}
//----------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct AxisFromEvCodeError;
//----------------------------------------------------------------------------------------------------------------------

impl fmt::Display for AxisFromEvCodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to obtain GamepadAxis code from EvCode!")
    }
}
//----------------------------------------------------------------------------------------------------------------------

impl TryFrom<EvCode> for Axis {
    type Error = AxisFromEvCodeError;

    fn try_from(code: EvCode) -> Result<Self, Self::Error> {
        match code.into_u32() {
            0 => Ok(Axis::LeftStickX),
            1 => Ok(Axis::LeftStickY),
            3 => Ok(Axis::RightStickX),
            4 => Ok(Axis::RightStickY),
            10 => Ok(Axis::LeftTrigger),
            11 => Ok(Axis::RightTrigger),
            _ => Err(Self::Error {}),
        }
    }
}
//----------------------------------------------------------------------------------------------------------------------

// TODO investigate how these codes translate across OS-es
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Button {
    DPadUp,
    DPadDown,
    DPadLeft,
    DPadRight,
    North,
    South,
    East,
    West,
    LeftBumper,
    RightBumper,
    Select,
    Start,
}
//----------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct ButtonFromEvCodeError;
//----------------------------------------------------------------------------------------------------------------------

impl fmt::Display for ButtonFromEvCodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to obtain Button code from EvCode!")
    }
}
//----------------------------------------------------------------------------------------------------------------------

impl TryFrom<EvCode> for Button {
    type Error = ButtonFromEvCodeError;

    fn try_from(code: EvCode) -> Result<Self, Self::Error> {
        match code.into_u32() {
            27 => Ok(Button::DPadUp),
            28 => Ok(Button::DPadDown),
            29 => Ok(Button::DPadLeft),
            30 => Ok(Button::DPadRight),
            15 => Ok(Button::North),
            12 => Ok(Button::South),
            16 => Ok(Button::East),
            13 => Ok(Button::West),
            18 => Ok(Button::LeftBumper),
            19 => Ok(Button::RightBumper),
            22 => Ok(Button::Select),
            23 => Ok(Button::Start),
            _ => Err(Self::Error {}),
        }
    }
}
//----------------------------------------------------------------------------------------------------------------------

pub struct GamepadManager {
    inner: Gilrs,
}
//----------------------------------------------------------------------------------------------------------------------

impl GamepadManager {
    pub fn init() -> Self {
        let inner = Gilrs::new().expect("Input::GamepadManager - Failed to instantiate Gilrs!");

        Self { inner }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn next_event(&mut self) -> Option<GamepadEvent> {
        self.inner.next_event()
    }
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------

pub struct Input {
    keyboard: HashMap<Key, (InputState, Instant)>,
    mouse: HashMap<Mouse, (InputState, Instant)>,
    mouse_deltas: (f64, f64),
    scroll_delta: f64,
    //------------------------------------------------------------------------------------------------------------------
    gamepad_manager: GamepadManager,
    gamepad: HashMap<Button, (InputState, Instant)>,
    gamepad_axis: HashMap<Axis, f32>,
    //------------------------------------------------------------------------------------------------------------------
    hold_time_millis: u128,
}
//----------------------------------------------------------------------------------------------------------------------

impl Input {
    pub fn init(hold_time_millis: Option<u128>) -> Self {
        info!("----- Input::init -----");
        Self {
            keyboard: HashMap::new(),
            mouse: HashMap::new(),
            mouse_deltas: (0.0, 0.0),
            scroll_delta: 0.0,
            //----------------------------------------------------------------------------------------------------------
            gamepad_manager: GamepadManager::init(),
            gamepad: HashMap::new(),
            gamepad_axis: HashMap::new(),
            //----------------------------------------------------------------------------------------------------------
            hold_time_millis: hold_time_millis.unwrap_or(500),
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn update_keyboard_input(&mut self, key: Key, event: Event) {
        update_input_entry(
            &mut self.keyboard,
            key,
            InputState::from(event),
            self.hold_time_millis,
        );
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn update_mouse_input(&mut self, code: u32, event: Event) {
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

    pub fn update_mouse_deltas(&mut self, delta: (f64, f64)) {
        self.mouse_deltas = delta;
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn update_mouse_scroll(&mut self, delta: ScrollDelta) {
        match delta {
            ScrollDelta::LineDelta(delta, _) => self.scroll_delta = delta as f64,
            ScrollDelta::PixelDelta(logical_pos) => self.scroll_delta = logical_pos.y,
        };
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn update_gamepad_input(&mut self) {
        // flush gamepad event queue. TODO Investigate if there's a better way to implement this...
        while let Some(gamepad_event) = self.gamepad_manager.next_event() {
            let GamepadEvent {
                event: r#type, id, ..
            } = gamepad_event;

            match r#type {
                GamepadEventType::Connected => {}
                GamepadEventType::Disconnected => {}
                GamepadEventType::AxisValueChanged(value, code) => {
                    if let Some(gamepad) = self.gamepad_manager.inner.gamepad(id) {
                        if let Ok(axis) = Axis::try_from(code) {
                            if let Some(axis_info) = gamepad.axis_info(code) {
                                let v = clamp_axis_value(axis, axis_info, value);
                                info!("{:?} value: {}", axis, v);
                                self.gamepad_axis.insert(axis, v);
                            }
                        }
                    }
                }
                GamepadEventType::ButtonPressed(code) => {
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
                GamepadEventType::ButtonReleased(code) => {
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

    pub fn get_axis_offset(&self, axis: Axis) -> f32 {
        match self.gamepad_axis.get(&axis) {
            Some(offset) => offset.clone(),
            None => 0.0,
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

fn clamp_axis_value(axis: Axis, info: &AxisInfo, value: i32) -> f32 {
    let AxisInfo { min, max, .. } = *info;
    let mut range = max as f32 - min as f32;
    let mut val = value as f32 - min as f32;

    if (max - min) % 2 == 1 {
        range += 1.0;
        val += 1.0;
    }

    val = val / range * 2.0 - 1.0;

    if IS_Y_AXIS_REVERSED && (axis == Axis::LeftStickY || axis == Axis::RightStickY) && val != 0.0 {
        val = -val;
    }

    math::clamp_f(val, -1.0, 1.0)
}
//----------------------------------------------------------------------------------------------------------------------
