use engine::geometry::primitive::prim2d::Primitive2DBuilder;
use engine::geometry::primitive::v2d::Vertex2D;
use engine::geometry::primitive::PrimitiveType;
use engine::graphics::color::Color;
use engine::graphics::storage::m2d::{Model2D, Model2DBuilder};
use engine::graphics::texture::t2d::Texture2D;
use engine::support::image::bitmap::load_bitmap_from_bytes;

const B24_BMP: &[u8] = include_bytes!("../assets/24b.bmp");

pub(super) fn create_2d_bmp_24b() -> Model2D {
    let bmp_24b_x_zero = 150.0;
    let bmp_24b_y_zero = 150.0;
    let bmp_24b = load_bitmap_from_bytes(B24_BMP).expect("failed to load ../assets/24b.bmp");
    let bmp_24b_width = bmp_24b.width as f32;
    let bmp_24b_height = bmp_24b.height as f32;

    Model2DBuilder::new()
        .with_texture(Texture2D::new(bmp_24b, bmp_24b_x_zero, bmp_24b_y_zero, 1.0))
        .with_primitive(Primitive2DBuilder::new()
            .with_type(PrimitiveType::Point { point_size: 6.0 })
            .with_color(Color::BLUE)
            .with_vertex(Vertex2D::new(bmp_24b_x_zero + bmp_24b_width, bmp_24b_y_zero + bmp_24b_height))
            .build())
        /*
        .with_primitive(Primitive2DBuilder::new()
            .with_type(PrimitiveType::LineStrip { thickness: 1.0 })
            .with_color(Color::BLUE)
            .with_vertex(Vertex2D::new(bmp_24b_x_zero, bmp_24b_y_zero))
            .with_vertex(Vertex2D::new(bmp_24b_x_zero + bmp_24b_width, bmp_24b_y_zero))
            .with_vertex(Vertex2D::new(bmp_24b_x_zero + bmp_24b_width, bmp_24b_y_zero + bmp_24b_height))
            .with_vertex(Vertex2D::new(bmp_24b_x_zero, bmp_24b_y_zero + bmp_24b_height))
            .with_vertex(Vertex2D::new(bmp_24b_x_zero, bmp_24b_y_zero))
            .build())
        */
        .build()
}
