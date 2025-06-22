use std::{cell::RefCell, rc::Rc};

use mlua::FromLua;

mod menu_registry;

// Data structure responsible for handling all registered data in the game, I.E. Enemy Types,
// Menus, Levels, Etc...
#[derive(Debug)]
pub struct GameRegistries {
    pub menu_registry: menu_registry::MenuRegistry,
}

impl GameRegistries {
    pub fn new(
        command_queue: RegistryQueue,
        asset_manager: &mut crate::assets::AssetManager,
    ) -> Self {
        // Create a safe lua state
        let lua = mlua::Lua::new();

        let mut menu_registry = menu_registry::MenuRegistry::default();

        log::info!("Begining Registry creation.");
        for cmd in command_queue.internal.take().commands.into_iter() {
            let _ = match cmd {
                RegistryCommand::RegisterMenu(menu_id) => {
                    menu_registry.register_menu(menu_id, asset_manager, &lua)
                }
            };
        }
        log::info!("Registry creation complete.");
        GameRegistries { menu_registry }
    }
}

// Command Queue for lua interfacing during startup, uses rc>refCell pattern to transfer data back
// out of lua state
#[derive(Debug, Default)]
pub struct RegistryQueue {
    internal: Rc<RefCell<RegistryQueueInternal>>,
}

#[derive(Debug)]
enum RegistryCommand {
    RegisterMenu(String),
}

impl Clone for RegistryQueue {
    fn clone(&self) -> Self {
        RegistryQueue {
            internal: Rc::clone(&self.internal),
        }
    }
}

#[derive(Debug, Default)]
struct RegistryQueueInternal {
    commands: Vec<RegistryCommand>,
}

impl mlua::UserData for RegistryQueue {
    // Methods for creating a usedata queue
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("register_menu", |_, mut this, menu_id: String| {
            this.internal
                .borrow_mut()
                .commands
                .push(RegistryCommand::RegisterMenu(menu_id));
            Ok(())
        });
    }
}
