use std::cmp;
use model::g2d::Graph2D;
use model::g3d::Graph3D;
use model::m2d::Model2D;
use crate::geometry::point::p2d::Point2D;
use crate::graphics::model::color::Color;
use crate::graphics::model::renderer_info::RendererInfo;
use crate::graphics::subsystem::{grss_factory, GraphicsSubSystem, RenderingSubSystemHandle};
use crate::logger::log;
use crate::logger::log_level::LogLevel;
use crate::image::text::{text_2d_image, TextConfig};
use num_traits::Float;
use std::ops::{Add, Sub};
use std::time::Instant;
use crate::graphics::camera::Camera;
use crate::image::t2d::Texture2D;
use crate::window::context::RendererContext;

pub mod model;
pub mod subsystem;
pub mod camera;

///
/// Graphics rendering intermediary.
///
/// This system separates/abstracts the concrete graphics rendering subsystem
/// from both the operating system and the geometry systems.
pub(crate) struct GraphicsIntermediary<F: Float + Add<F> + Sub<F>> {
    subsystem: Box<dyn RenderingSubSystemHandle<F>>,
    info: Option<RendererInfo>,

    pub(crate) last_frame: Instant,
    pub(crate) fps_enabled: bool,
}

impl<F: Float + Add<F> + Sub<F>> GraphicsIntermediary<F> {
    pub(crate) fn new(grss: GraphicsSubSystem) -> GraphicsIntermediary<F> {
        GraphicsIntermediary {
            subsystem: grss_factory(grss),
            info: None,
            last_frame: Instant::now(),
            fps_enabled: true,
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

    pub(crate) fn before_scene(&self, camera: &Camera) {
        self.subsystem.before_scene(camera);
    }

    pub(crate) fn prepare_2d(&self, g2d: &mut Graph2D<F>, camera: &Camera) {
        /* prepare the screen for 2d */
        self.subsystem.prepare_2d(camera);

        /* initialize un-initialized (new) textures */
        for (_, model) in &mut g2d.models {
            for texture in &mut model.textures {
                if !texture.initialized {
                    self.subsystem.initialize_texture_2d(texture);
                }
                self.subsystem.update_texture_2d(texture);
            }
        }
    }

    pub(crate) fn render_2d(&mut self, g2d: &mut Graph2D<F>) {
        for (_, model) in g2d.models.iter() {
            model.lines.iter().for_each(|x| self.subsystem.render_2d_lines(x));
            model.points.iter().for_each(|x| self.subsystem.render_2d_points(x));
            model.textures.iter()
                .filter(|x| x.initialized)
                .for_each(|x| self.subsystem.render_2d_textures(x));
        }

        if self.fps_enabled {
            /* calculate fps */
            let next = Instant::now();
            let duration_seconds = next.duration_since(self.last_frame).as_secs_f64();
            self.last_frame = next;
            let fps_float = 1.0 / duration_seconds;
            let fps = cmp::min(fps_float as u16, 999);

            /* prepare to render text */
            let config = TextConfig {
                top_left: Point2D { x: 100.0, y: 100.0 },
                foreground: Color::RED,
                ..Default::default()
            };

            /* add or update models */
            g2d.models
                .entry("99-builtin-fps".parse().unwrap())
                .and_modify(|m| m.textures[0].replacement = Option::from(text_2d_image(config.clone(), || String::from(format!("FPS:{:3}", fps)))))
                .or_insert(Model2D::new(vec!(), vec!(),
                        vec!(Texture2D::new(
                                text_2d_image(config.clone(), || String::from(format!("FPS:{:3}", fps))),
                                Point2D::new(F::from(1700.0).unwrap(), F::from(20.0).unwrap()),
                                F::from(2).unwrap()))));
        }
    }

    pub(crate) fn after_2d(&self) {
        self.subsystem.after_2d();
    }

    pub(crate) fn prepare_3d(&self, context: &RendererContext<F>) {
        self.subsystem.prepare_3d(context);
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
