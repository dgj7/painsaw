use std::collections::HashMap;
use crate::command::Command;
use crate::input::r#in::InputName;

#[allow(unused)]// todo: remove this
pub struct CommandConfig {
    pub commands: HashMap<InputName, Box<dyn Command>>,
}
