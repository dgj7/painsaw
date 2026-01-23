use crate::graphics::geometry::line::Lines2D;
use crate::graphics::geometry::line::Lines3D;
use crate::graphics::geometry::point::Points2D;
use crate::graphics::geometry::point::Points3D;
use crate::graphics::camera::Camera;
use crate::graphics::model::g2d::Graph2D;
use crate::graphics::model::g3d::Graph3D;
use crate::graphics::model::renderer_info::RendererInfo;
use crate::graphics::subsystem::opengl::opengl_api::{gl_begin_lines, gl_begin_points, gl_begin_quads, gl_bind_texture, gl_blend_func, gl_clear, gl_clear_color, gl_color_3f, gl_disable, gl_enable, gl_end, gl_gen_textures, gl_get_string, gl_line_width, gl_load_identity, gl_matrix_mode, gl_ortho, gl_point_size, gl_pop_matrix, gl_push_matrix, gl_rotate_f, gl_tex_coord_2f, gl_tex_env_f, gl_tex_image_2d, gl_tex_parameter_i, gl_tex_sub_image_2d, gl_translate_f, gl_vertex_2f, gl_vertex_3f, gl_viewport, glu_perspective};
use crate::graphics::subsystem::{OpenGLPipeline, RenderingSubSystemHandle};
use crate::graphics::image::t2d::Texture2D;
use crate::logger::log;
use crate::logger::log_level::LogLevel;
use num_traits::Float;
use std::ffi::c_void;
use std::ops::{Add, Sub};
use windows::Win32::Graphics::OpenGL::{GL_BLEND, GL_COLOR_BUFFER_BIT, GL_DEPTH_BUFFER_BIT, GL_DEPTH_TEST, GL_MODELVIEW, GL_NEAREST, GL_ONE_MINUS_SRC_ALPHA, GL_PROJECTION, GL_RENDERER, GL_REPLACE, GL_RGBA, GL_SRC_ALPHA, GL_TEXTURE_2D, GL_TEXTURE_ENV, GL_TEXTURE_ENV_MODE, GL_TEXTURE_MAG_FILTER, GL_TEXTURE_MIN_FILTER, GL_UNSIGNED_BYTE, GL_VENDOR};
use crate::window::context::{RendererContext, CVAR_FOV, DEFAULT_FOV};

pub(crate) mod opengl_mswin_api;
pub(crate) mod opengl_mswin;
pub(crate) mod opengl_api;
mod opengl_errors;

pub struct OpenGLHandle {
    pub(crate) pipeline: OpenGLPipeline,
}

impl OpenGLHandle {
    
}

impl<F: Float + Add<F> + Sub<F>> RenderingSubSystemHandle<F> for OpenGLHandle {
    fn identify(&self) -> Option<RendererInfo> {
        match self.pipeline {
            OpenGLPipeline::FixedFunction => {
                let version = gl_get_string(GL_VENDOR);
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

    fn update_texture_2d(&self, texture: &mut Texture2D<F>) {
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

    fn resize(&self, context: &RendererContext<F>) {
        let camera = &context.camera;
        let fov: f64 = context.get_cvar(CVAR_FOV, |x| x.parse().unwrap()).unwrap_or(DEFAULT_FOV);
        gl_viewport(0, 0, camera.width as i32, camera.height as i32);
        gl_matrix_mode(GL_PROJECTION);
        gl_load_identity();
        glu_perspective(fov, (camera.width / camera.height) as f64, 0.01, 500.0);
        //gl_frustum(-1.0, 1.0, -1.0, 1.0, 1.5, 20.0);
    }

    fn before_scene(&self, _camera: &Camera) {
        match self.pipeline {
            OpenGLPipeline::FixedFunction => {
                gl_clear_color(0.0, 0.0, 0.0, 1.0);
                gl_clear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
            }
            OpenGLPipeline::Shaders => {}
        }
    }

    fn prepare_2d(&self, camera: &Camera) {
        match self.pipeline {
            OpenGLPipeline::FixedFunction => {
                gl_disable(GL_DEPTH_TEST);
                gl_matrix_mode(GL_PROJECTION);
                gl_push_matrix();
                gl_load_identity();
                gl_ortho(0.0,
                         camera.width as f64,
                         camera.height as f64,
                         0.0,
                         -99999.0,
                         99999.0
                );
                gl_matrix_mode(GL_MODELVIEW);
                gl_push_matrix();
                gl_load_identity();
            }
            OpenGLPipeline::Shaders => {}
        }
    }

    fn after_2d(&self) {
        gl_matrix_mode(GL_PROJECTION);
        gl_pop_matrix();
        gl_matrix_mode(GL_MODELVIEW);
        gl_pop_matrix();
    }

    fn prepare_3d(&self, context: &RendererContext<F>) {
        match self.pipeline {
            OpenGLPipeline::FixedFunction => {
                let camera = &context.camera;

                gl_enable(GL_DEPTH_TEST);

                gl_matrix_mode(GL_MODELVIEW);
                gl_load_identity();

                //gl_translate_f(0.0, 0.0, -1.0);

                //gl_rotate_f(-45.0, 0.0, 1.0, 0.0);// rotate: yaw,  y-axis; only degrees and y-axis are set
                //gl_rotate_f(-20.0, 1.0, 0.0, 0.0);// rotate: pitch, x-axis; only degrees and x-axis are set; positive rotates forward down
                //gl_rotate_f(0.0, 0.0, 0.0, 1.0);// rotate: roll/bank, z-axis; only degrees and z-axis are set

                gl_rotate_f(-camera.pitch, 1.0, 0.0, 0.0);
                gl_rotate_f(-camera.yaw, 0.0, 1.0, 0.0);
                gl_translate_f(-camera.position.x, -camera.position.y, -camera.position.z);
            }
            OpenGLPipeline::Shaders => {}
        }
    }

    fn after_3d(&self) {
        // todo: needed later?
    }

    fn render_2d_points(&self, points: &Points2D<F>) {
        match self.pipeline {
            OpenGLPipeline::FixedFunction => {
                gl_color_3f(points.color.red, points.color.green, points.color.blue);
                gl_point_size(points.thickness.to_f32().unwrap());

                gl_begin_points();
                for point in points.points.iter() {
                    gl_vertex_2f(point.x.to_f32().unwrap(), point.y.to_f32().unwrap());
                }
                gl_end();
            }
            OpenGLPipeline::Shaders => {}
        }
    }

    fn render_2d_lines(&self, lines: &Lines2D<F>) {
        match self.pipeline {
            OpenGLPipeline::FixedFunction => {
                gl_color_3f(lines.color.red, lines.color.green, lines.color.blue);
                gl_line_width(lines.thickness.to_f32().unwrap());

                gl_begin_lines();
                for line in lines.lines.iter() {
                    gl_vertex_2f(line.x.x.to_f32().unwrap(), line.x.y.to_f32().unwrap());
                    gl_vertex_2f(line.y.x.to_f32().unwrap(), line.y.y.to_f32().unwrap());
                }
                gl_end();
            }
            OpenGLPipeline::Shaders => {}
        }
    }

    fn render_2d_textures(&self, texture: &Texture2D<F>) {
        match self.pipeline {
            OpenGLPipeline::FixedFunction => {
                /* gather variables */
                let scale = texture.scale.to_f32().unwrap();
                let x = texture.world_pos.x.to_f32().unwrap();
                let y = texture.world_pos.y.to_f32().unwrap();
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
                gl_disable(GL_TEXTURE_2D);
                gl_disable(GL_BLEND);
            }
            OpenGLPipeline::Shaders => {}
        }
    }

    fn render_3d_points(&self, points: &Points3D<F>) {
        match self.pipeline {
            OpenGLPipeline::FixedFunction => {
                gl_color_3f(points.color.red, points.color.green, points.color.blue);
                gl_point_size(points.thickness.to_f32().unwrap());

                gl_begin_points();
                for point in points.points.iter() {
                    gl_vertex_3f(point.x.to_f32().unwrap(), point.y.to_f32().unwrap(), point.z.to_f32().unwrap());
                }
                gl_end();
            }
            OpenGLPipeline::Shaders => {}
        }
    }

    fn render_3d_lines(&self, lines: &Lines3D<F>) {
        match self.pipeline {
            OpenGLPipeline::FixedFunction => {
                gl_color_3f(lines.color.red, lines.color.green, lines.color.blue);
                gl_line_width(lines.thickness.to_f32().unwrap());

                gl_begin_lines();
                for line in &lines.lines {
                    gl_vertex_3f(line.a.x.to_f32().unwrap(), line.a.y.to_f32().unwrap(), line.a.z.to_f32().unwrap());
                    gl_vertex_3f(line.b.x.to_f32().unwrap(), line.b.y.to_f32().unwrap(), line.b.z.to_f32().unwrap());
                }
                gl_end();
            }
            OpenGLPipeline::Shaders => {}
        }
    }
}
