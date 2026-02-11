use crate::graphics::color::Color;
use crate::graphics::geometry::orient::Orientation;
use crate::graphics::geometry::primitive::prim3d::Primitive3D;
use crate::graphics::geometry::primitive::v3d::Vertex3D;
use crate::graphics::geometry::primitive::PrimitiveType;

pub struct QuadBuilder {
    the_orientation: Option<Orientation>,
    the_width: Option<f32>,
    the_height: Option<f32>,
    the_depth: Option<f32>,

    the_color: Option<Color>,
}

impl QuadBuilder {
    pub fn new() -> QuadBuilder {
        QuadBuilder {
            the_orientation: None,
            the_width: None,
            the_height: None,
            the_depth: None,
            the_color: None,
        }
    }

    pub fn with_orientation(mut self, the_orientation: Orientation) -> QuadBuilder {
        self.the_orientation = Some(the_orientation);
        self
    }

    pub fn with_width(mut self, the_width: f32) -> QuadBuilder {
        self.the_width = Some(the_width);
        self
    }

    pub fn with_height(mut self, the_height: f32) -> QuadBuilder {
        self.the_height = Some(the_height);
        self
    }

    pub fn with_depth(mut self, the_depth: f32) -> QuadBuilder {
        self.the_depth = Some(the_depth);
        self
    }

    pub fn with_color(mut self, the_color: Color) -> QuadBuilder {
        self.the_color = Some(the_color);
        self
    }

    pub fn build(self) -> Option<Primitive3D> {
        if self.the_width == None || self.the_height == None || self.the_depth == None {
            return None;
        }

        let orientation = self
            .the_orientation
            .unwrap_or_else(|| Orientation::default());
        let color = self.the_color.unwrap_or_else(|| Color::WHITE);
        let width = self.the_width.unwrap();
        let height = self.the_height.unwrap();
        let depth = self.the_depth.unwrap();
        let mut vertices: Vec<Vertex3D> = vec![];

        /* top face */
        vertices.push(Vertex3D::new(0.0, 0.0, 0.0));
        vertices.push(Vertex3D::new(0.0, 0.0, -depth));
        vertices.push(Vertex3D::new(-width, 0.0, -depth));
        vertices.push(Vertex3D::new(-width, 0.0, 0.0));

        /* front face */
        vertices.push(Vertex3D::new(0.0, 0.0, 0.0));
        vertices.push(Vertex3D::new(-width, 0.0, 0.0));
        vertices.push(Vertex3D::new(-width, -height, 0.0));
        vertices.push(Vertex3D::new(0.0, -height, 0.0));

        /* right face */
        vertices.push(Vertex3D::new(0.0, 0.0, 0.0));
        vertices.push(Vertex3D::new(0.0, -height, 0.0));
        vertices.push(Vertex3D::new(0.0, -height, -depth));
        vertices.push(Vertex3D::new(0.0, 0.0, -depth));

        /* left face */
        vertices.push(Vertex3D::new(-width, 0.0, 0.0));
        vertices.push(Vertex3D::new(-width, 0.0, -depth));
        vertices.push(Vertex3D::new(-width, -height, -depth));
        vertices.push(Vertex3D::new(-width, -height, 0.0));

        /* bottom face */
        vertices.push(Vertex3D::new(0.0, -height, 0.0));
        vertices.push(Vertex3D::new(0.0, -height, -depth));
        vertices.push(Vertex3D::new(-width, -height, -depth));
        vertices.push(Vertex3D::new(-width, -height, 0.0));

        /* back face */
        vertices.push(Vertex3D::new(0.0, 0.0, -depth));
        vertices.push(Vertex3D::new(-width, 0.0, -depth));
        vertices.push(Vertex3D::new(-width, -height, -depth));
        vertices.push(Vertex3D::new(0.0, -height, -depth));

        Some(Primitive3D::new(PrimitiveType::Quad {}, vertices, orientation, color))
    }
}
