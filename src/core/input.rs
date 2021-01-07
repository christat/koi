use std::collections::{hash_map::Entry, HashMap};
use std::time::Instant;
//----------------------------------------------------------------------------------------------------------------------

use std::fmt;
use std::hash::Hash;
use winapi::_core::convert::TryFrom;
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
        write!(f, "Failed to build pipeline!")
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

pub struct Input {
    keyboard: HashMap<Keycode, (InputState, Instant)>,
    mouse: HashMap<MouseButton, (InputState, Instant)>,
    mouse_deltas: (f64, f64),
    scroll_delta: f64,
    //------------------------------------------------------------------------------------------------------------------
    gamepad: HashMap<u32, (InputState, Instant)>,
    //------------------------------------------------------------------------------------------------------------------
    hold_time_millis: u128,
}
//----------------------------------------------------------------------------------------------------------------------

impl Input {
    pub fn init(hold_time_millis: Option<u128>) -> Self {
        Self {
            keyboard: HashMap::new(),
            mouse: HashMap::new(),
            mouse_deltas: (0.0, 0.0),
            scroll_delta: 0.0,
            //----------------------------------------------------------------------------------------------------------
            gamepad: HashMap::new(),
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

    pub fn update_gamepad_input(&mut self) {}
    //------------------------------------------------------------------------------------------------------------------

    pub fn is_key_down(&self, keycode: Keycode) -> bool {
        if let Some((state, _)) = self.keyboard.get(&keycode) {
            return match state {
                InputState::Up => false,
                _ => true,
            };
        }
        false
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn is_key_hold(&self, keycode: Keycode) -> bool {
        if let Some((state, _)) = self.keyboard.get(&keycode) {
            return match state {
                InputState::Hold => true,
                _ => false,
            };
        }
        false
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn is_mouse_down(&self, button: MouseButton) -> bool {
        if let Some((state, _)) = self.mouse.get(&button) {
            return match state {
                InputState::Up => false,
                _ => true,
            };
        }
        false
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn is_mouse_hold(&self, button: MouseButton) -> bool {
        if let Some((state, _)) = self.mouse.get(&button) {
            return match state {
                InputState::Hold => true,
                _ => false,
            };
        }
        false
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
}
//----------------------------------------------------------------------------------------------------------------------

// fn update_input_entry<T>(
//     entry: &mut OccupiedEntry<T, (InputState, Instant)>,
//     state: InputState,
//     hold_time_millis: u128,
// ) {
//     match (entry.get(), state) {
//         ((InputState::Hold, _), InputState::Down) => {}
//         ((InputState::Down, start), InputState::Down) => {
//             let now = Instant::now();
//             if now.duration_since(*start).as_millis() > hold_time_millis {
//                 entry.insert((InputState::Hold, now));
//             }
//         }
//         _ => {
//             entry.insert((state.clone(), Instant::now()));
//         }
//     };
// }
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
