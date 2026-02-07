use crate::config::EngineConfig;
use crate::graphics::camera::Camera;
use crate::graphics::storage::g2d::Graph2D;
use crate::graphics::storage::g3d::Graph3D;
use crate::graphics::timing::EngineTiming;
use crate::graphics::GraphicsIntermediary;
use crate::input::UserInput;
use crate::logger::log;
use crate::logger::log_level::LogLevel;
use std::sync::{Arc, Mutex};

pub struct RendererContext {
    /* scene for game statistics */
    pub first_frame_rendered: bool,
    pub frame_count: u128,
    
    /* timing */
    pub timing: EngineTiming,

    /* scene for world state */
    pub g2d: Graph2D,
    pub g3d: Graph3D,
    pub camera: Camera,
    
    /* rendering subsystem */
    pub(crate) graphics: GraphicsIntermediary,

    /* scene for input state */
    pub input: Arc<Mutex<UserInput>>,
    pub config: EngineConfig,
}

impl RendererContext {
    pub(crate) fn new(input: &Arc<Mutex<UserInput>>, config: EngineConfig) -> RendererContext {
        let dim = &input.lock().unwrap().current_client_dimensions.clone();
        log(LogLevel::Info, &|| String::from(format!("initializing camera with width={},height={}", &dim.width, &dim.height)));
        RendererContext {
            first_frame_rendered: false,
            frame_count: 0,
            
            timing: EngineTiming::new(&config.renderer),
            
            g2d: Graph2D::new(),
            g3d: Graph3D::new(),
            camera: Camera::new(dim),
            
            graphics: GraphicsIntermediary::new(config.renderer.graphics.clone()),

            input: input.clone(),
            config,
        }
    }
}
