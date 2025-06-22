#![allow(unused)]
pub mod assets;
pub mod game;
pub mod registries;
pub mod renderer;
use log::*;
use std::time::Instant;

use eyre::{Ok, Result};
use winit::{application::ApplicationHandler, event::WindowEvent, event_loop::EventLoop, platform};

fn main() -> Result<()> {
    // Initialize Logger
    pretty_env_logger::init();

    // Locate asset file
    let asset_file_path = std::env::current_exe()?.with_file_name("assets.zip");
    let asset_file = std::fs::File::open(asset_file_path)?;
    let mut asset_manager = assets::AssetManager::new(vec![asset_file])?;

    // TODO: Find init files in asset files and run registry builder scripts
    let registry_command_queue = registries::RegistryQueue::default();
    let init_script = asset_manager.get_file_raw("init.lua".to_owned());
    let lua = mlua::Lua::new_with(
        mlua::StdLib::TABLE & mlua::StdLib::STRING & mlua::StdLib::MATH,
        mlua::LuaOptions::new(),
    )
    .expect("Failed to create Lua state");

    // Upload registry queue object to lua state
    lua.globals()
        .set("Registry", registry_command_queue.clone());
    lua.load(init_script.expect("init.lua not present")).exec();

    // Purge excess lua functionality
    lua.globals().set("Registry", mlua::Nil);

    let registries = registries::GameRegistries::new(registry_command_queue, &mut asset_manager);

    // TODO: Create a dummy game state and apply commands from init scripts

    // Initialize App State
    let mut app_state = AppState {
        asset_manager,
        registry_manager: registries,
        render_state: None,
        game_state: game::GameState::new(lua),
    };

    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    event_loop.run_app(&mut app_state);

    Ok(())
}

#[derive(Debug)]
struct AppState {
    render_state: Option<renderer::RenderState>,
    asset_manager: assets::AssetManager,
    registry_manager: registries::GameRegistries,
    game_state: game::GameState,
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
                // Check if time since last frame is enough to run a sim tick
                // TODO: Rum Simulation Tick
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
