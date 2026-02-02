use crate::config::EngineConfig;
use crate::graphics::camera::Camera;
use crate::graphics::subsystem::{grss_factory, GraphicsSubSystem, RenderingSubSystemHandle};
use crate::logger::log;
use crate::logger::log_level::LogLevel;
use crate::window::context::RendererContext;
use num_traits::Float;
use std::ops::{Add, Sub};
use storage::g2d::Graph2D;
use storage::g3d::Graph3D;
use subsystem::RendererInfo;
use crate::graphics::hud::coords::show_cam_coords;
use crate::graphics::hud::fps::show_fps;
use crate::graphics::timing::EngineTiming;

pub mod camera;
pub mod color;
pub mod geometry;
pub mod image;
pub mod storage;
pub mod subsystem;
pub(super) mod timing;
pub(crate) mod hud;

///
/// Graphics rendering intermediary.
///
/// This system separates/abstracts the concrete graphics rendering subsystem
/// from both the operating system and the geometry systems.
///
pub(crate) struct GraphicsIntermediary<F: Float + Add<F> + Sub<F>> {
    subsystem: Box<dyn RenderingSubSystemHandle<F>>,
    info: Option<RendererInfo>,
}

impl<F: Float + Add<F> + Sub<F>> GraphicsIntermediary<F> {
    pub(crate) fn new(grss: GraphicsSubSystem) -> GraphicsIntermediary<F> {
        GraphicsIntermediary {
            subsystem: grss_factory(grss),
            info: None,
        }
    }

    pub(crate) fn initialize(&mut self, g2d: &mut Graph2D<F>, g3d: &mut Graph3D<F>) {
        self.subsystem.initialize(g2d, g3d);
        self.info = self.subsystem.identify();

        log(LogLevel::Info, &|| String::from(format!("{:?}", self.info)));
        log(LogLevel::Debug, &|| String::from("initialization complete"));
    }

    pub(crate) fn resize(&self, context: &RendererContext<F>) {
        self.subsystem.resize(context);
    }

    pub(crate) fn before_scene(&mut self, camera: &Camera<F>) {
        self.subsystem.before_scene(camera);
    }

    pub(crate) fn prepare_2d(&self, g2d: &mut Graph2D<F>, camera: &Camera<F>) {
        self.subsystem.prepare_2d(camera, g2d);
    }

    pub(crate) fn render_2d(
        &mut self,
        g2d: &mut Graph2D<F>,
        timing: &EngineTiming,
        config: &EngineConfig<F>,
        camera: &Camera<F>,
    ) {
        /* render primitives */
        self.subsystem.render_2d(g2d);

        /* conditional display */
        show_fps(g2d, timing, config);
        show_cam_coords(g2d, config, camera, 1.0);
    }

    pub(crate) fn after_2d(&self) {
        self.subsystem.after_2d();
    }

    pub(crate) fn prepare_3d(&self, context: &RendererContext<F>) {
        self.subsystem.prepare_3d(context);
    }

    pub(crate) fn render_3d(&self, g3d: &mut Graph3D<F>) {
        self.subsystem.render_3d(g3d);
    }

    pub(crate) fn after_3d(&self, context: &RendererContext<F>) {
        self.subsystem.after_3d(context);
    }
}
