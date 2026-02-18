use std::collections::HashMap;
use std::sync::Arc;
use crate::input::is::InputState;
use crate::input::kin::KeyInputName;
use crate::support::logger::log;
use crate::support::logger::log_level::LogLevel;
use crate::window::context::RendererContext;

///
/// handle keys via the configured key handler.
///
pub(crate) fn handle_key_change(handler: Arc<dyn KeyHandler>, name: &KeyInputName, state: &mut InputState, context: &mut RendererContext) {
    match name {
        KeyInputName::KeyEscape => handler.handle_escape_key_change(name, state, context),
        KeyInputName::KeyA => handler.handle_a_key_change(name, state, context),
        KeyInputName::KeyD => handler.handle_d_key_change(name, state, context),
        KeyInputName::KeyG => handler.handle_g_key_change(name, state, context),
        KeyInputName::KeyM => handler.handle_m_key_change(name, state, context),
        KeyInputName::KeyS => handler.handle_s_key_change(name, state, context),
        KeyInputName::KeyW => handler.handle_w_key_change(name, state, context),
    }
}

///
/// core key handler trait.
///
pub trait KeyHandler {
    ///
    /// check key states.
    ///
    /// this is useful for handling scenarios where holding a key down might not be a
    /// "new" change, but still needs to be handled as input for some games.
    ///
    fn check_key_states(&self, states: &HashMap<KeyInputName, InputState>, context: &mut RendererContext) {
        log(LogLevel::Trace, &|| String::from(format!("{:?}/{:?}", states, context.frame_count)));
    }

    fn handle_escape_key_change(&self, name: &KeyInputName, state: &mut InputState, context: &mut RendererContext) {
        log(LogLevel::Trace, &|| String::from(format!("{:?}({:?},{:?}) handler unused", name, state, context.frame_count)));
    }

    fn handle_a_key_change(&self, name: &KeyInputName, state: &mut InputState, context: &mut RendererContext) {
        log(LogLevel::Trace, &|| String::from(format!("{:?}({:?},{:?}) handler unused", name, state, context.frame_count)));
    }

    fn handle_d_key_change(&self, name: &KeyInputName, state: &mut InputState, context: &mut RendererContext) {
        log(LogLevel::Trace, &|| String::from(format!("{:?}({:?},{:?}) handler unused", name, state, context.frame_count)));
    }

    fn handle_g_key_change(&self, name: &KeyInputName, state: &mut InputState, context: &mut RendererContext) {
        log(LogLevel::Trace, &|| String::from(format!("{:?}({:?},{:?}) handler unused", name, state, context.frame_count)));
    }

    fn handle_m_key_change(&self, name: &KeyInputName, state: &mut InputState, context: &mut RendererContext) {
        log(LogLevel::Trace, &|| String::from(format!("{:?}({:?},{:?}) handler unused", name, state, context.frame_count)));
    }

    fn handle_s_key_change(&self, name: &KeyInputName, state: &mut InputState, context: &mut RendererContext) {
        log(LogLevel::Trace, &|| String::from(format!("{:?}({:?},{:?}) handler unused", name, state, context.frame_count)));
    }

    fn handle_w_key_change(&self, name: &KeyInputName, state: &mut InputState, context: &mut RendererContext) {
        log(LogLevel::Trace, &|| String::from(format!("{:?}({:?},{:?}) handler unused", name, state, context.frame_count)));
    }
}

///
/// default key handler struct; no fields necessary.
///
pub struct DefaultKeyHandler {}

///
/// identify default key handler as a key handler.
///
impl KeyHandler for DefaultKeyHandler {}
