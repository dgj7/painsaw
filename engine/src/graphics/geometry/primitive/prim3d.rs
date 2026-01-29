use num_traits::Float;
use crate::graphics::color::Color;
use crate::graphics::geometry::orient::Orientation;
use crate::graphics::geometry::primitive::p3d::Point3D;
use crate::graphics::geometry::primitive::PrimitiveType;
use crate::logger::log;
use crate::logger::log_level::LogLevel;

///
/// simple shapes that can be drawn by various low-level rendering apis.
///
pub struct Primitive3D<F: Float> {
    pub ptype: PrimitiveType<F>,
    pub vertices: Vec<Point3D<F>>,
    pub orientation: Orientation<F>,
    pub color: Color,
}

pub struct Primitive3DBuilder<F: Float> {
    the_type: Option<PrimitiveType<F>>,
    the_vertices: Vec<Point3D<F>>,
    the_orientation: Option<Orientation<F>>,
    the_color: Option<Color>,
}

impl<F: Float> Primitive3D<F> {
    pub fn new(ptype: PrimitiveType<F>, vertices: Vec<Point3D<F>>, orientation: Orientation<F>, color: Color) -> Primitive3D<F> {
        /* warn for no vertices */
        if vertices.len() == 0 {
            log(LogLevel::Warning, &|| String::from("0 vertices specified"));
        }

        /* print warnings based on content of vertices */
        match ptype {
            PrimitiveType::Point { point_size: _point_size } => {}
            PrimitiveType::Line { thickness: _thickness } => {
                if vertices.len() / 2 != 0 {
                    log(LogLevel::Warning, &|| String::from(format!("lines configured with odd number of vertices: {}", vertices.len())))
                }
            }
        }

        /* done */
        Primitive3D {
            ptype,
            vertices,
            orientation,
            color
        }
    }
}

impl<F: Float> Primitive3DBuilder<F> {
    pub fn new() -> Primitive3DBuilder<F> {
        Primitive3DBuilder{
            the_type: None,
            the_vertices: vec!(),
            the_orientation: None,
            the_color: None,
        }
    }

    pub fn with_type(mut self, the_type: PrimitiveType<F>) -> Self {
        self.the_type = Some(the_type);
        self
    }

    pub fn with_vertex(mut self, vertex: Point3D<F>) -> Self {
        self.the_vertices.push(vertex);
        self
    }

    pub fn with_orientation(mut self, orientation: Orientation<F>) -> Self {
        self.the_orientation = Some(orientation);
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.the_color = Some(color);
        self
    }

    pub fn build(self) -> Primitive3D<F> {
        Primitive3D {
            ptype: self.the_type.unwrap_or_else(|| PrimitiveType::Point {point_size: F::one()}),
            vertices: self.the_vertices,
            orientation: self.the_orientation.unwrap_or_else(|| Orientation::default()),
            color: self.the_color.unwrap_or_else(|| Color::WHITE),
        }
    }
}
