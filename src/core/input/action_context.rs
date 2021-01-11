use std::collections::{HashMap, HashSet};
//----------------------------------------------------------------------------------------------------------------------

use paste::paste;
//----------------------------------------------------------------------------------------------------------------------

use crate::core::input::{
    types::{
        ActionBindings, Axis, Button, GamepadInput, Key, KeyboardMouseInput, Mouse, MouseMotion,
        Trigger,
    },
    InputManager,
};
//----------------------------------------------------------------------------------------------------------------------

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

            pub struct InputActions {
                active_ctxs: HashSet<ActionContexts>,
                $(
                    [<$ctx:snake>]: HashMap<[<$ctx Actions>], ActionBindings>,
                )+
            }

            impl InputActions {
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
                                    ActionBindings::builder()
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

                $(
                    pub fn [<is_ $ctx:snake _action_down>] (&self, action: &[<$ctx Actions>], mgr: &InputManager) -> bool {
                        match self.[<$ctx:snake>].get(action) {
                            Some(binding) => mgr.is_binding_down(binding),
                            None => false
                        }
                    }

                    pub fn [<is_ $ctx:snake _action_hold>] (&self, action: &[<$ctx Actions>], mgr: &InputManager) -> bool {
                        match self.[<$ctx:snake>].get(action) {
                            Some(binding) => mgr.is_binding_hold(binding),
                            None => false
                        }
                    }

                    pub fn [<get_ $ctx:snake _action_value>] (&self, action: &[<$ctx Actions>], mgr: &InputManager) -> f32 {
                        match self.[<$ctx:snake>].get(action) {
                            Some(binding) => mgr.get_binding_value(binding),
                            None => 0.0
                        }
                    }
                )+
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
