use std::error::Error as StdError;
use std::fmt;

/// Top-level error type for the GAVAR application shell (Phase 1).
#[derive(Debug)]
pub enum AppError {
    Winit(winit::error::OsError),
    EventLoop(winit::error::EventLoopError),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Winit(err) => write!(f, "winit OS error: {err}"),
            AppError::EventLoop(err) => write!(f, "event loop error: {err}"),
        }
    }
}

impl StdError for AppError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            AppError::Winit(err) => Some(err),
            AppError::EventLoop(err) => Some(err),
        }
    }
}

impl From<winit::error::OsError> for AppError {
    fn from(err: winit::error::OsError) -> Self {
        AppError::Winit(err)
    }
}

impl From<winit::error::EventLoopError> for AppError {
    fn from(err: winit::error::EventLoopError) -> Self {
        AppError::EventLoop(err)
    }
}
