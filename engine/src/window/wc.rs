//!
//! this file encapsulates the renderer, as it is viewed from the perspective of the client application.
//!
//! from this perspective, the "renderer" doesn't have implementation details relating to the specific
//! subsystem in use (for ex: opengl, directx, etc).
//!

use crate::logger::log;
use crate::logger::log_level::LogLevel;
use crate::window::context::RendererContext;
use crate::input::r#in::InputName;

///
/// Control various aspects of the world, as called by the windowing system.
///
pub trait WorldController {
    ///
    /// initialize the game world.
    ///
    fn initialize_world(&self, context: &mut RendererContext) {
        self.initialize_world_helper(context);

        let graphics = &mut context.graphics;
        graphics.initialize(&mut context.g2d, &mut context.g3d);

        log(LogLevel::Debug, &|| String::from("initialization complete"));
    }

    ///
    /// initialize game world - customizer for client.
    ///
    fn initialize_world_helper(&self, context: &mut RendererContext);

    ///
    /// update the game world state - fully controlled by client.
    ///
    fn update_world(&self, context: &mut RendererContext) {
        match context.input.clone().lock() {
            Ok(mut uin) => {
                /* handle key changes */
                while !uin.changes.is_empty() {
                    let change = uin.changes.pop_front().unwrap();
                    let state = uin.states.get_mut(&change).unwrap();
                    if !state.current.is_handled() {
                        match context.config.input.behaviors.get(&change) {
                            None => {}
                            Some(behavior) => behavior(context, state)
                        }
                        state.current.set_handled();
                    }
                }

                /* handle screen resize */
                if uin.screen_resized {
                    context.camera.update_screen(&uin.current_client_dimensions);
                    context.graphics.resize(context);
                }

                /* handle mouse moves */
                while !uin.mouse_moves.is_empty() {
                    let change = uin.mouse_moves.pop_front().unwrap();
                    match change {
                        InputName::MouseMove { x, y } => {log(LogLevel::Debug, &|| String::from(format!("mousemove: x={},y={}", x, y)))}
                        _ => {}
                    }
                }
            }
            Err(_) => {}
        }

        self.update_world_helper(context);

        match context.input.lock() {
            Ok(mut uin) => {
                uin.screen_resized = false;
            }
            Err(_) => {panic!("todo: resetting screen_resized")}
        }
    }

    fn update_world_helper(&self, context: &mut RendererContext);

    ///
    /// display the game world scene.
    ///
    /// fully controlled by engine; the engine is data-driven, meaning that graphics instructions
    /// come from models supplied during initialization, along with changes to those models
    /// during the update world step.
    ///
    fn display_world_scene(&self, context: &mut RendererContext) {
        /* prepare for drawing */
        context.graphics.before_scene(&context.camera);

        /* draw 2d, if desired */
        context.graphics.prepare_2d(&mut context.g2d, &context.camera);
        context.graphics.render_2d(&mut context.g2d, &context.timing, &context.config, &context.camera);
        context.graphics.after_2d();

        /* draw 3d, if desired */
        context.graphics.prepare_3d(&context);
        context.graphics.render_3d(&mut context.g3d);
        context.graphics.after_3d(&context);
    }
}
