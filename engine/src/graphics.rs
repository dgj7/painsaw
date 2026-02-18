use crate::config::EngineConfig;
use crate::graphics::camera::Camera;
use crate::support::hud::coords::show_cam_coords;
use crate::support::hud::fps::show_fps;
use crate::graphics::subsystem::{grss_factory, GraphicsSubSystem, RenderingSubSystemHandle};
use crate::support::timing::EngineTiming;
use crate::support::logger::log;
use crate::support::logger::log_level::LogLevel;
use crate::window::context::RendererContext;
use storage::g2d::Graph2D;
use storage::g3d::Graph3D;
use subsystem::RendererInfo;

pub mod camera;
pub mod color;
pub mod image;
pub mod storage;
pub mod subsystem;

///
/// Graphics rendering intermediary.
///
/// This system separates/abstracts the concrete graphics rendering subsystem
/// from both the operating system and the geometry systems.
///
pub(crate) struct GraphicsIntermediary {
    subsystem: Box<dyn RenderingSubSystemHandle>,
    info: Option<RendererInfo>,
}

impl GraphicsIntermediary {
    pub(crate) fn new(grss: GraphicsSubSystem) -> GraphicsIntermediary {
        GraphicsIntermediary {
            subsystem: grss_factory(grss),
            info: None,
        }
    }

    pub(crate) fn initialize(&mut self, g2d: &mut Graph2D, g3d: &mut Graph3D) {
        self.subsystem.initialize(g2d, g3d);
        self.info = self.subsystem.identify();

        log(LogLevel::Info, &|| String::from(format!("{:?}", self.info)));
        log(LogLevel::Debug, &|| String::from("initialization complete"));
    }

    pub(crate) fn resize(&self, context: &RendererContext) {
        self.subsystem.resize(context);
    }

    pub(crate) fn before_scene(&mut self, camera: &Camera) {
        self.subsystem.before_scene(camera);
    }

    pub(crate) fn prepare_2d(&self, g2d: &mut Graph2D, camera: &Camera) {
        self.subsystem.prepare_2d(camera, g2d);
    }

    pub(crate) fn render_2d(&mut self, g2d: &mut Graph2D, timing: &EngineTiming, config: &EngineConfig, camera: &Camera) {
        /* render primitives */
        self.subsystem.render_2d(g2d);

        /* conditional display */
        show_fps(g2d, timing, config);
        show_cam_coords(g2d, config, camera, 1.0);
    }

    pub(crate) fn after_2d(&self) {
        self.subsystem.after_2d();
    }

    pub(crate) fn prepare_3d(&self, context: &RendererContext) {
        self.subsystem.prepare_3d(context);
    }

    pub(crate) fn render_3d(&self, g3d: &mut Graph3D) {
        self.subsystem.render_3d(g3d);
    }

    pub(crate) fn after_3d(&self, context: &RendererContext) {
        self.subsystem.after_3d(context);
    }
}
