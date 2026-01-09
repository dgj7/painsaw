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
        self.initialize_world_helper(context);

        let graphics = &mut context.graphics;
        graphics.initialize(&mut context.g2d, &mut context.g3d);

        log(LogLevel::Debug, &|| String::from("initialization complete"));
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
        /* gather variables */
        let graphics = &context.graphics;
        let ccd = &context.copy_client_dimensions();

        /* prepare for drawing */
        graphics.before_scene(ccd);

        /* draw 2d, if desired */
        graphics.prepare_2d(&mut context.g2d, ccd);
        graphics.render_2d(&context.g2d);
        graphics.after_2d();

        /* draw 3d, if desired */
        graphics.prepare_3d();
        graphics.render_3d(&context.g3d);
        graphics.after_3d();
    }
}
