//-----------------------------------------------------------------------------
// Copyright (c) 2020 Krzysztof Statkiewicz. All rights reserved.
// This work is licensed under the terms of the MIT license.
// For a copy, see <https://opensource.org/licenses/MIT>.
//-----------------------------------------------------------------------------

#[macro_use]
extern crate log;
//-----------------------------------------------------------------------------

pub mod core;
pub mod renderer;
pub mod utils;
//-----------------------------------------------------------------------------

use crate::core::App;
//-----------------------------------------------------------------------------

fn main() {
    info!("----- Logger::init -----");
    utils::Logger::init().unwrap();

    App::init("Vulkan MVP")
        .run()
        .expect("main - App::run threw an error!");
}
//-----------------------------------------------------------------------------
