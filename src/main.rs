use winit::event_loop::EventLoop;

use crate::app::event_loop::AppHandler;
use crate::app::window::WindowConfig;

mod app;

fn main() {
    let event_loop =
        EventLoop::new().expect("Failed to create event loop");

    let config = WindowConfig::default();

    let mut app = AppHandler::new(config);

    event_loop
        .run_app(&mut app)
        .expect("Application crashed");
}