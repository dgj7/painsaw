use crate::graphics::camera::Camera;
use crate::graphics::model::g2d::Graph2D;
use crate::graphics::model::g3d::Graph3D;
use crate::graphics::subsystem::GraphicsSubSystem;
use crate::graphics::GraphicsIntermediary;
use crate::input::InputState;
use num_traits::Float;
use std::ops::{Add, Sub};
use std::sync::{Arc, Mutex};
use crate::logger::log;
use crate::logger::log_level::LogLevel;

pub struct RendererContext<F: Float + Add<F> + Sub<F>> {
    /* scene for game statistics */
    pub first_frame_rendered: bool,
    pub frame_count: u128,

    /* scene for input state */
    pub input: Arc<Mutex<InputState<f32>>>,

    /* scene for world state */
    pub g2d: Graph2D<F>,
    pub g3d: Graph3D<F>,
    pub camera: Camera,
    
    /* rendering subsystem */
    pub(crate) graphics: GraphicsIntermediary<F>,
}

impl<F: Float + Add<F> + Sub<F>> RendererContext<F> {
    pub(crate) fn new(input: &Arc<Mutex<InputState<f32>>>, grss: GraphicsSubSystem) -> RendererContext<F> {
        let dim = &input.lock().unwrap().current_client_dimensions.clone();
        log(LogLevel::Info, &|| String::from(format!("initializing camera with width={},height={}", &dim.width, &dim.height)));
        RendererContext {
            first_frame_rendered: false,
            frame_count: 0,

            input: input.clone(),
            
            g2d: Graph2D::new(),
            g3d: Graph3D::new(),
            camera: Camera::new(dim),
            
            graphics: GraphicsIntermediary::new(grss),
        }
    }
}
