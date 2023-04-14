use winit::{event_loop::EventLoop, window::Window};

use crate::RunError;

#[derive(Debug)]
pub struct TriangleWindow {
    pub event_loop: EventLoop<()>,
    pub window: Window,
}

impl TriangleWindow {
    pub fn new() -> Result<Self, RunError> {
        let event_loop = EventLoop::new();
        let window = Window::new(&event_loop)?;

        Ok(Self {
            event_loop,
            window,
        })
    }
}
