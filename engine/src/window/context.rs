use crate::config::EngineConfig;
use crate::graphics::camera::Camera;
use crate::graphics::model::g2d::Graph2D;
use crate::graphics::model::g3d::Graph3D;
use crate::graphics::GraphicsIntermediary;
use crate::input::InputState;
use crate::logger::log;
use crate::logger::log_level::LogLevel;
use num_traits::Float;
use std::collections::BTreeMap;
use std::ops::{Add, Sub};
use std::sync::{Arc, Mutex};

/* cvar keys */
pub static CVAR_FOV: &str = "cvar-fov";

/* cvar defaults */
pub static DEFAULT_FOV: f64 = 45.0;

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

    /* configurations */
    cvars: BTreeMap<String, String>,
    pub config: EngineConfig<F>,
}

impl<F: Float> RendererContext<F> {
    pub fn get_cvar<FN, O>(&self, name: &str, f: FN) -> Option<O>
    where
        FN: Fn(&str) -> O,
    {
        let maybe = self.cvars.get(name);
        maybe.map(|x| f(x))
    }
}

impl<F: Float + Add<F> + Sub<F>> RendererContext<F> {
    pub(crate) fn new(input: &Arc<Mutex<InputState<f32>>>, config: EngineConfig<F>) -> RendererContext<F> {
        let dim = &input.lock().unwrap().current_client_dimensions.clone();
        log(LogLevel::Info, &|| String::from(format!("initializing camera with width={},height={}", &dim.width, &dim.height)));
        RendererContext {
            first_frame_rendered: false,
            frame_count: 0,

            input: input.clone(),
            
            g2d: Graph2D::new(),
            g3d: Graph3D::new(),
            camera: Camera::new(dim),
            
            graphics: GraphicsIntermediary::new(config.renderer.graphics.clone()),

            cvars: BTreeMap::from([
                (CVAR_FOV.to_string(), DEFAULT_FOV.to_string()),
            ]),
            config,
        }
    }
}
