use crate::geometry::orient::Orientation;
use crate::geometry::primitive::face::PolygonFace;
use crate::geometry::primitive::mode::PolygonMode;
use crate::geometry::primitive::prim3d::Primitive3D;
use crate::geometry::primitive::v3d::Vertex3D;
use crate::geometry::primitive::PrimitiveType;
use crate::graphics::color::Color;

pub struct CubeBuilder {
    the_orientation: Option<Orientation>,
    the_width: Option<f32>,
    the_height: Option<f32>,
    the_depth: Option<f32>,
    the_color: Option<Color>,
    the_mode: Option<PolygonMode>,
    the_face: Option<PolygonFace>,
}

impl CubeBuilder {
    pub fn new() -> CubeBuilder {
        CubeBuilder {
            the_orientation: None,
            the_width: None,
            the_height: None,
            the_depth: None,
            the_color: None,
            the_mode: None,
            the_face: None,
        }
    }

    pub fn with_orientation(mut self, the_orientation: Orientation) -> CubeBuilder {
        self.the_orientation = Some(the_orientation);
        self
    }

    pub fn with_width(mut self, the_width: f32) -> CubeBuilder {
        self.the_width = Some(the_width);
        self
    }

    pub fn with_height(mut self, the_height: f32) -> CubeBuilder {
        self.the_height = Some(the_height);
        self
    }

    pub fn with_depth(mut self, the_depth: f32) -> CubeBuilder {
        self.the_depth = Some(the_depth);
        self
    }

    pub fn with_color(mut self, the_color: Color) -> CubeBuilder {
        self.the_color = Some(the_color);
        self
    }
    
    pub fn with_mode(mut self, mode: PolygonMode) -> CubeBuilder {
        self.the_mode = Some(mode);
        self
    }

    pub fn with_face(mut self, face: PolygonFace) -> CubeBuilder {
        self.the_face = Some(face);
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
        let mode = self.the_mode.unwrap_or_else(|| PolygonMode::Line);
        let face = self.the_face.unwrap_or_else(|| PolygonFace::FrontAndBack);
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

        Some(Primitive3D::new(PrimitiveType::Cube {}, vertices, orientation, color, mode, face))
    }
}
