use crate::config::input_config::kc::KeyHandler;
use crate::config::input_config::mc::MouseHandler;
use crate::config::EngineConfig;
use crate::geometry::primitive::v2d::Vertex2D;
use crate::graphics::camera::Camera;
use crate::graphics::subsystem::{grss_factory, GraphicsSubSystem, RenderingSubSystemHandle};
use crate::input::mouse::min::MouseInputName;
use crate::input::screen::ScreenState;
use crate::input::UserInput;
use crate::support::logger::log;
use crate::support::logger::log_level::LogLevel;
use crate::support::stats::coords::show_cam_coords;
use crate::support::stats::fps::show_fps;
use crate::support::stats::screen::show_screen_stats;
use crate::support::timing::EngineTiming;
use crate::WorldController;
use std::sync::MutexGuard;
use storage::g2d::Graph2D;
use storage::g3d::Graph3D;
use subsystem::RendererInfo;
use crate::graphics::storage::gxd::Models;

pub mod camera;
pub mod color;
pub mod scenegraph;
pub mod storage;
pub mod subsystem;
pub mod texture;

///
/// Graphics rendering intermediary.
///
/// This system separates/abstracts the concrete graphics rendering subsystem
/// from both the operating system and the geometry systems.
///
pub struct RendererWrapper {
    subsystem: Box<dyn RenderingSubSystemHandle>,
    info: Option<RendererInfo>,
}

impl RendererWrapper {
    pub(crate) fn new(grss: GraphicsSubSystem) -> RendererWrapper {
        RendererWrapper {
            subsystem: grss_factory(grss),
            info: None,
        }
    }

    pub(crate) fn initialize(&mut self, models: &mut Models) {
        self.subsystem.initialize(models);
        self.info = self.subsystem.identify();

        log(LogLevel::Info, &|| String::from(format!("{:?}", self.info)));
        log(LogLevel::Debug, &|| String::from("initialization complete"));
    }

    pub(crate) fn resize(&self, camera: &Camera) {
        self.subsystem.resize(camera);
    }

    pub(crate) fn before_scene(&mut self, camera: &Camera) {
        self.subsystem.before_scene(camera);
    }

    pub(crate) fn prepare_2d(&self, camera: &Camera, g2d: &mut Graph2D) {
        self.subsystem.prepare_2d(camera, g2d);
    }

    pub(crate) fn render_2d<T: KeyHandler + MouseHandler + WorldController + 'static>(
        &mut self,
        config: &EngineConfig,
        input: MutexGuard<UserInput>,
        screen: &ScreenState,
        camera: &Camera,
        timing: &EngineTiming,
        g2d: &mut Graph2D,
    ) {
        /* track down the mouse position */
        let mouse_pos = input
            .mouse_states
            .get(&MouseInputName::MouseMove)
            .map(|ms| (ms.current.x as f32, ms.current.y as f32))
            .or_else(|| Some((0.0, 0.0)))
            .map(|(x, y)| Vertex2D::new(x, y))
            .unwrap();

        /* render primitives */
        self.subsystem.render_2d(g2d);

        /* conditional display */
        show_fps(g2d, timing, config);
        show_cam_coords(g2d, config, camera);
        show_screen_stats(g2d, config, &screen, &mouse_pos);
    }

    pub(crate) fn after_2d(&self) {
        self.subsystem.after_2d();
    }

    pub(crate) fn prepare_3d(&self, camera: &Camera) {
        self.subsystem.prepare_3d(camera);
    }

    pub(crate) fn render_3d(&self, g3d: &mut Graph3D) {
        self.subsystem.render_3d(g3d);
    }

    pub(crate) fn after_3d(&self) {
        self.subsystem.after_3d();
    }
}
