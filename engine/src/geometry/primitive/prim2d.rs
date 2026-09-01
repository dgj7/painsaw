use crate::geometry::primitive::face::PolygonFace;
use crate::geometry::primitive::mode::PolygonMode;
use crate::geometry::primitive::v2d::Vertex2D;
use crate::geometry::primitive::PrimitiveType;
use crate::graphics::color::Color;
use crate::support::logger::log;
use crate::support::logger::log_level::LogLevel;

pub struct Primitive2D {
    pub p_type: PrimitiveType,
    pub vertices: Vec<Vertex2D>,
    pub color: Color,
    pub mode: PolygonMode,
    pub face: PolygonFace,
}

pub struct Primitive2DBuilder {
    the_p_type: Option<PrimitiveType>,
    the_vertices: Vec<Vertex2D>,
    the_color: Option<Color>,
    the_mode: Option<PolygonMode>,
    the_face: Option<PolygonFace>,
}

impl Primitive2D {
    pub fn new(p_type: PrimitiveType, vertices: Vec<Vertex2D>, color: Color, mode: PolygonMode, face: PolygonFace) -> Primitive2D {
        /* warn if no vertices are supplied */
        if vertices.len() == 0 {
            log(LogLevel::Warning, &|| String::from("0 vertices specified"));
        }

        /* log other warnings */
        match p_type {
            PrimitiveType::Point { point_size: _point_size, } => {}
            PrimitiveType::Line { thickness: _thickness, } => {
                if vertices.len() / 2 != 0 {
                    log(LogLevel::Warning, &|| { String::from(format!("lines configured with odd number of vertices: {}", vertices.len())) })
                }
            }
            PrimitiveType::Cube {} => {}
            PrimitiveType::LineStrip { .. } => {}
        }

        /* create the primitive */
        Primitive2D {
            p_type,
            vertices,
            color,
            mode,
            face,
        }
    }
}

impl Primitive2DBuilder {
    pub fn new() -> Primitive2DBuilder {
        Primitive2DBuilder {
            the_p_type: None,
            the_vertices: vec![],
            the_color: None,
            the_mode: None,
            the_face: None,
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

    pub fn with_mode(mut self, mode: PolygonMode) -> Self {
        self.the_mode = Some(mode);
        self
    }

    pub fn with_face(mut self, face: PolygonFace) -> Self {
        self.the_face = Some(face);
        self
    }

    pub fn build(self) -> Primitive2D {
        Primitive2D {
            p_type: self.the_p_type.unwrap_or_else(|| PrimitiveType::Point { point_size: 1.0 }),
            vertices: self.the_vertices,
            color: self.the_color.unwrap_or_else(|| Color::WHITE),
            mode: self.the_mode.unwrap_or_else(|| PolygonMode::Line),
            face: self.the_face.unwrap_or_else(|| PolygonFace::FrontAndBack),
        }
    }
}
