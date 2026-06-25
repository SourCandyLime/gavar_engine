use winit::dpi::LogicalSize;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowAttributes};

use crate::app::AppError;

#[derive(Debug, Clone)]
pub struct WindowConfig {
    pub title: &'static str,
    pub width: u32,
    pub height: u32,
    pub resizable: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: "GAVAR_Engine",
            width: 1280,
            height: 720,
            resizable: true,
        }
    }
}

fn window_attributes(config: &WindowConfig) -> WindowAttributes {
    Window::default_attributes()
        .with_title(config.title)
        .with_inner_size(LogicalSize::new(config.width, config.height))
        .with_resizable(config.resizable)
}

pub fn create_window(
    event_loop: &ActiveEventLoop,
    config: &WindowConfig,
) -> Result<Window, AppError> {
    let attrs = window_attributes(config);
    let window = event_loop.create_window(attrs)?;
    Ok(window)
}