use crate::graphics::camera::Camera;
use crate::graphics::geometry::primitive::PrimitiveType;
use crate::graphics::storage::g2d::Graph2D;
use crate::graphics::storage::g3d::Graph3D;
use crate::graphics::subsystem::opengl::ffp::ffp2d::{ffp_2d_initialize_textures, ffp_2d_update_textures};
use crate::graphics::subsystem::opengl::ffp::ffp3d::{ffp_3d_lines, ffp_3d_points, ffp_3d_setup, ffp_3d_teardown};
use crate::graphics::subsystem::opengl::ffp::{ffp_before_scene, ffp_resize};
use crate::graphics::subsystem::RendererInfo;
use crate::graphics::subsystem::{OpenGLPipeline, RenderingSubSystemHandle};
use crate::window::context::RendererContext;
use ffp::api::gl_get_string;
use ffp::ffp2d::{ffp_2d_setup, ffp_2d_teardown, ffp_render_2d_lines, ffp_render_2d_points, ffp_render_2d_texture};
use num_traits::Float;
use std::ops::{Add, Sub};
use windows::Win32::Graphics::OpenGL::{GL_RENDERER, GL_VENDOR, GL_VERSION};

pub(crate) mod opengl_mswin_api;
pub(crate) mod opengl_mswin;
mod errors;
pub mod ffp;

pub struct OpenGLHandle {
    pub(crate) pipeline: OpenGLPipeline,
}

impl<F: Float + Add<F> + Sub<F>> RenderingSubSystemHandle<F> for OpenGLHandle {
    fn identify(&self) -> Option<RendererInfo> {
        match self.pipeline {
            OpenGLPipeline::FixedFunction => {
                let version = gl_get_string(GL_VERSION);
                let vendor = gl_get_string(GL_VENDOR);
                let device = gl_get_string(GL_RENDERER);

                Some(RendererInfo { name: Some(String::from("OpenGL")), version, vendor, device })
            }
            OpenGLPipeline::ProgrammableShader => {
                None
            }
        }
    }

    fn initialize(&self, g2d: &mut Graph2D<F>, _g3d: &mut Graph3D<F>) {
        match self.pipeline {
            OpenGLPipeline::FixedFunction => {ffp_2d_initialize_textures(g2d)}
            OpenGLPipeline::ProgrammableShader => {}
        }
    }

    fn resize(&self, context: &RendererContext<F>) {
        match self.pipeline {
            OpenGLPipeline::FixedFunction => {ffp_resize(&context.camera)}
            OpenGLPipeline::ProgrammableShader => {}
        }
    }

    fn before_scene(&self, _camera: &Camera<F>) {
        match self.pipeline {
            OpenGLPipeline::FixedFunction => {ffp_before_scene()}
            OpenGLPipeline::ProgrammableShader => {}
        }
    }

    fn prepare_2d(&self, camera: &Camera<F>, g2d: &mut Graph2D<F>) {
        match self.pipeline {
            OpenGLPipeline::FixedFunction => {
                ffp_2d_setup(camera);
                ffp_2d_update_textures(g2d);
            }
            OpenGLPipeline::ProgrammableShader => {}
        }
    }

    fn render_2d(&self, g2d: &mut Graph2D<F>) {
        match self.pipeline {
            OpenGLPipeline::FixedFunction => {
                for (_, model) in g2d.models.iter() {
                    for primitive in model.primitives.iter() {
                        match primitive.p_type {
                            PrimitiveType::Point{point_size} => { ffp_render_2d_points(primitive, point_size)},
                            PrimitiveType::Line{thickness} => { ffp_render_2d_lines(primitive, thickness)},
                        }
                    }

                    model.textures
                        .iter()
                        .filter(|x| x.initialized)
                        .for_each(|x| ffp_render_2d_texture(x));
                }
            },
            OpenGLPipeline::ProgrammableShader => {},
        }
    }

    fn after_2d(&self) {
        match self.pipeline {
            OpenGLPipeline::FixedFunction => {ffp_2d_teardown()},
            OpenGLPipeline::ProgrammableShader => {},
        }
    }

    fn prepare_3d(&self, context: &RendererContext<F>) {
        match self.pipeline {
            OpenGLPipeline::FixedFunction => {ffp_3d_setup(context)},
            OpenGLPipeline::ProgrammableShader => {}
        }
    }

    fn render_3d(&self, g3d: &mut Graph3D<F>) {
        match self.pipeline {
            OpenGLPipeline::FixedFunction => {
                for (_, model) in g3d.iter() {
                    for primitive in model.primitives.iter() {
                        match primitive.ptype {
                            PrimitiveType::Point {point_size} => {
                                match self.pipeline {
                                    OpenGLPipeline::FixedFunction => {ffp_3d_points(primitive, point_size)},
                                    OpenGLPipeline::ProgrammableShader => {},
                                }
                            },
                            PrimitiveType::Line {thickness} => {
                                match self.pipeline {
                                    OpenGLPipeline::FixedFunction => {ffp_3d_lines(primitive, thickness)},
                                    OpenGLPipeline::ProgrammableShader => {},
                                }
                            }
                        }
                    }
                }
            },
            OpenGLPipeline::ProgrammableShader => {},
        }
    }

    fn after_3d(&self, _context: &RendererContext<F>) {
        match self.pipeline {
            OpenGLPipeline::FixedFunction => {ffp_3d_teardown()},
            OpenGLPipeline::ProgrammableShader => {},
        }
    }
}
