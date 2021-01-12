use std::collections::{HashMap, HashSet};
//----------------------------------------------------------------------------------------------------------------------

use paste::paste;
//----------------------------------------------------------------------------------------------------------------------

use crate::core::input::{
    types::{
        ActionBindings, Button, GamepadInput, InputMode, Key, KeyboardMouseInput, Mouse,
        MouseMotion, Stick, Trigger,
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
                    use Stick::*;
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

                pub fn remove_active_context(&mut self, ctx: ActionContexts) {
                    self.active_ctxs.remove(&ctx);
                }

                pub fn is_context_active(&self, ctx: ActionContexts) -> bool {
                    self.active_ctxs.contains(&ctx)
                }

                $(
                    pub fn [<is_ $ctx:snake _action_down>] (&self, mgr: &InputManager, action: [<$ctx Actions>]) -> bool {
                        if !self.is_context_active(ActionContexts::$ctx) {
                            return false;
                        }
                        match self.[<$ctx:snake>].get(&action) {
                            Some(binding) => self.is_binding_down(binding, mgr),
                            None => false
                        }
                    }

                    pub fn [<is_ $ctx:snake _action_hold>] (&self, mgr: &InputManager, action: [<$ctx Actions>]) -> bool {
                        if !self.is_context_active(ActionContexts::$ctx) {
                            return false;
                        }
                        match self.[<$ctx:snake>].get(&action) {
                            Some(binding) => self.is_binding_hold(binding, mgr),
                            None => false
                        }
                    }

                    pub fn [<get_ $ctx:snake _action_value>] (&self, mgr: &InputManager, action: [<$ctx Actions>]) -> f32 {
                        if !self.is_context_active(ActionContexts::$ctx) {
                            return 0.0;
                        }
                        match self.[<$ctx:snake>].get(&action) {
                            Some(binding) => self.get_binding_value(binding, mgr),
                            None => 0.0
                        }
                    }
                )+

                fn is_binding_down(&self, binding: &ActionBindings, mgr: &InputManager) -> bool {
                    let input_mode = mgr.get_current_input_mode();
                    match input_mode {
                        InputMode::KBM => {
                            for input in binding.get_kbm_bindings().iter() {
                                if let Some(input) = input {
                                    if self.is_kb_input_down(input, mgr) {
                                        return true;
                                    }
                                }
                            }
                        },
                        InputMode::Gamepad => {
                            for input in binding.get_gamepad_bindings().iter() {
                                if let Some(input) = input {
                                    if self.is_gp_input_down(input, mgr) {
                                        return true;
                                    }
                                }
                            }
                        },
                    };
                    false
                }

                fn is_kb_input_down(&self, input: &KeyboardMouseInput, mgr: &InputManager) -> bool {
                    use KeyboardMouseInput::*;
                    match *input {
                        Key(key) => mgr.is_key_down(key),
                        Mouse(mouse) => mgr.is_mouse_down(mouse),
                        MouseMotion(mm) => mgr.is_mouse_in_motion(mm)
                    }
                }

                fn is_gp_input_down(&self, input: &GamepadInput, mgr: &InputManager) -> bool {
                    use GamepadInput::*;
                    match *input {
                        Button(btn) => mgr.is_button_down(btn),
                        Stick(stick) => mgr.is_stick_in_motion(stick),
                        Trigger(trigger) => mgr.is_trigger_active(trigger),
                    }
                }

                fn is_binding_hold(&self, binding: &ActionBindings, mgr: &InputManager) -> bool {
                    let input_mode = mgr.get_current_input_mode();
                    match input_mode {
                        InputMode::KBM => {
                            for input in binding.get_kbm_bindings().iter() {
                                if let Some(input) = input {
                                    if self.is_kb_input_hold(input, mgr) {
                                        return true;
                                    }
                                }
                            }
                        },
                        InputMode::Gamepad => {
                            for input in binding.get_gamepad_bindings().iter() {
                                if let Some(input) = input {
                                    if self.is_gp_input_hold(input, mgr) {
                                        return true;
                                    }
                                }
                            }
                        },
                    };
                    false
                }

                fn is_kb_input_hold(&self, input: &KeyboardMouseInput, mgr: &InputManager) -> bool {
                    match *input {
                        KeyboardMouseInput::Key(key) => mgr.is_key_hold(key),
                        KeyboardMouseInput::Mouse(mouse) => mgr.is_mouse_hold(mouse),
                        KeyboardMouseInput::MouseMotion(mm) => mgr.is_mouse_in_motion(mm),
                    }
                }

                fn is_gp_input_hold(&self, input: &GamepadInput, mgr: &InputManager) -> bool {
                    use GamepadInput::*;
                    match *input {
                        Button(btn) => mgr.is_button_hold(btn),
                        Stick(stick) => mgr.is_stick_in_motion(stick),
                        Trigger(trigger) => mgr.is_trigger_active(trigger),
                    }
                }

                fn get_binding_value(&self, binding: &ActionBindings, mgr: &InputManager) -> f32 {
                    let input_mode = mgr.get_current_input_mode();
                    match input_mode {
                        InputMode::KBM => {
                            for input in binding.get_kbm_bindings().iter() {
                                if let Some(input) = input {
                                    let value = self.get_kb_input_value(input, mgr);
                                    if value > 0.0 {
                                        return value;
                                    }
                                }
                            }
                        },
                        InputMode::Gamepad => {
                            for input in binding.get_gamepad_bindings().iter() {
                                if let Some(input) = input {
                                    let value = self.get_gp_input_value(input, mgr);
                                    if value > 0.0 {
                                        return value;
                                    }
                                }
                            }
                        },
                    };
                    0.0
                }

                fn get_kb_input_value(&self, input: &KeyboardMouseInput, mgr: &InputManager) -> f32 {
                    match *input {
                        KeyboardMouseInput::Key(key) => mgr.get_key_value(key),
                        KeyboardMouseInput::Mouse(mouse) => mgr.get_mouse_value(mouse),
                        KeyboardMouseInput::MouseMotion(mm) => mgr.get_mouse_motion(mm)
                    }
                }

                fn get_gp_input_value(&self, input: &GamepadInput, mgr: &InputManager) -> f32 {
                    use GamepadInput::*;
                    match *input {
                        Button(btn) => mgr.get_button_value(btn),
                        Stick(stick) => mgr.get_stick_value(stick),
                        Trigger(trigger) => mgr.get_trigger_value(trigger)
                    }
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
        { Forward,      Stick::LSUp,        None,   Key::W,         None }
        { Backward,     Stick::LSDown,      None,   Key::S,         None }
        { Left,         Stick::LSLeft,      None,   Key::A,         None }
        { Right,        Stick::LSRight,     None,   Key::D,         None }
        { LookUp,       Stick::RSUp,        None,   None,           None }
        { LookDown,     Stick::RSDown,      None,   None,           None }
        { LookLeft,     Stick::RSLeft,      None,   None,           None }
        { LookRight,    Stick::RSRight,     None,   None,           None }
    }
);
