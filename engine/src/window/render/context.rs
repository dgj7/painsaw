use std::ops::{Add, Sub};
use crate::geometry::dim::d2d::Dimension2D;
use crate::geometry::storage::g2d::Graph2D;
use crate::geometry::storage::g3d::Graph3D;
use crate::input::model::input_state::InputState;
use crate::render::handle::def::RenderingSubSystemHandle;
use crate::render::handle::info::RendererInfo;
use std::sync::{Arc, Mutex};
use num_traits::Float;

pub struct RendererContext<F: Float + Add<F> + Sub<F>> {
    /* storage for game statistics */
    pub first_frame_rendered: bool,
    pub frame_count: u128,
    pub info: Option<RendererInfo>,

    /* storage for input state */
    pub input: Arc<Mutex<InputState<f32>>>,

    /* storage for world state */
    pub g2d: Graph2D<F>,
    pub g3d: Graph3D<F>,
    
    /* rendering subsystem */
    pub subsystem: Box<dyn RenderingSubSystemHandle<F>>,
}

impl<F: Float + Add<F> + Sub<F>> RendererContext<F> {
    pub(crate) fn new(input: &Arc<Mutex<InputState<f32>>>, grss: Box<dyn RenderingSubSystemHandle<F>>) -> RendererContext<F> {
        RendererContext {
            first_frame_rendered: false,
            frame_count: 0,
            info: None,

            input: input.clone(),
            
            g2d: Graph2D::new(),
            g3d: Graph3D::new(),
            
            subsystem: grss,
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
