use std::{
    collections::{hash_map::Entry, HashMap},
    convert::TryFrom,
    fmt,
    hash::Hash,
    time::Instant,
};
//----------------------------------------------------------------------------------------------------------------------

use gilrs_core::{EvCode, Event as GamepadEvent, EventType as GamepadEventType, Gilrs};
use winit::event::{
    ElementState as Event, MouseScrollDelta as ScrollDelta, VirtualKeyCode as Keycode,
};
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
pub enum MouseButton {
    Left,
    Middle,
    Right,
}
//----------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct MouseButtonFromU32Error;
//----------------------------------------------------------------------------------------------------------------------

impl fmt::Display for MouseButtonFromU32Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to obtain MouseButton code from u32!")
    }
}
//----------------------------------------------------------------------------------------------------------------------

impl TryFrom<u32> for MouseButton {
    type Error = MouseButtonFromU32Error;

    fn try_from(code: u32) -> Result<Self, Self::Error> {
        match code {
            1 => Ok(MouseButton::Left),
            2 => Ok(MouseButton::Middle),
            3 => Ok(MouseButton::Right),
            _ => Err(Self::Error {}),
        }
    }
}
//----------------------------------------------------------------------------------------------------------------------

// TODO investigate how these codes translate across OS-es
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GamepadAxis {
    LeftStickX,
    LeftStickY,
    RightStickX,
    RightStickY,
    LeftTrigger,
    RightTrigger,
}
//----------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct GamepadAxisFromEvCodeError;
//----------------------------------------------------------------------------------------------------------------------

impl fmt::Display for GamepadAxisFromEvCodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to obtain GamepadAxis code from EvCode!")
    }
}
//----------------------------------------------------------------------------------------------------------------------

impl TryFrom<EvCode> for GamepadAxis {
    type Error = GamepadAxisFromEvCodeError;

    fn try_from(code: EvCode) -> Result<Self, Self::Error> {
        match code.into_u32() {
            0 => Ok(GamepadAxis::LeftStickX),
            1 => Ok(GamepadAxis::LeftStickY),
            3 => Ok(GamepadAxis::RightStickX),
            4 => Ok(GamepadAxis::RightStickY),
            10 => Ok(GamepadAxis::LeftTrigger),
            11 => Ok(GamepadAxis::RightTrigger),
            _ => Err(Self::Error {}),
        }
    }
}
//----------------------------------------------------------------------------------------------------------------------

// TODO investigate how these codes translate across OS-es
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GamepadButton {
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
pub struct GamepadButtonFromEvCodeError;
//----------------------------------------------------------------------------------------------------------------------

impl fmt::Display for GamepadButtonFromEvCodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to obtain GamepadButton code from EvCode!")
    }
}
//----------------------------------------------------------------------------------------------------------------------

impl TryFrom<EvCode> for GamepadButton {
    type Error = GamepadButtonFromEvCodeError;

    fn try_from(code: EvCode) -> Result<Self, Self::Error> {
        match code.into_u32() {
            27 => Ok(GamepadButton::DPadUp),
            28 => Ok(GamepadButton::DPadDown),
            29 => Ok(GamepadButton::DPadLeft),
            30 => Ok(GamepadButton::DPadRight),
            15 => Ok(GamepadButton::North),
            12 => Ok(GamepadButton::South),
            16 => Ok(GamepadButton::East),
            13 => Ok(GamepadButton::West),
            18 => Ok(GamepadButton::LeftBumper),
            19 => Ok(GamepadButton::RightBumper),
            22 => Ok(GamepadButton::Select),
            23 => Ok(GamepadButton::Start),
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
    keyboard: HashMap<Keycode, (InputState, Instant)>,
    mouse: HashMap<MouseButton, (InputState, Instant)>,
    mouse_deltas: (f64, f64),
    scroll_delta: f64,
    //------------------------------------------------------------------------------------------------------------------
    gamepad_manager: GamepadManager,
    gamepad: HashMap<GamepadButton, (InputState, Instant)>,
    gamepad_axis: HashMap<GamepadAxis, i32>,
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

    pub fn update_keyboard_input(&mut self, keycode: Keycode, event: Event) {
        update_input_entry(
            &mut self.keyboard,
            keycode,
            InputState::from(event),
            self.hold_time_millis,
        );
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn update_mouse_input(&mut self, code: u32, event: Event) {
        if let Ok(button) = MouseButton::try_from(code) {
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
            let GamepadEvent { event: r#type, .. } = gamepad_event;

            match r#type {
                GamepadEventType::Connected => {}
                GamepadEventType::Disconnected => {}
                GamepadEventType::AxisValueChanged(value, code) => {
                    if let Ok(axis) = GamepadAxis::try_from(code) {
                        self.gamepad_axis.insert(axis, value);
                    }
                }
                GamepadEventType::ButtonPressed(code) => {
                    if let Ok(button) = GamepadButton::try_from(code) {
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
                    if let Ok(button) = GamepadButton::try_from(code) {
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

    pub fn is_key_down(&self, keycode: Keycode) -> bool {
        match self.keyboard.get(&keycode) {
            Some((state, _)) => match state {
                InputState::Up => false,
                _ => true,
            },
            None => false,
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn is_key_hold(&self, keycode: Keycode) -> bool {
        match self.keyboard.get(&keycode) {
            Some((state, _)) => match state {
                InputState::Hold => true,
                _ => false,
            },
            None => false,
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn is_mouse_down(&self, button: MouseButton) -> bool {
        match self.mouse.get(&button) {
            Some((state, _)) => match state {
                InputState::Up => false,
                _ => true,
            },
            None => false,
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn is_mouse_hold(&self, button: MouseButton) -> bool {
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

    pub fn is_button_down(&self, button: GamepadButton) -> bool {
        match self.gamepad.get(&button) {
            Some((state, _)) => match state {
                InputState::Up => false,
                _ => true,
            },
            None => false,
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn is_button_hold(&self, button: GamepadButton) -> bool {
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

    pub fn get_axis_offset(&self, axis: GamepadAxis) -> i32 {
        match self.gamepad_axis.get(&axis) {
            Some(offset) => offset.clone(),
            None => 0,
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
