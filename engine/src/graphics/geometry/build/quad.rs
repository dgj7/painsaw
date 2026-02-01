use crate::graphics::geometry::primitive::prim3d::Primitive3D;
use num_traits::Float;
use crate::graphics::color::Color;
use crate::graphics::geometry::orient::Orientation;
use crate::graphics::geometry::primitive::PrimitiveType;
use crate::graphics::geometry::primitive::v3d::Vertex3D;

pub struct QuadBuilder<F: Float> {
    the_orientation: Option<Orientation<F>>,
    the_width: Option<F>,
    the_height: Option<F>,
    the_depth: Option<F>,

    the_color: Option<Color>,
}

impl<F: Float> QuadBuilder<F> {
    pub fn new() -> QuadBuilder<F> {
        QuadBuilder {
            the_orientation: None,
            the_width: None,
            the_height: None,
            the_depth: None,
            the_color: None,
        }
    }

    pub fn with_orientation(mut self, the_orientation: Orientation<F>) -> QuadBuilder<F> {
        self.the_orientation = Some(the_orientation);
        self
    }

    pub fn with_width(mut self, the_width: F) -> QuadBuilder<F> {
        self.the_width = Some(the_width);
        self
    }

    pub fn with_height(mut self, the_height: F) -> QuadBuilder<F> {
        self.the_height = Some(the_height);
        self
    }

    pub fn with_depth(mut self, the_depth: F) -> QuadBuilder<F> {
        self.the_depth = Some(the_depth);
        self
    }
    
    pub fn with_color(mut self, the_color: Color) -> QuadBuilder<F> {
        self.the_color = Some(the_color);
        self
    }

    pub fn build(self) -> Option<Primitive3D<F>> {
        if self.the_width == None || self.the_height == None || self.the_depth == None {
            return None;
        }

        let orientation = self.the_orientation.unwrap_or_else(|| Orientation::default());
        let color = self.the_color.unwrap_or_else(|| Color::WHITE);
        let width = self.the_width.unwrap();
        let height = self.the_height.unwrap();
        let depth = self.the_depth.unwrap();
        let mut vertices: Vec<Vertex3D<F>> = vec!();

        /* top face */
        vertices.push(Vertex3D::new(F::zero(), F::zero(), F::zero()));
        vertices.push(Vertex3D::new(F::zero(), F::zero(), -depth));
        vertices.push(Vertex3D::new(-width, F::zero(), -depth));
        vertices.push(Vertex3D::new(-width, F::zero(), F::zero()));

        /* front face */
        vertices.push(Vertex3D::new(F::zero(), F::zero(), F::zero()));
        vertices.push(Vertex3D::new(-width, F::zero(), F::zero()));
        vertices.push(Vertex3D::new(-width, -height, F::zero()));
        vertices.push(Vertex3D::new(F::zero(), -height, F::zero()));

        /* right face */
        vertices.push(Vertex3D::new(F::zero(), F::zero(), F::zero()));
        vertices.push(Vertex3D::new(F::zero(), -height, F::zero()));
        vertices.push(Vertex3D::new(F::zero(), -height, -depth));
        vertices.push(Vertex3D::new(F::zero(), F::zero(), -depth));

        /* left face */
        vertices.push(Vertex3D::new(-width, F::zero(), F::zero()));
        vertices.push(Vertex3D::new(-width, F::zero(), -depth));
        vertices.push(Vertex3D::new(-width, -height, -depth));
        vertices.push(Vertex3D::new(-width, -height, F::zero()));

        /* bottom face */
        vertices.push(Vertex3D::new(F::zero(), -height, F::zero()));
        vertices.push(Vertex3D::new(F::zero(), -height, -depth));
        vertices.push(Vertex3D::new(-width, -height, -depth));
        vertices.push(Vertex3D::new(-width, -height, F::zero()));

        /* back face */
        vertices.push(Vertex3D::new(F::zero(), F::zero(), -depth));
        vertices.push(Vertex3D::new(-width, F::zero(), -depth));
        vertices.push(Vertex3D::new(-width, -height, -depth));
        vertices.push(Vertex3D::new(F::zero(), -height, -depth));

        Some(Primitive3D::new(PrimitiveType::Quad {}, vertices, orientation, color))
    }
}
