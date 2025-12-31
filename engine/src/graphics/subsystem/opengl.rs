use crate::geometry::dim::d2d::Dimension2D;
use crate::geometry::line::ls2d::Lines2D;
use crate::geometry::line::ls3d::Lines3D;
use crate::geometry::vector::ps2d::Points2D;
use crate::geometry::vector::ps3d::Points3D;
use crate::graphics::model::renderer_info::RendererInfo;
use crate::graphics::subsystem::opengl::opengl_api::{gl_begin_lines, gl_begin_points, gl_clear, gl_clear_color, gl_color_3f, gl_disable, gl_enable, gl_end, gl_line_width, gl_load_identity, gl_matrix_mode, gl_ortho, gl_point_size, gl_vertex_2f, gl_vertex_3f, gl_viewport};
use crate::graphics::subsystem::{OpenGLPipeline, RenderingSubSystemHandle};
use crate::logger::log;
use crate::logger::log_level::LogLevel;
use num_traits::{Float, ToPrimitive};
use std::ops::{Add, Sub};
use windows::Win32::Graphics::OpenGL::{GL_COLOR_BUFFER_BIT, GL_DEPTH_BUFFER_BIT, GL_DEPTH_TEST, GL_MODELVIEW, GL_PROJECTION};

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
                None
            }
            OpenGLPipeline::Shaders => {
                None
            }
        }
    }

    fn initialize(&self) {
        match self.pipeline {
            OpenGLPipeline::FixedFunction => {
                log(LogLevel::Debug, &|| String::from("initialization complete"));
            }
            OpenGLPipeline::Shaders => {}
        }
    }

    fn before_scene(&self, ccd: &Dimension2D<f32>) {
        match self.pipeline {
            OpenGLPipeline::FixedFunction => {
                gl_clear_color(0.0, 0.0, 0.0, 1.0);
                gl_clear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
                gl_viewport(0, 0, ccd.width.to_i32().unwrap(), ccd.height.to_i32().unwrap());
            }
            OpenGLPipeline::Shaders => {}
        }
    }

    fn prepare_2d(&self, ccd: &Dimension2D<f32>) {
        match self.pipeline {
            OpenGLPipeline::FixedFunction => {
                gl_disable(GL_DEPTH_TEST);
                gl_matrix_mode(GL_PROJECTION);
                gl_load_identity();
                gl_ortho(0.0,
                         ccd.width.to_f64().unwrap(),
                         ccd.height.to_f64().unwrap(),
                         0.0,
                         -99999.0,
                         99999.0
                );
                gl_matrix_mode(GL_MODELVIEW);
            }
            OpenGLPipeline::Shaders => {}
        }
    }

    fn prepare_3d(&self) {
        match self.pipeline {
            OpenGLPipeline::FixedFunction => {
                gl_enable(GL_DEPTH_TEST);

                gl_matrix_mode(GL_PROJECTION);
                gl_load_identity();
                // todo: do camera; gluPerspective, glFrustum
                gl_matrix_mode(GL_MODELVIEW);
                gl_load_identity();
            }
            OpenGLPipeline::Shaders => {}
        }
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
