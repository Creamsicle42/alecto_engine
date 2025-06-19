#![allow(unused)]
pub mod assets;
pub mod renderer;
use std::time::Instant;

use eyre::{Ok, Result};
use winit::{application::ApplicationHandler, event::WindowEvent, event_loop::EventLoop, platform};

fn main() -> Result<()> {
    let asset_file_path = std::env::current_exe()?.with_file_name("assets.zip");
    let asset_file = std::fs::File::open(asset_file_path)?;

    let event_loop = EventLoop::new()?;

    // Initialize App State
    let mut app_state = AppState {
        asset_manager: Some(assets::AssetManager::new(vec![asset_file])?),
        ..Default::default()
    };
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    event_loop.run_app(&mut app_state);

    Ok(())
}

#[derive(Debug, Default)]
struct AppState {
    render_state: Option<renderer::RenderState>,
    asset_manager: Option<assets::AssetManager>,
}

impl ApplicationHandler for AppState {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.render_state = Some(renderer::RenderState::setup_state(event_loop));
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(new_size) => {
                if let Some(render_state) = &mut self.render_state {
                    render_state.resize(new_size);
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(render_state) = &mut self.render_state {
                    render_state.redraw();
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if let (Some(render_sate)) = (&self.render_state) {
            render_sate.request_redraw();
        }
    }
}
