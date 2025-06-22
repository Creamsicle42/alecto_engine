use std::default;

#[derive(Debug, Default)]
pub enum QueuedEvent {
    #[default]
    None,
    /// Run start.lua script
    RunStartScript,
    /// Push menu
    PushMenu(Box<str>, mlua::Table),
    /// Pop top menu from stack
    PopMenu,
    /// Run script scripts/script_id.lua
    RunScript(Box<str>),
}

#[derive(Debug)]
pub struct ScheduledEvent {
    pub ticks_left: usize,
    pub event: QueuedEvent,
}
