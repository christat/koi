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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
    ScrollUp,
    ScrollDown,
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
        use Mouse::*;
        match code {
            1 => Ok(LMB),
            2 => Ok(MMB),
            3 => Ok(RMB),
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
pub enum Stick {
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
        use HwAxis::*;
        match code.into_u32() {
            0 => Ok(LeftStickX),
            1 => Ok(LeftStickY),
            3 => Ok(RightStickX),
            4 => Ok(RightStickY),
            10 => Ok(RightTrigger),
            11 => Ok(LeftTrigger),
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
        use Button::*;
        match code.into_u32() {
            27 => Ok(DPadUp),
            28 => Ok(DPadDown),
            29 => Ok(DPadLeft),
            30 => Ok(DPadRight),
            15 => Ok(North),
            12 => Ok(South),
            16 => Ok(East),
            13 => Ok(West),
            18 => Ok(LeftBumper),
            19 => Ok(RightBumper),
            22 => Ok(Select),
            23 => Ok(Start),
            25 => Ok(LeftStick),
            26 => Ok(RightStick),
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
    Stick(Stick),
    Trigger(Trigger),
}
//----------------------------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputMode {
    KBM,
    Gamepad,
}
//----------------------------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
pub struct ActionBindings {
    pub kbm: Option<KeyboardMouseInput>,
    pub kbm_alt: Option<KeyboardMouseInput>,
    pub gamepad: Option<GamepadInput>,
    pub gamepad_alt: Option<GamepadInput>,
}
//----------------------------------------------------------------------------------------------------------------------

impl ActionBindings {
    pub fn builder() -> Self {
        Self {
            kbm: None,
            kbm_alt: None,
            gamepad: None,
            gamepad_alt: None,
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn kbm(mut self, kbm: KeyboardMouseInput) -> Self {
        self.kbm = Some(kbm);
        self
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn kbm_alt(mut self, kbm: KeyboardMouseInput) -> Self {
        self.kbm_alt = Some(kbm);
        self
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn gamepad(mut self, gamepad: GamepadInput) -> Self {
        self.gamepad = Some(gamepad);
        self
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn gamepad_alt(mut self, gamepad: GamepadInput) -> Self {
        self.gamepad_alt = Some(gamepad);
        self
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn get_gamepad_bindings(&self) -> [Option<GamepadInput>; 2] {
        [self.gamepad, self.gamepad_alt]
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn get_kbm_bindings(&self) -> [Option<KeyboardMouseInput>; 2] {
        [self.kbm, self.kbm_alt]
    }
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------
