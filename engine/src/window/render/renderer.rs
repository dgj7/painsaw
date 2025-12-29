//!
//! renderer methods which will be called by the windowing system.
//!
//! does NOT expose rendering subsystem methods; ie, opengl, directx, etc.
//!

use std::ops::{Add, Sub};
use num_traits::Float;
use crate::window::render::context::RendererContext;

// todo: this needs a rename; it's not a renderer, it's more of a client controller
pub trait Renderer<F: Float + Add<F> + Sub<F>> {
    fn initialize(&self, context: &mut RendererContext<f32>);

    fn update_world(&self, context: &mut RendererContext<f32>);

    fn render_scene(&self, context: &mut RendererContext<f32>) {
        let ss = &context.subsystem;
        let ccd = &context.copy_client_dimensions();
        ss.before_scene(ccd);
        ss.render_2d(&context.g2d, ccd);
        ss.render_3d(&context.g3d);
    }
}
