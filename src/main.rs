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

fn main() {
    utils::Logger::init().unwrap();
    info!("----- Logger::init -----");

    let app_name = "Vullkan MVP";

    let mut window = app::Window::init(app_name, 800, 600);

    let _renderer_backend = renderer::RendererBackend::init(app_name, window.get_window_handle());

    window
        .run_loop()
        .expect("Main - window threw an error while running loop!");
}
//-----------------------------------------------------------------------------
