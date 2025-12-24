use crate::math::draw_config::DrawingConfig;
use crate::math::threed::line_3d::Line3D;
use crate::math::threed::point_3d::Point3D;
use crate::math::twod::dimension_2d::Dimension2D;
use crate::math::twod::line_2d::Line2D;
use crate::math::twod::point_2d::Point2D;
use crate::render::graphics::opengl::opengl_wrapper_2d::paint_2d_lines;
use crate::render::graphics::opengl::opengl_wrapper_3d::paint_3d_lines;
use crate::render::model::color::Color;

pub fn paint_grid(client: &Dimension2D<f32>) {
    /* draw x&y axes in white */
    paint_2d_lines(&*vec!(
        Line2D::new(Point2D::origin(), Point2D::new(0.0, client.height)),
        Line2D::new(Point2D::origin(), Point2D::new(client.width, 0.0)),
    ), &DrawingConfig::new(Color::from_rgb(0.498, 0.0, 1.0), 10.0));

    /* draw x-axis horizontal lines */
    let hgap = 10;
    let hiters = ((client.height + (hgap as f32))/(hgap as f32)) as u16;
    let mut hlines: Vec<Line2D<f32>> = Vec::with_capacity((hiters + 10) as usize);
    for h in 0..hiters {
        hlines.push(Line2D::new(Point2D::new(0.0, (h * hgap) as f32), Point2D::new(client.width, (h * hgap) as f32)));
    }
    paint_2d_lines(&*hlines, &DrawingConfig::new(Color::from_rgb(0.2, 0.2, 0.2), 1.0));

    /* draw y-axis vertical lines */
    let vgap = 10;
    let viters = ((client.width + (vgap as f32))/(vgap as f32)) as u16;
    let mut vlines: Vec<Line2D<f32>> = Vec::with_capacity((viters + 10) as usize);
    for v in 0..viters {
        vlines.push(Line2D::new(Point2D::new((v * vgap) as f32, 0.0), Point2D::new((v * vgap) as f32, client.height)));
    }
    paint_2d_lines(&*vlines, &DrawingConfig::new(Color::from_rgb(0.2, 0.2, 0.2), 1.0));
}

pub fn paint_axes() {
    let distance: f32 = 100.0;

    let x_abscissa = Line3D::new(Point3D::origin(), Point3D::new(distance, 0.0, 0.0));
    let y_ordinate = Line3D::new(Point3D::origin(), Point3D::new(0.0, distance, 0.0));
    let z_applicate = Line3D::new(Point3D::origin(), Point3D::new(0.0, 0.0, distance));

    paint_3d_lines(&[x_abscissa], &DrawingConfig::new(Color::RED, 1.0));
    paint_3d_lines(&[y_ordinate], &DrawingConfig::new(Color::GREEN, 1.0));
    paint_3d_lines(&[z_applicate], &DrawingConfig::new(Color::BLUE, 1.0));
}
