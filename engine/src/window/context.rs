use crate::config::EngineConfig;
use crate::graphics::camera::Camera;
use crate::graphics::storage::g2d::Graph2D;
use crate::graphics::storage::g3d::Graph3D;
use crate::graphics::GraphicsIntermediary;
use crate::input::InputState;
use crate::logger::log;
use crate::logger::log_level::LogLevel;
use num_traits::Float;
use std::ops::{Add, Sub};
use std::sync::{Arc, Mutex};
use std::time::Instant;

pub struct RendererContext<F: Float + Add<F> + Sub<F>> {
    /* scene for game statistics */
    pub first_frame_rendered: bool,
    pub frame_count: u128,
    
    /* timing */
    pub(crate) last_frame: Instant,
    pub delta_time: f64,

    /* scene for world state */
    pub g2d: Graph2D<F>,
    pub g3d: Graph3D<F>,
    pub camera: Camera<F>,
    
    /* rendering subsystem */
    pub(crate) graphics: GraphicsIntermediary<F>,

    /* scene for input state */
    pub input: Arc<Mutex<InputState<f32>>>,
    pub config: EngineConfig<F>,
}

impl<F: Float + Add<F> + Sub<F>> RendererContext<F> {
    pub(crate) fn new(input: &Arc<Mutex<InputState<f32>>>, config: EngineConfig<F>) -> RendererContext<F> {
        let dim = &input.lock().unwrap().current_client_dimensions.clone();
        log(LogLevel::Info, &|| String::from(format!("initializing camera with width={},height={}", &dim.width, &dim.height)));
        RendererContext {
            first_frame_rendered: false,
            frame_count: 0,
            
            last_frame: Instant::now(),
            delta_time: 0.0,
            
            g2d: Graph2D::new(),
            g3d: Graph3D::new(),
            camera: Camera::new(dim),
            
            graphics: GraphicsIntermediary::new(config.renderer.graphics.clone()),

            input: input.clone(),
            config,
        }
    }
}
