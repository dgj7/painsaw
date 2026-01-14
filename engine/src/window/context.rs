use crate::geometry::dim::d2d::Dimension2D;
use crate::geometry::scene::g2d::Graph2D;
use crate::geometry::scene::g3d::Graph3D;
use crate::graphics::subsystem::GraphicsSubSystem;
use crate::graphics::GraphicsIntermediary;
use num_traits::Float;
use std::ops::{Add, Sub};
use std::sync::{Arc, Mutex};
use crate::input::InputState;

pub struct RendererContext<F: Float + Add<F> + Sub<F>> {
    /* scene for game statistics */
    pub first_frame_rendered: bool,
    pub frame_count: u128,

    /* scene for input state */
    pub input: Arc<Mutex<InputState<f32>>>,

    /* scene for world state */
    pub g2d: Graph2D<F>,
    pub g3d: Graph3D<F>,
    
    /* rendering subsystem */
    pub(crate) graphics: GraphicsIntermediary<F>,
}

impl<F: Float + Add<F> + Sub<F>> RendererContext<F> {
    pub(crate) fn new(input: &Arc<Mutex<InputState<f32>>>, grss: GraphicsSubSystem) -> RendererContext<F> {
        RendererContext {
            first_frame_rendered: false,
            frame_count: 0,

            input: input.clone(),
            
            g2d: Graph2D::new(),
            g3d: Graph3D::new(),
            
            graphics: GraphicsIntermediary::new(grss),
        }
    }

    pub fn copy_client_dimensions(&self) -> Dimension2D<f32> {
        self.input
            .lock()
            .expect("retrieve_client_dimensions(): can't lock")
            .current_client_dimensions
            .clone()
    }
}
