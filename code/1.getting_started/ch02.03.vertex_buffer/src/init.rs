// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

cfg_if::cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        use wasm_bindgen::prelude::wasm_bindgen;
        use winit::platform::web::WindowExtWebSys;
    }
}
use winit::dpi::PhysicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

use crate::{Error, State};

fn event_loop_handler<T>(event: &Event<T>, control_flow: &mut ControlFlow, state: &mut State) {
    match event {
        Event::WindowEvent {
            ref event,
            window_id,
            ..
        } if *window_id == state.window().id() => {
            if !state.input(event) {
                match event {
                    WindowEvent::Resized(physical_size) => state.resize(*physical_size),
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        state.resize(**new_inner_size);
                    }
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    _ => {}
                }
            }
        }
        Event::RedrawRequested(window_id) if *window_id == state.window().id() => {
            state.update();
            match state.render() {
                Ok(_) => {}
                Err(wgpu::SurfaceError::Lost) => state.resize(state.size()),
                Err(wgpu::SurfaceError::OutOfMemory) => {
                    log::error!("System out of memory!");
                    *control_flow = ControlFlow::Exit;
                }
                Err(err) => log::error!("{err:?}"),
            }
        }

        Event::MainEventsCleared => state.window().request_redraw(),
        _ => {}
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() -> Result<(), Error> {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            wasm_logger::init(wasm_logger::Config::default());
        } else {
            env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
        }
    }

    log::info!("Creating a new with with size 800x600");
    let event_loop = EventLoop::new();
    let window: Window = WindowBuilder::new()
        .with_title("Vertex Buffer")
        .with_inner_size(PhysicalSize::new(800, 600))
        .build(&event_loop)?;

    #[cfg(target_arch = "wasm32")]
    {
        let element = web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| doc.get_element_by_id("wasm-container"))
            .ok_or_else(|| Error::Web("failed to find dom element '#wasm-container'".to_owned()))?;
        let canvas = web_sys::Element::from(window.canvas());
        element.append_child(&canvas)?;
    }

    let mut state = State::new(window).await?;

    event_loop
        .run(move |event, _, control_flow| event_loop_handler(&event, control_flow, &mut state));
}
