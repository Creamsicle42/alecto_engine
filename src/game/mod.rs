use std::collections::VecDeque;

use crate::game::events::ScheduledEvent;

mod events;

// Struct for holding all active data about the current game, that being the level data, the menu
// data, and the save progression data
#[derive(Debug)]
pub struct GameState {
    lua_state: mlua::Lua,
    event_schedule: Vec<ScheduledEvent>,
}

impl GameState {
    pub fn new(lua_state: mlua::Lua) -> Self {
        GameState {
            lua_state,
            event_schedule: vec![],
        }
    }
}

pub fn game_state_update_tick(
    game_state: &mut GameState,
    asset_manager: &mut crate::assets::AssetManager,
    registries: &crate::registries::GameRegistries,
) {
    let mut event_queue: VecDeque<events::QueuedEvent> = VecDeque::new();
    // Append scheduled events to event queue
    let mut completed: Vec<usize> = vec![];
    let schedule_size = game_state.event_schedule.len();
    for (event, i) in game_state.event_schedule.iter_mut().zip(0..schedule_size) {
        if event.ticks_left == 0 {
            let new_event = std::mem::take(&mut event.event);
            event_queue.push_back(new_event);
            completed.push(i);
        } else {
            event.ticks_left -= 1;
        }
    }
    for remove_id in completed.iter().rev() {
        game_state.event_schedule.swap_remove(*remove_id);
    }
    // Modify and remove scheduled events

    // Push Input down menu stack, if not canceled then give it to any input thinkers
    // Process event queue untill queue is empty
    // Run physics simulation if there is a current world (events will queue from this)
    //  - Raycast Interaction
    //  - Character Movements
    //  - Level Changes
    // Process events queued from physics step
}
