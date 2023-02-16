// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

fn event_loop_handler(event: Event, control_flow: ControlFlow, window: Window) {
    match event {
        Event::WindowEvent {
            ref window_event,
            window_id,
        } if window_id == window.id() => match window_event {
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
        },
        _ => {}
    }
}

pub fn run() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    event_loop.run(move |event, _, control_flow| event_loop_handler(event, control_flow, window));
}
