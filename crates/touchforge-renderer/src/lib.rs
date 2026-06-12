use anyhow::Result;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowAttributes},
};

pub struct Renderer {
    window: Option<Window>,
}

impl Renderer {
    pub fn run() -> Result<()> {
        let event_loop = EventLoop::new()?;

        let mut app = Renderer {
            window: None,
        };

        event_loop.run_app(&mut app)?;

        Ok(())
    }
}

impl ApplicationHandler for Renderer {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(
                WindowAttributes::default()
                    .with_title("TouchForge"),
            )
            .unwrap();

        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        if let WindowEvent::CloseRequested = event {
            event_loop.exit();
        }
    }
}
