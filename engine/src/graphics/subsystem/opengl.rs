use crate::config::{CVAR_FOV, DEFAULT_FOV};
use crate::graphics::camera::Camera;
use crate::graphics::geometry::primitive::PrimitiveType;
use crate::graphics::storage::g2d::Graph2D;
use crate::graphics::storage::g3d::Graph3D;
use crate::graphics::subsystem::opengl::ffp::ffp2d::{ffp_2d_initialize_textures, ffp_2d_update_textures};
use crate::graphics::subsystem::opengl::ffp::{ffp_before_scene, ffp_resize};
use crate::graphics::subsystem::RendererInfo;
use crate::graphics::subsystem::{OpenGLPipeline, RenderingSubSystemHandle};
use crate::window::context::RendererContext;
use ffp::api::{gl_begin_lines, gl_begin_points, gl_color_3f, gl_enable, gl_end, gl_get_string, gl_line_width, gl_load_identity, gl_matrix_mode, gl_point_size, gl_pop_attrib, gl_pop_matrix, gl_push_attrib, gl_push_matrix, gl_rotate_f, gl_translate_f, gl_vertex_3f, glu_perspective};
use ffp::ffp2d::{ffp_2d_setup, ffp_2d_teardown, ffp_render_2d_lines, ffp_render_2d_points, ffp_render_2d_texture};
use num_traits::Float;
use std::ops::{Add, Sub};
use windows::Win32::Graphics::OpenGL::{GL_ALL_ATTRIB_BITS, GL_DEPTH_TEST, GL_MODELVIEW, GL_PROJECTION, GL_RENDERER, GL_VENDOR, GL_VERSION};

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
            OpenGLPipeline::FixedFunction => {
                /* save prior state before 3d rendering */
                gl_push_matrix();
                gl_push_attrib(GL_ALL_ATTRIB_BITS);

                /* gather camera data */
                let camera = &context.camera;
                let position = camera.orientation.position.column_major_position();

                /* projection: reset matrix */
                gl_matrix_mode(GL_PROJECTION);
                gl_load_identity();

                /* adjust perspective (removes ortho) */
                let fov: f64 = context.config.get_cvar(CVAR_FOV, |x| x.parse().unwrap()).unwrap_or(DEFAULT_FOV);
                glu_perspective(fov, camera.aspect(), camera.near.to_f64().unwrap(), camera.far.to_f64().unwrap());

                /* storage/view: reset matrix; enable depth test; ready for 3d drawing */
                gl_matrix_mode(GL_MODELVIEW);
                gl_load_identity();
                gl_enable(GL_DEPTH_TEST);

                /* storage/view: adjust camera, before drawing */
                gl_rotate_f(-camera.orientation.pitch().to_f32().unwrap(), 1.0, 0.0, 0.0);
                gl_rotate_f(-camera.orientation.yaw().to_f32().unwrap(), 0.0, 1.0, 0.0);
                gl_translate_f(-position.x.to_f32().unwrap(), -position.y.to_f32().unwrap(), -position.z.to_f32().unwrap());
            }
            OpenGLPipeline::ProgrammableShader => {}
        }
    }

    fn render_3d(&self, g3d: &mut Graph3D<F>) {
        match self.pipeline {
            OpenGLPipeline::FixedFunction => {
                for (_, model) in g3d.models.iter() {
                    for primitive in model.primitives.iter() {
                        /* gather some access variables */
                        let vertices = &primitive.vertices;

                        /* do rendering, based on primitive type */
                        match primitive.ptype {
                            PrimitiveType::Point {point_size} => {
                                match self.pipeline {
                                    OpenGLPipeline::FixedFunction => {
                                        gl_push_matrix();
                                        gl_push_attrib(GL_ALL_ATTRIB_BITS);

                                        // todo: translate, rotate, scale

                                        gl_color_3f(primitive.color.red, primitive.color.green, primitive.color.blue);
                                        gl_point_size(point_size.to_f32().unwrap());

                                        gl_begin_points();
                                        for vert in vertices {
                                            gl_vertex_3f(vert.x.to_f32().unwrap(), vert.y.to_f32().unwrap(), vert.z.to_f32().unwrap());
                                        }
                                        gl_end();

                                        gl_pop_attrib();
                                        gl_pop_matrix();
                                    },
                                    OpenGLPipeline::ProgrammableShader => {},
                                }
                            },
                            PrimitiveType::Line {thickness} => {
                                match self.pipeline {
                                    OpenGLPipeline::FixedFunction => {
                                        gl_push_matrix();
                                        gl_push_attrib(GL_ALL_ATTRIB_BITS);

                                        // todo: translate, rotate, scale

                                        gl_color_3f(primitive.color.red, primitive.color.green, primitive.color.blue);
                                        gl_line_width(thickness.to_f32().unwrap());

                                        gl_begin_lines();
                                        for vert in vertices {
                                            gl_vertex_3f(vert.x.to_f32().unwrap(), vert.y.to_f32().unwrap(), vert.z.to_f32().unwrap());
                                        }
                                        gl_end();

                                        gl_pop_attrib();
                                        gl_pop_matrix();
                                    },
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
            OpenGLPipeline::FixedFunction => {
                /* storage/view: reset matrix */
                //gl_matrix_mode(GL_MODELVIEW);
                //gl_load_identity();

                /* disable stuff we don't need anymore */
                //gl_disable(GL_DEPTH_TEST);


                gl_pop_attrib();
                gl_pop_matrix();
            },
            OpenGLPipeline::ProgrammableShader => {},
        }
    }
}
