//-----------------------------------------------------------------------------
// Copyright (c) 2020 Krzysztof Statkiewicz. All rights reserved.
// This work is licensed under the terms of the MIT license.
// For a copy, see <https://opensource.org/licenses/MIT>.
//-----------------------------------------------------------------------------

#[macro_use]
extern crate log;
//-----------------------------------------------------------------------------

pub mod app;
pub mod renderer;
pub mod utils;
//-----------------------------------------------------------------------------

use renderer::Renderer;
//-----------------------------------------------------------------------------

fn main() {
    utils::Logger::init().unwrap();
    info!("----- Logger::init -----");

    let app_name = "Vulkan MVP";
    let mut window = app::window::Window::init(app_name, 800, 600);
    let mut renderer = Renderer::init(app_name, &window);

    renderer.run(&mut window);
}
//-----------------------------------------------------------------------------
