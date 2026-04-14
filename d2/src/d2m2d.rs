use engine::geometry::primitive::prim2d::Primitive2DBuilder;
use engine::geometry::primitive::v2d::Vertex2D;
use engine::geometry::primitive::PrimitiveType;
use engine::graphics::color::Color;
use engine::graphics::storage::m2d::{Model2D, Model2DBuilder};
use engine::graphics::texture::t2d::Texture2D;
use engine::support::image::bitmap::Bitmap;
use engine::support::image::Image;
use engine::support::image::targa::Targa;

const B24_BMP: &[u8] = include_bytes!("../assets/img1_24b.bmp");
const TGA: &[u8] = include_bytes!("../assets/img1_32b.tga");

pub(super) fn create_2d_bmp_24b() -> Model2D {
    let bmp_24b_x_zero = 15.0;
    let bmp_24b_y_zero = 50.0;
    let bmp_24b = Bitmap::load_from_bytes(B24_BMP).expect("failed to load ../assets/img1_24b.bmp");
    let bmp_24b_width = bmp_24b.width as f32;
    let bmp_24b_height = bmp_24b.height as f32;

    Model2DBuilder::new()
        .with_texture(Texture2D::new(bmp_24b, bmp_24b_x_zero, bmp_24b_y_zero, 1.0))
        .with_primitive(Primitive2DBuilder::new()
            .with_type(PrimitiveType::Point { point_size: 12.0 })
            .with_color(Color::BLUE)
            .with_vertex(Vertex2D::new(bmp_24b_x_zero + bmp_24b_width, bmp_24b_y_zero + bmp_24b_height))
            .build())
        .build()
}

pub(super) fn create_2d_tga() -> Model2D {
    let tga_x_zero = 200.0;
    let tga_y_zero = 50.0;
    let tga = Targa::load_from_bytes(TGA).expect("failed to load ../assets/img1_32b.tga");
    let tga_width = tga.width as f32;
    let tga_height = tga.height as f32;

    Model2DBuilder::new()
        .with_texture(Texture2D::new(tga, tga_x_zero, tga_y_zero, 1.0))
        .with_primitive(Primitive2DBuilder::new()
            .with_type(PrimitiveType::Point { point_size: 12.0 })
            .with_color(Color::BLUE)
            .with_vertex(Vertex2D::new(tga_x_zero + tga_width, tga_y_zero + tga_height))
            .build())
        .with_primitive(Primitive2DBuilder::new()
            .with_type(PrimitiveType::Line {thickness: 5.0})
            .with_color(Color::RED)
            .with_vertex(Vertex2D::new(tga_x_zero + tga_width, tga_y_zero + tga_height))
            .with_vertex(Vertex2D::new(tga_x_zero, tga_y_zero + tga_height))
            .build())
        .with_primitive(Primitive2DBuilder::new()
            .with_type(PrimitiveType::Line {thickness: 5.0})
            .with_color(Color::RED)
            .with_vertex(Vertex2D::new(tga_x_zero + tga_width, tga_y_zero))
            .with_vertex(Vertex2D::new(tga_x_zero + tga_width, tga_y_zero + tga_height))
            .build())
        .build()
}
