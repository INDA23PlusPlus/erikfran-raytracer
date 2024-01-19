use std::{borrow::Cow, env::args};
use renderdoc::RenderDoc;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

mod compute;
mod fragment;

fn main() {
    let event_loop = EventLoop::new();
    let window = winit::window::Window::new(&event_loop).unwrap();
    window.set_inner_size(winit::dpi::LogicalSize::new(256, 256));
    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::init();
        if args().nth(1) == Some("frag".to_string()) {
            pollster::block_on(fragment::run(event_loop, window));
        } else if args().nth(1) == Some("comp".to_string()) {
            pollster::block_on(compute::run(event_loop, window));
        }
    }
    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init().expect("could not initialize logger");
        use winit::platform::web::WindowExtWebSys;
        // On wasm, append the canvas to the document body
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| doc.body())
            .and_then(|body| {
                body.append_child(&web_sys::Element::from(window.canvas()))
                    .ok()
            })
            .expect("couldn't append canvas to document body");
        wasm_bindgen_futures::spawn_local(run(event_loop, window));
    }
}
