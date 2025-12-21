use crate::math::twod::line_2d::Line2D;
use crate::render::graphics::opengl::opengl_api::{gl_begin_lines, gl_clear, gl_clear_color, gl_color_3f, gl_end, gl_flush, gl_line_width, gl_load_identity, gl_matrix_mode, gl_ortho, gl_vertex_2f, gl_viewport};
use crate::render::model::color::Color;
use windows::Win32::Graphics::OpenGL::{GL_COLOR_BUFFER_BIT, GL_DEPTH_BUFFER_BIT, GL_PROJECTION};
use crate::math::twod::dimension_2d::Dimension2D;
use crate::math::twod::draw_config_2d::DrawingConfig2D;
use crate::math::twod::point_2d::Point2D;
use crate::render::model::render_context::RendererContext;

pub fn paint_background(color: Color) {
    gl_clear_color(color.red, color.green, color.blue, color.alpha);
    gl_clear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
}

pub fn prepare_2d(context: &mut RendererContext) {
    let ccd = context.copy_client_dimensions();
    gl_viewport(0, 0, ccd.width as i32, ccd.height as i32);
    gl_matrix_mode(GL_PROJECTION);
    gl_load_identity();
    gl_ortho(0.0, 
             ccd.width as f64, 
             ccd.height as f64, 
             0.0, 
             -1.0, 
             1.0
    );
}

pub fn paint_lines(lines: &[Line2D], config: &DrawingConfig2D) {
    gl_color_3f(config.color.red, config.color.green, config.color.blue);
    gl_line_width(config.line_thickness);
    
    gl_begin_lines();
    for line in lines {
        gl_vertex_2f(line.x.x, line.x.y);
        gl_vertex_2f(line.y.x, line.y.y);
    }
    gl_end();
    
    gl_flush();
}

pub fn paint_grid(client: &Dimension2D) {
    /* draw x&y axes in white */
    paint_lines(&*vec!(
        Line2D::new(Point2D::origin(), Point2D::new(0.0, client.height)),
        Line2D::new(Point2D::origin(), Point2D::new(client.width, 0.0)),
    ), &DrawingConfig2D::new(Color::from_rgb(0.498, 0.0, 1.0), 10.0));

    /* draw x-axis horizontal lines */
    let hgap = 10;
    let hiters = ((client.height + (hgap as f32))/(hgap as f32)) as u16;
    let mut hlines: Vec<Line2D> = Vec::with_capacity((hiters + 10) as usize);
    for h in 0..hiters {
        hlines.push(Line2D::new(Point2D::new(0.0, (h * hgap) as f32), Point2D::new(client.width, (h * hgap) as f32)));
    }
    paint_lines(&*hlines, &DrawingConfig2D::new(Color::from_rgb(0.2, 0.2, 0.2), 1.0));

    /* draw y-axis vertical lines */
    let vgap = 10;
    let viters = ((client.width + (vgap as f32))/(vgap as f32)) as u16;
    let mut vlines: Vec<Line2D> = Vec::with_capacity((viters + 10) as usize);
    for v in 0..viters {
        vlines.push(Line2D::new(Point2D::new((v * vgap) as f32, 0.0), Point2D::new((v * vgap) as f32, client.height)));
    }
    paint_lines(&*vlines, &DrawingConfig2D::new(Color::from_rgb(0.2, 0.2, 0.2), 1.0));
}
