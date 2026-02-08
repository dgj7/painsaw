use crate::graphics::color::Color;
use crate::graphics::geometry::orient::Orientation;
use crate::graphics::geometry::primitive::v3d::Vertex3D;
use crate::graphics::geometry::primitive::PrimitiveType;
use crate::logger::log;
use crate::logger::log_level::LogLevel;

///
/// simple shapes that can be drawn by various low-level rendering apis.
///
pub struct Primitive3D {
    pub ptype: PrimitiveType,
    pub vertices: Vec<Vertex3D>,
    pub orientation: Orientation,
    pub color: Color,
}

pub struct Primitive3DBuilder {
    the_type: Option<PrimitiveType>,
    the_vertices: Vec<Vertex3D>,
    the_orientation: Option<Orientation>,
    the_color: Option<Color>,
}

impl Primitive3D {
    pub fn new(ptype: PrimitiveType, vertices: Vec<Vertex3D>, orientation: Orientation, color: Color) -> Primitive3D {
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
            PrimitiveType::Quad {} => {},
            PrimitiveType::LineStrip { .. } => {}
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

impl Primitive3DBuilder {
    pub fn new() -> Primitive3DBuilder {
        Primitive3DBuilder{
            the_type: None,
            the_vertices: vec!(),
            the_orientation: None,
            the_color: None,
        }
    }

    pub fn with_type(mut self, the_type: PrimitiveType) -> Self {
        self.the_type = Some(the_type);
        self
    }

    pub fn with_vertex(mut self, vertex: Vertex3D) -> Self {
        self.the_vertices.push(vertex);
        self
    }

    pub fn with_orientation(mut self, orientation: Orientation) -> Self {
        self.the_orientation = Some(orientation);
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.the_color = Some(color);
        self
    }

    pub fn build(self) -> Primitive3D {
        Primitive3D {
            ptype: self.the_type.unwrap_or_else(|| PrimitiveType::Point {point_size: 1.0}),
            vertices: self.the_vertices,
            orientation: self.the_orientation.unwrap_or_else(|| Orientation::default()),
            color: self.the_color.unwrap_or_else(|| Color::WHITE),
        }
    }
}
