use winit::{
    event_loop::{ControlFlow, EventLoop},
};

mod render;

use render::Application;

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut application = Application::uninitialized();

    event_loop
        .run_app(&mut application)
        .expect("Event loop error");
}
