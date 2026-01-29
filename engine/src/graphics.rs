use crate::config::EngineConfig;
use crate::graphics::camera::Camera;
use color::Color;
use subsystem::RendererInfo;
use crate::graphics::subsystem::{grss_factory, GraphicsSubSystem, RenderingSubSystemHandle};
use crate::logger::log;
use crate::logger::log_level::LogLevel;
use crate::window::context::RendererContext;
use geometry::primitive::point::p2d::Point2D;
use image::t2d::Texture2D;
use image::text::{text_2d_image, TextConfig};
use storage::g2d::Graph2D;
use storage::g3d::Graph3D;
use storage::m2d::Model2D;
use num_traits::Float;
use std::cmp;
use std::ops::{Add, Sub};

pub mod camera;
pub mod geometry;
pub mod image;
pub mod storage;
pub mod subsystem;
pub mod color;

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

    pub(crate) fn render_2d(
        &mut self,
        g2d: &mut Graph2D<F>,
        delta_time: f64,
        config: &EngineConfig<F>,
        camera: &Camera<F>,
    ) {
        for (_, model) in g2d.models.iter() {
            model
                .lines
                .iter()
                .for_each(|x| self.subsystem.render_2d_lines(x));
            model
                .points
                .iter()
                .for_each(|x| self.subsystem.render_2d_points(x));
            model
                .textures
                .iter()
                .filter(|x| x.initialized)
                .for_each(|x| self.subsystem.render_2d_textures(x));
        }
        
        /* do rendering */
        self.subsystem.render_2d(g2d, delta_time, config, camera);

        /* conditional display */
        show_fps(g2d, delta_time, config);
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

fn show_fps<F: Float>(g2d: &mut Graph2D<F>, delta_time: f64, config: &EngineConfig<F>) {
    /* nothing to do if not enabled */
    if !config.renderer.show_fps {
        return;
    }

    /* calculate fps */
    let fps_float = 1.0 / delta_time;
    let fps = cmp::min(fps_float as u16, 9999);

    /* prepare to render text */
    let config = TextConfig {
        top_left: Point2D { x: 100.0, y: 100.0 },
        foreground: Color::RED,
        ..Default::default()
    };

    /* add or update models */
    g2d.models
        .entry("99-builtin-fps".parse().unwrap())
        .and_modify(|m| {
            m.textures[0].replacement = Option::from(text_2d_image(config.clone(), || {
                String::from(format!("FPS:{:4}", fps))
            }))
        })
        .or_insert(Model2D::new(
            vec![],
            vec![],
            vec![Texture2D::new(
                text_2d_image(config.clone(), || String::from(format!("FPS:{:4}", fps))),
                Point2D::new(F::from(10.0).unwrap(), F::from(100.0).unwrap()),
                F::from(1.0).unwrap(),
            )],
        ));
}

fn show_cam_coords<F: Float>(
    g2d: &mut Graph2D<F>,
    config: &EngineConfig<F>,
    camera: &Camera<F>,
    scale: f32,
) {
    /* nothing to do if not enabled */
    if !config.renderer.show_cam_coords {
        return;
    }

    /* get all the camera info */
    let position = camera.orientation.position.column_major_position();
    let forward = camera.orientation.position.column_major_z_forward();
    let right = camera.orientation.position.column_major_x_right();
    let up = camera.orientation.position.column_major_y_up();

    /* positioning variables */
    let height = 13.7 * scale;
    let x = F::from(10.0).unwrap();
    let y = 5.0;
    let y_cam = F::from(y).unwrap();
    let y_forward = F::from(y + height).unwrap();
    let y_right = F::from(y + height + height).unwrap();
    let y_up = F::from(y + height + height + height).unwrap();

    /* determine how the text should be displayed */
    let config = TextConfig {
        top_left: Point2D { x: 20.0, y: 20.0 },
        foreground: Color::RED,
        scale,
        ..Default::default()
    };

    /* update models */
    g2d.models
        .entry("6-2d-text-cam-pos".parse().unwrap())
        .and_modify(|m| {
            m.textures[0].replacement = Option::from(text_2d_image(config.clone(), || {
                String::from(format!(
                    "cam-pos: ({:+08.2},{:+08.2},{:+08.2})",
                    position.x.to_f32().unwrap(),
                    position.y.to_f32().unwrap(),
                    position.z.to_f32().unwrap(),
                ))
            }))
        })
        .or_insert(Model2D::new(
            vec![],
            vec![],
            vec![Texture2D::new(
                text_2d_image(config.clone(), || {
                    String::from(format!(
                        "cam-pos: ({:+08.2},{:+08.2},{:+08.2})",
                        position.x.to_f32().unwrap(),
                        position.y.to_f32().unwrap(),
                        position.z.to_f32().unwrap(),
                    ))}),
                Point2D::new(x, y_cam),
                F::from(1.0).unwrap(),
            )],));
    g2d.models
        .entry("6-2d-text-forward".parse().unwrap())
        .and_modify(|m| {
            m.textures[0].replacement = Option::from(text_2d_image(config.clone(), || {
                String::from(format!(
                    "forward: ({:+.2},{:+.2},{:+.2})",
                    forward.x.to_f32().unwrap(),
                    forward.y.to_f32().unwrap(),
                    forward.z.to_f32().unwrap(),
                ))}))
        })
        .or_insert(Model2D::new(
            vec![],
            vec![],
            vec![Texture2D::new(
                text_2d_image(config.clone(), || {
                    String::from(format!(
                        "forward: ({:+.2},{:+.2},{:+.2})",
                        forward.x.to_f32().unwrap(),
                        forward.y.to_f32().unwrap(),
                        forward.z.to_f32().unwrap(),
                    ))}),
                Point2D::new(x, y_forward),
                F::from(1.0).unwrap(),
            )],
        ));
    g2d.models
        .entry("6-2d-text-right".parse().unwrap())
        .and_modify(|m| {
            m.textures[0].replacement = Option::from(text_2d_image(config.clone(), || {
                String::from(format!(
                    "right:   ({:+.2},{:+.2},{:+.2})",
                    right.x.to_f32().unwrap(),
                    right.y.to_f32().unwrap(),
                    right.z.to_f32().unwrap(),
                ))}))
        })
        .or_insert(Model2D::new(
            vec![],
            vec![],
            vec![Texture2D::new(
                text_2d_image(config.clone(), || {
                    String::from(format!(
                        "right:   ({:+.2},{:+.2},{:+.2})",
                        right.x.to_f32().unwrap(),
                        right.y.to_f32().unwrap(),
                        right.z.to_f32().unwrap(),
                    ))}),
                Point2D::new(x, y_right),
                F::from(1.0).unwrap(),
            )],
        ));
    g2d.models
        .entry("6-2d-text-up".parse().unwrap())
        .and_modify(|m| {
            m.textures[0].replacement = Option::from(text_2d_image(config.clone(), || {
                String::from(format!("up:      ({:+.2},{:+.2},{:+.2})",
                                     up.x.to_f32().unwrap(),
                                     up.y.to_f32().unwrap(),
                                     up.z.to_f32().unwrap(),
                ))}))
        })
        .or_insert(Model2D::new(
            vec![],
            vec![],
            vec![Texture2D::new(
                text_2d_image(config.clone(), || {
                    String::from(format!("up:      ({:+.2},{:+.2},{:+.2})",
                                         up.x.to_f32().unwrap(),
                                         up.y.to_f32().unwrap(),
                                         up.z.to_f32().unwrap(),
                    ))}),
                Point2D::new(x, y_up),
                F::from(1.0).unwrap(),
            )],
        ));
}
