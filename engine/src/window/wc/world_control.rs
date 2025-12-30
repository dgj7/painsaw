//!
//! renderer methods which will be called by the windowing system.
//!
//! does NOT expose rendering subsystem methods; ie, opengl, directx, etc.
//!

use crate::window::wc::context::RendererContext;
use num_traits::Float;
use std::ops::{Add, Sub};
use crate::logger::log;
use crate::logger::log_level::LogLevel;

///
/// Control various aspects of the world, as called by the windowing system.
///
pub trait WorldController<F: Float + Add<F> + Sub<F>> {
    ///
    /// initialize the game world.
    ///
    fn initialize_world(&self, context: &mut RendererContext<f32>) {
        let ss = &context.subsystem;
        ss.initialize(context);
        self.initialize_world_helper(context);
        log(LogLevel::Info, &|| String::from("initialization complete"));
    }

    ///
    /// initialize game world - customizer for client.
    ///
    fn initialize_world_helper(&self, context: &mut RendererContext<f32>);

    ///
    /// update the game world state - fully controlled by client.
    ///
    fn update_world(&self, context: &mut RendererContext<f32>);

    ///
    /// display the game world scene.
    ///
    /// fully controlled by engine; the engine is data-driven, meaning that render instructions
    /// come from models supplied during initialization, along with changes to those models
    /// during the update world step.
    ///
    fn display_world_scene(&self, context: &mut RendererContext<f32>) {
        let ss = &context.subsystem;
        let ccd = &context.copy_client_dimensions();
        ss.before_scene(ccd);
        ss.render_2d(&context.g2d, ccd);
        ss.render_3d(&context.g3d);
    }
}
