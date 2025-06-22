use std::collections::HashMap;

use eyre::{Result, eyre};
#[derive(Debug, Default)]
pub struct MenuRegistry {
    menus: Vec<MenuDefinition>,
    menu_ids: HashMap<Box<str>, usize>,
}

#[derive(Debug)]
pub struct MenuDefinition {
    pub init_function: mlua::Function,
    pub process_function: mlua::Function,
    pub input_function: mlua::Function,
    pub draw_function: mlua::Function,
}

impl MenuRegistry {
    pub fn register_menu(
        &mut self,
        menu_id: String,
        asset_manager: &mut crate::assets::AssetManager,
        lua_state: &mlua::Lua,
    ) -> Result<()> {
        log::info!("Registering menu \"{}\".", menu_id);

        let menu_resource_id = format!("menus/{}.lua", menu_id).to_owned();
        let menu_data = match asset_manager.get_file_raw(menu_resource_id) {
            None => {
                log::error!("Menu lua definition file does not exist.");
                return Err(eyre!("Menu lua definition not found."));
            }
            Some(d) => d,
        };

        let menu_table: mlua::Table = match lua_state.load(menu_data).eval() {
            Ok(t) => t,
            Err(e) => {
                log::error!("Lua execution error: {}", e);
                return Err(eyre!(e.to_string()));
            }
        };

        let init_function: mlua::Function = match menu_table.get("init_menu") {
            Ok(f) => f,
            Err(e) => {
                log::error!("Menu definition table lacks \"init_menu\" function");
                return Err(eyre!("Menu lacks init function"));
            }
        };

        let process_function: mlua::Function = match menu_table.get("process_tick") {
            Ok(f) => f,
            Err(e) => {
                log::error!("Menu definition table lacks \"process_tick\" function");
                return Err(eyre!("Menu lacks process function"));
            }
        };
        let input_function: mlua::Function = match menu_table.get("process_input") {
            Ok(f) => f,
            Err(e) => {
                log::error!("Menu definition table lacks \"process_input\" function");
                return Err(eyre!("Menu lacks input function"));
            }
        };
        let draw_function: mlua::Function = match menu_table.get("draw_menu") {
            Ok(f) => f,
            Err(e) => {
                log::error!("Menu definition table lacks \"draw_menu\" function");
                return Err(eyre!("Menu lacks draw function"));
            }
        };

        let menu_def = MenuDefinition {
            init_function,
            process_function,
            input_function,
            draw_function,
        };

        let menu_id = menu_id.into_boxed_str();
        // If menu with this id is already created, overwrite it
        match self.menu_ids.get(&menu_id) {
            None => {
                self.menu_ids.insert(menu_id, self.menus.len());
                self.menus.push(menu_def);
            }
            Some(id) => {
                if let Some(menu) = self.menus.get_mut(*id) {
                    *menu = menu_def;
                    log::info!("Overwriting old menu definition.");
                }
            }
        }

        Ok(())
    }

    pub fn get_menu_index(&self, menu_id: Box<str>) -> Option<usize> {
        self.menu_ids.get(&menu_id).copied()
    }

    pub fn get_menu_reference(&self, menu_index: usize) -> Option<&MenuDefinition> {
        self.menus.get(menu_index)
    }
}
