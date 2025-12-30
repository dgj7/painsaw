use std::ops::{Add, Sub};
use num_traits::Float;
use windows::Win32::Graphics::OpenGL::{GL_COLOR_BUFFER_BIT, GL_DEPTH_BUFFER_BIT, GL_DEPTH_TEST, GL_MODELVIEW, GL_PROJECTION};
use crate::geometry::dim::d2d::Dimension2D;
use crate::geometry::storage::g2d::Graph2D;
use crate::geometry::storage::g3d::Graph3D;
use crate::graphics::model::color::Color;
use crate::graphics::model::renderer_info::RendererInfo;
use crate::graphics::subsystem::opengl::opengl_api::{gl_clear, gl_clear_color, gl_disable, gl_enable, gl_load_identity, gl_matrix_mode, gl_ortho, gl_viewport};
use crate::graphics::subsystem::opengl::opengl_wrapper_2d::paint_2d_lines;
use crate::graphics::subsystem::opengl::opengl_wrapper_3d::paint_3d_lines;
use crate::graphics::subsystem::RenderingSubSystemHandle;
use crate::logger::log;
use crate::logger::log_level::LogLevel;
use crate::window::wc::context::RendererContext;

pub(crate) mod opengl_mswin_api;
pub(crate) mod opengl_mswin;
pub(crate) mod opengl_api;
pub(crate) mod opengl_wrapper_2d;
pub(crate) mod opengl_wrapper_3d;
mod opengl_errors;

pub struct OpenGLHandle {}

impl<F: Float + Add<F> + Sub<F>> RenderingSubSystemHandle<F> for OpenGLHandle {
    fn identify(&self) -> RendererInfo {
        todo!()
    }

    fn initialize(&self, _context: &RendererContext<F>) {
        log(LogLevel::Debug, &|| String::from("initialization complete"));
    }

    fn before_scene(&self, ccd: &Dimension2D<F>) {
        paint_background(Color::BLACK);
        gl_viewport(0, 0, ccd.width.to_i32().unwrap(), ccd.height.to_i32().unwrap());
    }

    fn render_2d(&self, graph: &Graph2D<F>, ccd: &Dimension2D<F>) {
        /* prepare 2d rendering */
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

        /* graphics 2d */
        // todo: this is in need of another reorg
        for (_, model) in graph.models.iter() {
            for lines in &model.lines {
                paint_2d_lines(&lines);
            }
        }
    }

    fn render_3d(&self, graph: &Graph3D<F>) {
        /* prepare 3d rendering */
        gl_enable(GL_DEPTH_TEST);

        gl_matrix_mode(GL_PROJECTION);
        gl_load_identity();
        // todo: do camera; gluPerspective, glFrustum
        gl_matrix_mode(GL_MODELVIEW);
        gl_load_identity();

        /* graphics 3d */
        // todo: this is in need of another reorg
        for (_, model) in graph.models.iter() {
            for lines in &model.lines {
                paint_3d_lines(&lines);
            }
        }
    }
}

fn paint_background(color: Color) {
    gl_clear_color(color.red, color.green, color.blue, color.alpha);
    gl_clear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
}
