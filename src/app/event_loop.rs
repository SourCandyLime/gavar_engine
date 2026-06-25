use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::Window;

use crate::app::time::Clock;
use crate::app::window::{create_window, WindowConfig};

pub struct AppHandler {
    window: Option<Window>,
    clock: Clock,
    config: WindowConfig,
}

impl AppHandler {
    pub fn new(config: WindowConfig) -> Self {
        Self {
            window: None,
            clock: Clock::new(),
            config,
        }
    }
}

impl ApplicationHandler for AppHandler {
    fn resumed(
        &mut self,
        event_loop: &ActiveEventLoop,
    ) {
        if self.window.is_none() {
            let window =
                create_window(event_loop, &self.config)
                    .expect("Failed to create window");
    
            self.window = Some(window);
        }
    }
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.clock.tick();
            
                let fps: f32 = self.clock.fps();

                println!(
                    "FPS: {:.2}",
                    fps
                );
                
                

                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            _ => {},

        }
    }
}