use std::collections::HashMap;
use crate::command::Command;
use crate::input::r#in::InputName;

pub struct CommandConfig {
    pub commands: HashMap<InputName, Box<dyn Command>>,
}

impl Default for CommandConfig {
    fn default() -> CommandConfig {
        CommandConfig {
            commands: HashMap::new(),
        }
    }
}
