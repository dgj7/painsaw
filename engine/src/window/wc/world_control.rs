//!
//! renderer methods which will be called by the windowing system.
//!
//! does NOT expose rendering subsystem methods; ie, opengl, directx, etc.
//!

use crate::logger::log;
use crate::logger::log_level::LogLevel;
use crate::window::wc::context::RendererContext;
use num_traits::Float;
use std::ops::{Add, Sub};

///
/// Control various aspects of the world, as called by the windowing system.
///
pub trait WorldController<F: Float + Add<F> + Sub<F>> {
    ///
    /// initialize the game world.
    ///
    fn initialize_world(&self, context: &mut RendererContext<F>) {
        let graphics = &context.graphics;
        graphics.initialize();
        
        self.initialize_world_helper(context);
        
        log(LogLevel::Info, &|| String::from("initialization complete"));
    }

    ///
    /// initialize game world - customizer for client.
    ///
    fn initialize_world_helper(&self, context: &mut RendererContext<F>);

    ///
    /// update the game world state - fully controlled by client.
    ///
    fn update_world(&self, context: &mut RendererContext<F>);

    ///
    /// display the game world scene.
    ///
    /// fully controlled by engine; the engine is data-driven, meaning that graphics instructions
    /// come from models supplied during initialization, along with changes to those models
    /// during the update world step.
    ///
    fn display_world_scene(&self, context: &mut RendererContext<F>) {
        let graphics = &context.graphics;
        let ccd = &context.copy_client_dimensions();
        graphics.before_scene(ccd);
        graphics.prepare_2d(ccd);
        graphics.render_2d(&context.g2d);
        graphics.prepare_3d();
        graphics.render_3d(&context.g3d);
    }
}
