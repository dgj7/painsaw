use crate::graphics::color::Color;
use crate::geometry::primitive::v2d::Vertex2D;
use crate::geometry::primitive::PrimitiveType;
use crate::support::logger::log;
use crate::support::logger::log_level::LogLevel;

pub struct Primitive2D {
    pub p_type: PrimitiveType,
    pub vertices: Vec<Vertex2D>,
    pub color: Color,
}

pub struct Primitive2DBuilder {
    the_p_type: Option<PrimitiveType>,
    the_vertices: Vec<Vertex2D>,
    the_color: Option<Color>,
}

impl Primitive2D {
    pub fn new(p_type: PrimitiveType, vertices: Vec<Vertex2D>, color: Color) -> Primitive2D {
        /* warn if no vertices are supplied */
        if vertices.len() == 0 {
            log(LogLevel::Warning, &|| String::from("0 vertices specified"));
        }

        /* log other warnings */
        match p_type {
            PrimitiveType::Point { point_size: _point_size } => {}
            PrimitiveType::Line { thickness: _thickness } => {
                if vertices.len() / 2 != 0 {
                    log(LogLevel::Warning, &|| String::from(format!("lines configured with odd number of vertices: {}", vertices.len())))
                }
            }
            PrimitiveType::Quad {} => {},
            PrimitiveType::LineStrip { .. } => {  }
        }

        /* create the primitive */
        Primitive2D {
            p_type,
            vertices,
            color,
        }
    }
}

impl Primitive2DBuilder {
    pub fn new() -> Primitive2DBuilder {
        Primitive2DBuilder {
            the_p_type: None,
            the_vertices: vec!(),
            the_color: None,
        }
    }

    pub fn with_type(mut self, the_type: PrimitiveType) -> Self {
        self.the_p_type = Some(the_type);
        self
    }

    pub fn with_vertex(mut self, vertex: Vertex2D) -> Self {
        self.the_vertices.push(vertex);
        self
    }

    pub fn with_vertices(mut self, vertices: Vec<Vertex2D>) -> Self {
        self.the_vertices.extend(vertices);
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.the_color = Some(color);
        self
    }

    pub fn build(self) -> Primitive2D {
        Primitive2D {
            p_type: self.the_p_type.unwrap_or_else(|| PrimitiveType::Point {point_size: 1.0}),
            vertices: self.the_vertices,
            color: self.the_color.unwrap_or_else(|| Color::WHITE),
        }
    }
}
