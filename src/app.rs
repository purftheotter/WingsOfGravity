use std::sync::Arc;
use crate::game::game::Game;

use winit::{
    application::ApplicationHandler,
    event::*,
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::PhysicalKey,
    window::Window
};


pub struct App {
    game_state: Option<Game>,
}

impl App {
    pub fn new() -> Self {
        Self { 
            game_state: None,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes();
        let window = 
            Arc::new(event_loop.create_window(window_attributes).unwrap());
        self.game_state = Some(pollster::block_on(Game::new(window, event_loop)).unwrap())
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let game_state = match &mut self.game_state {
            Some(canvas) => canvas,
            None => return,
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => game_state.resize(size.width, size.height),
            WindowEvent::RedrawRequested => {
                if let Err(e) = game_state.tick() {
                    log::error!("{e}");
                    event_loop.exit();
                }
                if !game_state.running {
                    event_loop.exit();
                }
            },

            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state: key_state,
                        ..
                    },
                ..
            } => game_state.handle_key(
                event_loop,
                code,
                key_state.is_pressed()
            ),

            _ => {}
        }
    }
}

pub fn run() -> anyhow::Result<()> {
    crate::logging::init();

    let event_loop = EventLoop::new()?;
    let mut app = App::new();
    event_loop.run_app(&mut app)?;

    Ok(())
}

