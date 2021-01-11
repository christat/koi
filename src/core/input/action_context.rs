use std::collections::{HashMap, HashSet};
//----------------------------------------------------------------------------------------------------------------------

use paste::paste;
//----------------------------------------------------------------------------------------------------------------------

use crate::core::input::types::{
    Axis, Button, GamepadInput, Key, KeyboardMouseInput, Mouse, MouseMotion, Trigger,
};
//----------------------------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
pub struct ActionBindings {
    kbm: Option<KeyboardMouseInput>,
    kbm_alt: Option<KeyboardMouseInput>,
    gamepad: Option<GamepadInput>,
    gamepad_alt: Option<GamepadInput>,
}

impl ActionBindings {
    pub fn default() -> Self {
        Self {
            kbm: None,
            kbm_alt: None,
            gamepad: None,
            gamepad_alt: None,
        }
    }

    pub fn kbm(mut self, kbm: KeyboardMouseInput) -> Self {
        self.kbm = Some(kbm);
        self
    }

    pub fn kbm_alt(mut self, kbm: KeyboardMouseInput) -> Self {
        self.kbm_alt = Some(kbm);
        self
    }

    pub fn gamepad(mut self, gamepad: GamepadInput) -> Self {
        self.gamepad = Some(gamepad);
        self
    }

    pub fn gamepad_alt(mut self, gamepad: GamepadInput) -> Self {
        self.gamepad_alt = Some(gamepad);
        self
    }
}

macro_rules! define_contextual_action_bindings {
    ($($ctx:ident { $({ $action:ident, $gpe:ident$(::$gpi:ident)?, $gpe2:ident$(::$gpi2:ident)?, $kbme:ident$(::$kbmi:ident)?, $kbme2:ident$(::$kbmi2:ident)? })+ })+) => {
        $(
            paste! {
                #[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
                pub enum [<$ctx Actions>] { $($action,)+ }
            }
        )+

        #[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
        pub enum ActionContexts { $($ctx,)+ }

        paste! {

            pub struct ActionContext {
                active_ctxs: HashSet<ActionContexts>,
                $(
                    [<$ctx:snake>]: HashMap<[<$ctx Actions>], ActionBindings>,
                )+
            }

            impl ActionContext {
                pub fn init() -> Self {
                    #[allow(unused_imports)]
                    use Axis::*;
                    #[allow(unused_imports)]
                    use Button::*;
                    #[allow(unused_imports)]
                    use Trigger::*;
                    #[allow(unused_imports)]
                    use Key::*;
                    #[allow(unused_imports)]
                    use Mouse::*;
                    #[allow(unused_imports)]
                    use MouseMotion::*;

                    $(
                        let [<$ctx:snake>]: HashMap<[<$ctx Actions>], ActionBindings> =
                        [
                            $(
                                (
                                    [<$ctx Actions>]::$action,
                                    ActionBindings::default()
                                    $(.gamepad(GamepadInput::$gpe($gpi)))?
                                    $(.gamepad_alt(GamepadInput::$gpe2($gpi2)))?
                                    $(.kbm(KeyboardMouseInput::$kbme($kbmi)))?
                                    $(.kbm_alt(KeyboardMouseInput::$kbme2($kbmi2)))?
                                ),
                            )+
                        ].iter().cloned().collect();
                    )+

                    Self {
                        active_ctxs: HashSet::new(),
                        $(
                            [<$ctx:snake>],
                        )+
                    }
                }

                pub fn get_active_contexts(&self) -> &HashSet<ActionContexts> {
                    &self.active_ctxs
                }

                pub fn set_active_context(&mut self, ctx: ActionContexts) {
                    self.active_ctxs.insert(ctx);
                }

                pub fn remove_active_context(&mut self, ctx: &ActionContexts) {
                    self.active_ctxs.remove(ctx);
                }
            }
        }
    };
}

define_contextual_action_bindings!(
    MainMenu {
        { Up,           Button::DPadUp,     None,   Key::Up,        None }
        { Down,         Button::DPadDown,   None,   Key::Down,      None }
        { Left,         Button::DPadLeft,   None,   Key::Left,      None }
        { Right,        Button::DPadRight,  None,   Key::Right,     None }
    }
    InGame {
        { Forward,      Axis::LSUp,         None,   Key::W,         None }
        { Backward,     Axis::LSDown,       None,   Key::S,         None }
        { Left,         Axis::LSLeft,       None,   Key::A,         None }
        { Right,        Axis::LSRight,      None,   Key::D,         None }
        { LookUp,       Axis::RSUp,         None,   None,           None }
        { LookDown,     Axis::RSDown,       None,   None,           None }
        { LookLeft,     Axis::RSLeft,       None,   None,           None }
        { LookRight,    Axis::RSRight,      None,   None,           None }
    }
);
