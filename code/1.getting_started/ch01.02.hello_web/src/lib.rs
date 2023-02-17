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

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            wasm_logger::init(wasm_logger::Config::default());
        } else {
            env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
        }
    }

    log::info!("Creating a new with with size 800x600");
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Hello Window")
        .with_inner_size(PhysicalSize::new(800, 600))
        .build(&event_loop)
        .unwrap();

    #[cfg(target_arch = "wasm32")]
    {
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.get_element_by_id("wasm-container")?;
                let canvas = web_sys::Element::from(window.canvas());
                dst.append_child(&canvas).ok()?;
                Some(())
            })
            .expect("Couldn't append canvas to document body.");
    }

    event_loop.run(move |event, _, control_flow| event_loop_handler(&event, control_flow, &window));
}
