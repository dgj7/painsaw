use crate::config::{CVAR_FOV, DEFAULT_FOV};
use crate::graphics::camera::Camera;
use crate::graphics::geometry::primitive::PrimitiveType;
use crate::graphics::image::t2d::Texture2D;
use crate::graphics::storage::g2d::Graph2D;
use crate::graphics::storage::g3d::Graph3D;
use crate::graphics::subsystem::opengl::opengl_api::{gl_begin_lines, gl_begin_points, gl_begin_quads, gl_bind_texture, gl_blend_func, gl_clear, gl_clear_color, gl_color_3f, gl_disable, gl_enable, gl_end, gl_gen_textures, gl_get_string, gl_line_width, gl_load_identity, gl_matrix_mode, gl_ortho, gl_point_size, gl_pop_attrib, gl_pop_matrix, gl_push_attrib, gl_push_matrix, gl_rotate_f, gl_tex_coord_2f, gl_tex_env_f, gl_tex_image_2d, gl_tex_parameter_i, gl_tex_sub_image_2d, gl_translate_f, gl_vertex_2f, gl_vertex_3f, gl_viewport, glu_perspective};
use crate::graphics::subsystem::RendererInfo;
use crate::graphics::subsystem::{OpenGLPipeline, RenderingSubSystemHandle};
use crate::logger::log;
use crate::logger::log_level::LogLevel;
use crate::window::context::RendererContext;
use num_traits::Float;
use std::ffi::c_void;
use std::ops::{Add, Sub};
use windows::Win32::Graphics::OpenGL::{GL_ALL_ATTRIB_BITS, GL_BLEND, GL_COLOR_BUFFER_BIT, GL_DEPTH_BUFFER_BIT, GL_DEPTH_TEST, GL_MODELVIEW, GL_NEAREST, GL_ONE_MINUS_SRC_ALPHA, GL_PROJECTION, GL_RENDERER, GL_REPLACE, GL_RGBA, GL_SRC_ALPHA, GL_TEXTURE_2D, GL_TEXTURE_ENV, GL_TEXTURE_ENV_MODE, GL_TEXTURE_MAG_FILTER, GL_TEXTURE_MIN_FILTER, GL_UNSIGNED_BYTE, GL_VENDOR, GL_VERSION};

pub(crate) mod opengl_mswin_api;
pub(crate) mod opengl_mswin;
pub(crate) mod opengl_api;
mod opengl_errors;

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
            OpenGLPipeline::Shaders => {
                None
            }
        }
    }

    fn initialize(&self, g2d: &mut Graph2D<F>, _g3d: &mut Graph3D<F>) {
        match self.pipeline {
            OpenGLPipeline::FixedFunction => {
                /* initialize textures */
                gl_enable(GL_TEXTURE_2D);
                for (_, value) in g2d.models.iter_mut() {
                    for tex in &mut value.textures {
                        self.initialize_texture_2d(tex);
                    }
                }

                /* done */
                log(LogLevel::Debug, &|| String::from("initialization complete"));
            }
            OpenGLPipeline::Shaders => {}
        }
    }

    fn initialize_texture_2d(&self, texture: &mut Texture2D<F>) {
        match self.pipeline {
            OpenGLPipeline::FixedFunction => {
                /* gen 1 texture; bind it */
                let texture_id_ptr: *mut u32 = (&mut texture.id) as *mut u32;
                gl_gen_textures(1, texture_id_ptr);
                gl_bind_texture(GL_TEXTURE_2D, texture.id);

                /* set texture params */
                gl_tex_parameter_i(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST as i32);
                gl_tex_parameter_i(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST as i32);

                /* inform opengl of the texture data; see https://registry.khronos.org/OpenGL-Refpages/gl4/html/glTexImage2D.xhtml */
                gl_tex_image_2d(
                    GL_TEXTURE_2D,                                  // target
                    0,                                              // level
                    GL_RGBA as i32,                                 // internal format; the number of color components in the texture data
                    texture.image.width as i32,                     // width
                    texture.image.height as i32,                    // height
                    0,                                              // border
                    GL_RGBA,                                        // format: (or order) of pixel data (r,g,b,a)
                    GL_UNSIGNED_BYTE,                               // r-type: the data type of the pixel data
                    texture.image.data.as_ptr() as *const c_void,   // pixels
                );

                /* mark the texture as initialized (ready) */
                texture.initialized = true;

                /* done */
                log(LogLevel::Info, &|| String::from(format!("created texture, id=[{}]", texture.id)));
            }
            OpenGLPipeline::Shaders => {}
        }
    }

    fn update_texture_2d(&self, texture: &mut Texture2D<F>) {
        match self.pipeline {
            OpenGLPipeline::FixedFunction => {
                if texture.replacement.is_some() {
                    let repl = texture.replacement.take().unwrap();
                    gl_bind_texture(GL_TEXTURE_2D, texture.id);
                    gl_tex_sub_image_2d(
                        GL_TEXTURE_2D,                              // target
                        0,                                          // level
                        0,                                          // x-offset
                        0,                                          // y-offset
                        repl.width as i32,                          // width
                        repl.height as i32,                         // height
                        GL_RGBA,                                    // format: (order) of pixel data (r, g, b, a)
                        GL_UNSIGNED_BYTE,                           // r-type: the dat type of the pixel data
                        repl.data.as_ptr() as *const c_void,        // pixels
                    );
                    texture.image = repl;
                }
            }
            OpenGLPipeline::Shaders => {}
        }
    }

    fn resize(&self, context: &RendererContext<F>) {
        match self.pipeline {
            OpenGLPipeline::FixedFunction => {
                /* get camera data */
                let camera = &context.camera;

                /* set the viewport; this call doesn't need a specific matrix mode as it's an independent function */
                gl_viewport(0, 0, camera.width.to_i32().unwrap(), camera.height.to_i32().unwrap());

                /* observe and report */
                log(LogLevel::Debug, &|| String::from(format!("resize(): w=[{}],h=[{}]", camera.width.to_f32().unwrap(), camera.height.to_f32().unwrap())));
            }
            OpenGLPipeline::Shaders => {}
        }
    }

    fn before_scene(&self, _camera: &Camera<F>) {
        match self.pipeline {
            OpenGLPipeline::FixedFunction => {
                gl_clear_color(0.0, 0.0, 0.0, 1.0);
                gl_clear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
            }
            OpenGLPipeline::Shaders => {}
        }
    }

    fn prepare_2d(&self, camera: &Camera<F>) {
        match self.pipeline {
            OpenGLPipeline::FixedFunction => {
                /* save prior state before 2d rendering */
                gl_push_matrix();
                gl_push_attrib(GL_ALL_ATTRIB_BITS);

                /* projection: reset matrix; setup ortho for 2d drawing */
                gl_matrix_mode(GL_PROJECTION);
                gl_load_identity();
                gl_ortho(0.0, camera.width.to_f64().unwrap(), camera.height.to_f64().unwrap(), 0.0, -99999.0, 99999.0);

                /* storage/view: reset matrix; ready for 2d drawing */
                gl_matrix_mode(GL_MODELVIEW);
                gl_load_identity();
            }
            OpenGLPipeline::Shaders => {}
        }
    }

    fn render_2d(&self, g2d: &mut Graph2D<F>) {
        match self.pipeline {
            OpenGLPipeline::FixedFunction => {
                for (_, model) in g2d.models.iter() {
                    for primitive in model.primitives.iter() {
                        /* gather some variables */
                        let vertices = &primitive.vertices;

                        match primitive.p_type {
                            PrimitiveType::Point{point_size} => {
                                /* save prior state before making changes */
                                gl_push_matrix();
                                gl_push_attrib(GL_ALL_ATTRIB_BITS);

                                gl_color_3f(primitive.color.red, primitive.color.green, primitive.color.blue);
                                gl_point_size(point_size.to_f32().unwrap());

                                gl_begin_points();
                                for point in vertices.iter() {
                                    gl_vertex_2f(point.x.to_f32().unwrap(), point.y.to_f32().unwrap());
                                }
                                gl_end();

                                gl_pop_attrib();
                                gl_pop_matrix();
                            },
                            PrimitiveType::Line{thickness} => {
                                gl_push_matrix();
                                gl_push_attrib(GL_ALL_ATTRIB_BITS);

                                gl_color_3f(primitive.color.red, primitive.color.green, primitive.color.blue);
                                gl_line_width(thickness.to_f32().unwrap());

                                gl_begin_lines();
                                for vertex in vertices.iter() {
                                    gl_vertex_2f(vertex.x.to_f32().unwrap(), vertex.y.to_f32().unwrap());
                                }
                                gl_end();

                                gl_pop_attrib();
                                gl_pop_matrix();
                            }
                        }
                    }
                }
            },
            OpenGLPipeline::Shaders => {},
        }
    }

    fn after_2d(&self) {
        match self.pipeline {
            OpenGLPipeline::FixedFunction => {
                gl_pop_attrib();
                gl_pop_matrix();
            },
            OpenGLPipeline::Shaders => {},
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
            OpenGLPipeline::Shaders => {}
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
                                    OpenGLPipeline::Shaders => {},
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
                                    OpenGLPipeline::Shaders => {},
                                }
                            }
                        }
                    }
                }
            },
            OpenGLPipeline::Shaders => {},
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
            OpenGLPipeline::Shaders => {},
        }
    }

    fn render_2d_textures(&self, texture: &Texture2D<F>) {
        match self.pipeline {
            OpenGLPipeline::FixedFunction => {
                /* save prior state before making changes */
                gl_push_matrix();
                gl_push_attrib(GL_ALL_ATTRIB_BITS);

                /* gather variables */
                let scale = texture.scale.to_f32().unwrap();
                let x = texture.x.to_f32().unwrap();
                let y = texture.y.to_f32().unwrap();
                let width = texture.image.width as f32;
                let height = texture.image.height as f32;

                /* prepare to render textures */
                gl_enable(GL_TEXTURE_2D);
                gl_bind_texture(GL_TEXTURE_2D, texture.id);
                gl_tex_env_f(GL_TEXTURE_ENV, GL_TEXTURE_ENV_MODE, GL_REPLACE as f32);

                /* enable transparency in textures */
                gl_enable(GL_BLEND);
                gl_blend_func(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);

                /* start drawing texture; only text coords and vertexes until gl-end! */
                gl_begin_quads();

                /* top left */
                gl_tex_coord_2f(0.0, 0.0);
                gl_vertex_2f(x, y);

                /* bottom left */
                gl_tex_coord_2f(0.0, 1.0);
                gl_vertex_2f(x, y + height * scale);

                /* bottom right */
                gl_tex_coord_2f(1.0, 1.0);
                gl_vertex_2f(x + width * scale, y +  height * scale);

                /* top right */
                gl_tex_coord_2f(1.0, 0.0);
                gl_vertex_2f(x + width * scale, y);

                /* done */
                gl_end();

                /* turn off the stuff we enabled (specifically for texturing) */
                // todo: can these be removed?
                gl_disable(GL_TEXTURE_2D);
                gl_disable(GL_BLEND);

                /* restore prior state */
                gl_pop_attrib();
                gl_pop_matrix();
            }
            OpenGLPipeline::Shaders => {}
        }
    }
}
