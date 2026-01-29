use crate::graphics::color::Color;
use crate::graphics::geometry::primitive::v2d::Vertex2D;
use crate::graphics::geometry::primitive::PrimitiveType;
use crate::logger::log;
use crate::logger::log_level::LogLevel;
use num_traits::Float;

pub struct Primitive2D<F: Float> {
    pub p_type: PrimitiveType<F>,
    pub vertices: Vec<Vertex2D<F>>,
    pub color: Color,
}

pub struct Primitive2DBuilder<F: Float> {
    the_p_type: Option<PrimitiveType<F>>,
    the_vertices: Vec<Vertex2D<F>>,
    the_color: Option<Color>,
}

impl<F: Float> Primitive2D<F> {
    pub fn new(p_type: PrimitiveType<F>, vertices: Vec<Vertex2D<F>>, color: Color) -> Primitive2D<F> {
        if vertices.len() == 0 {
            log(LogLevel::Warning, &|| String::from("0 vertices specified"));
        }

        match p_type {
            PrimitiveType::Point { point_size: _point_size } => {}
            PrimitiveType::Line { thickness: _thickness } => {
                if vertices.len() / 2 != 0 {
                    log(LogLevel::Warning, &|| String::from(format!("lines configured with odd number of vertices: {}", vertices.len())))
                }
            }
        }

        Primitive2D {
            p_type,
            vertices,
            color,
        }
    }
}

impl<F: Float> Primitive2DBuilder<F> {
    pub fn new() -> Primitive2DBuilder<F> {
        Primitive2DBuilder {
            the_p_type: None,
            the_vertices: vec!(),
            the_color: None,
        }
    }

    pub fn with_type(mut self, the_type: PrimitiveType<F>) -> Self {
        self.the_p_type = Some(the_type);
        self
    }

    pub fn with_vertex(mut self, vertex: Vertex2D<F>) -> Self {
        self.the_vertices.push(vertex);
        self
    }

    pub fn with_vertices(mut self, vertices: Vec<Vertex2D<F>>) -> Self {
        self.the_vertices.extend(vertices);
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.the_color = Some(color);
        self
    }

    pub fn build(self) -> Primitive2D<F> {
        Primitive2D {
            p_type: self.the_p_type.unwrap_or_else(|| PrimitiveType::Point {point_size: F::one()}),
            vertices: self.the_vertices,
            color: self.the_color.unwrap_or_else(|| Color::WHITE),
        }
    }
}
