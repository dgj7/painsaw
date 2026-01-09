use crate::geometry::dim::d2d::Dimension2D;
use crate::geometry::storage::g2d::Graph2D;
use crate::geometry::storage::g3d::Graph3D;
use crate::graphics::subsystem::{grss_factory, GraphicsSubSystem, RenderingSubSystemHandle};
use num_traits::Float;
use std::ops::{Add, Sub};
use crate::graphics::model::renderer_info::RendererInfo;
use crate::logger::log;
use crate::logger::log_level::LogLevel;

pub mod model;
pub mod subsystem;

///
/// Graphics rendering intermediary.
///
/// This system separates/abstracts the concrete graphics rendering subsystem
/// from both the operating system and the geometry systems.
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

    pub(crate) fn before_scene(&self, ccd: &Dimension2D<f32>) {
        self.subsystem.before_scene(ccd);
    }

    pub(crate) fn prepare_2d(&self, g2d: &mut Graph2D<F>, ccd: &Dimension2D<f32>) {
        /* prepare the screen for 2d */
        self.subsystem.prepare_2d(ccd);

        /* initialize un-initialized (new) textures */
        for (_, model) in &mut g2d.models {
            for texture in &mut model.textures {
                if !texture.initialized {
                    self.subsystem.initialize_texture_2d(texture);
                }
            }
        }
    }

    pub(crate) fn render_2d(&self, g2d: &Graph2D<F>) {
        for (_, model) in g2d.models.iter() {
            model.lines.iter().for_each(|x| self.subsystem.render_2d_lines(x));
            model.points.iter().for_each(|x| self.subsystem.render_2d_points(x));
            model.textures.iter()
                .filter(|x| x.initialized)
                .for_each(|x| self.subsystem.render_2d_textures(x));
        }
    }

    pub(crate) fn after_2d(&self) {
        self.subsystem.after_2d();
    }

    pub(crate) fn prepare_3d(&self) {
        self.subsystem.prepare_3d();
    }

    pub(crate) fn render_3d(&self, g3d: &Graph3D<F>) {
        for (_, model) in g3d.models.iter() {
            model.lines.iter().for_each(|x| self.subsystem.render_3d_lines(x));
            model.points.iter().for_each(|x| self.subsystem.render_3d_points(x));
        }
    }

    pub(crate) fn after_3d(&self) {
        self.subsystem.after_3d();
    }
}
