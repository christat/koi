use std::{convert::TryFrom, fmt};
//----------------------------------------------------------------------------------------------------------------------

pub use gilrs_core::{
    AxisInfo, EvCode, Event as GamepadEventMeta, EventType as GamepadEvent, Gilrs,
    IS_Y_AXIS_REVERSED,
};
pub use winit::event::{
    ElementState as Event, MouseScrollDelta as ScrollDelta, VirtualKeyCode as Key,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Mouse {
    LMB,
    MMB,
    RMB,
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

// TODO investigate how these codes translate across OS-es
impl TryFrom<u32> for Mouse {
    type Error = MouseFromU32Error;

    fn try_from(code: u32) -> Result<Self, Self::Error> {
        match code {
            1 => Ok(Mouse::LMB),
            2 => Ok(Mouse::MMB),
            3 => Ok(Mouse::RMB),
            _ => Err(Self::Error {}),
        }
    }
}
//----------------------------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MouseMotion {
    YUp,
    YDown,
    XLeft,
    XRight,
}
//----------------------------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Axis {
    LSUp,
    LSDown,
    LSLeft,
    LSRight,
    RSUp,
    RSDown,
    RSLeft,
    RSRight,
}
//----------------------------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Trigger {
    LT,
    RT,
}
//----------------------------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum HwAxis {
    LeftStickX,
    LeftStickY,
    RightStickX,
    RightStickY,
    LeftTrigger,
    RightTrigger,
}
//----------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct HwAxisFromEvCodeError;
//----------------------------------------------------------------------------------------------------------------------

impl fmt::Display for HwAxisFromEvCodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to obtain GamepadAxis code from EvCode!")
    }
}
//----------------------------------------------------------------------------------------------------------------------

// TODO investigate how these codes translate across OS-es
impl TryFrom<EvCode> for HwAxis {
    type Error = HwAxisFromEvCodeError;

    fn try_from(code: EvCode) -> Result<Self, Self::Error> {
        match code.into_u32() {
            0 => Ok(HwAxis::LeftStickX),
            1 => Ok(HwAxis::LeftStickY),
            3 => Ok(HwAxis::RightStickX),
            4 => Ok(HwAxis::RightStickY),
            10 => Ok(HwAxis::RightTrigger),
            11 => Ok(HwAxis::LeftTrigger),
            _ => Err(Self::Error {}),
        }
    }
}
//----------------------------------------------------------------------------------------------------------------------

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
    LeftStick,
    RightStick,
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

// TODO investigate how these codes translate across OS-es
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
            25 => Ok(Button::LeftStick),
            26 => Ok(Button::RightStick),
            _ => Err(Self::Error {}),
        }
    }
}
//----------------------------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum KeyboardMouseInput {
    Key(Key),
    Mouse(Mouse),
    MouseMotion(MouseMotion),
}
//----------------------------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GamepadInput {
    Button(Button),
    Axis(Axis),
    Trigger(Trigger),
}
//----------------------------------------------------------------------------------------------------------------------
