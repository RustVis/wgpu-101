// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use winit::dpi::PhysicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

fn event_loop_handler<T>(event: &Event<T>, control_flow: &mut ControlFlow, window: &Window) {
    match event {
        Event::WindowEvent {
            ref event,
            window_id,
            ..
        } if *window_id == window.id() && event == &WindowEvent::CloseRequested => {
            *control_flow = ControlFlow::Exit;
        }
        _ => {}
    }
}

/// # Errors
/// Returns error if failed to create main window.
pub fn run() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("Creating a new with with size 800x600");
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Hello Window")
        .with_inner_size(PhysicalSize::new(800, 600))
        .build(&event_loop)?;

    event_loop.run(move |event, _, control_flow| event_loop_handler(&event, control_flow, &window));
}
