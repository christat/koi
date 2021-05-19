use shinzou::define_contextual_action_bindings;
//----------------------------------------------------------------------------------------------------------------------

define_contextual_action_bindings!(
    MainMenu {
        { Up,           Button::DPadUp,         None,   Key::Up,                None        }
        { Down,         Button::DPadDown,       None,   Key::Down,              None        }
        { Left,         Button::DPadLeft,       None,   Key::Left,              None        }
        { Right,        Button::DPadRight,      None,   Key::Right,             None        }
    }
    InGame {
        { Forward,      Stick::LSUp,            None,   Key::W,                 None        }
        { Backward,     Stick::LSDown,          None,   Key::S,                 None        }
        { Left,         Stick::LSLeft,          None,   Key::A,                 None        }
        { Right,        Stick::LSRight,         None,   Key::D,                 None        }
        { LookUp,       Stick::RSUp,            None,   MouseMotion::YUp,       None        }
        { LookDown,     Stick::RSDown,          None,   MouseMotion::YDown,     None        }
        { LookLeft,     Stick::RSLeft,          None,   MouseMotion::XLeft,     None        }
        { LookRight,    Stick::RSRight,         None,   MouseMotion::XRight,    None        }
        { Sprint,       Button::LeftBumper,     None,   Key::LShift,            Key::RShift }
    }
);
//----------------------------------------------------------------------------------------------------------------------
