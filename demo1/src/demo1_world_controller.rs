use engine::graphics::camera::Camera;
use engine::graphics::color::Color;
use engine::graphics::geometry::primitive::p2d::Point2D;
use engine::graphics::geometry::primitive::p3d::Point3D;
use engine::graphics::geometry::primitive::prim2d::Primitive2DBuilder;
use engine::graphics::geometry::primitive::prim3d::Primitive3DBuilder;
use engine::graphics::geometry::primitive::PrimitiveType;
use engine::graphics::storage::m2d::{Model2D, Model2DBuilder};
use engine::graphics::storage::m3d::Model3DBuilder;
use engine::input::kn::KeyName;
use engine::logger::log;
use engine::logger::log_level::LogLevel;
use engine::window::context::RendererContext;
use engine::window::wc::WorldController;

static M2D_XY_PURPLE: &str = "1-2d-xy-purple";
static M2D_X_HORIZ: &str = "2-2d-x-horizontal";
static M2D_Y_VERT: &str = "2-2d-y-vertical";

pub(crate) struct Demo1WorldController {}

impl WorldController<f32> for Demo1WorldController {
    fn initialize_world_helper(&self, context: &mut RendererContext<f32>) {
        /* 2d grid: axes (in thicc purple), x, and y lines */
        context.g2d.models.insert(M2D_XY_PURPLE.parse().unwrap(), create_2d_axes(&context.camera));
        context.g2d.models.insert(M2D_X_HORIZ.parse().unwrap(), create_2d_grid_x_lines(&context.camera));
        context.g2d.models.insert(M2D_Y_VERT.parse().unwrap(), create_2d_grid_y_lines(&context.camera));

        /* 3d: origin axes */
        context.g3d.models.insert("4-3d-axes".to_string(), Model3DBuilder::new()
            .with_primitive(Primitive3DBuilder::new()
                .with_type(PrimitiveType::Point {point_size: 5.0})
                .with_color(Color::WHITE)
                .with_vertex(Point3D::new(0.0, 0.0, 0.0))
                .build())
            .with_primitive(Primitive3DBuilder::new()
                .with_type(PrimitiveType::Line {thickness: 1.0})
                .with_color(Color::RED)
                .with_vertex(Point3D::origin())
                .with_vertex(Point3D::new(0.5, 0.0, 0.0))
                .build())
            .with_primitive(Primitive3DBuilder::new()
                .with_type(PrimitiveType::Point {point_size: 5.0})
                .with_color(Color::WHITE)
                .with_vertex(Point3D::new(0.5, 0.0, 0.0))
                .build())
            .with_primitive(Primitive3DBuilder::new()
                .with_type(PrimitiveType::Line {thickness: 1.0})
                .with_color(Color::GREEN)
                .with_vertex(Point3D::origin())
                .with_vertex(Point3D::new(0.0, 0.5, 0.0))
                .build())
            .with_primitive(Primitive3DBuilder::new()
                .with_type(PrimitiveType::Point {point_size: 5.0})
                .with_color(Color::WHITE)
                .with_vertex(Point3D::new(0.0, 0.5, 0.0))
                .build())
            .with_primitive(Primitive3DBuilder::new()
                .with_type(PrimitiveType::Line {thickness: 1.0})
                .with_color(Color::BLUE)
                .with_vertex(Point3D::origin())
                .with_vertex(Point3D::new(0.0, 0.0, 0.5))
                .build())
            .with_primitive(Primitive3DBuilder::new()
                .with_type(PrimitiveType::Point {point_size: 5.0})
                .with_color(Color::WHITE)
                .with_vertex(Point3D::new(0.0, 0.0, 0.5))
                .build())
            .build());
    }

    fn update_world_helper(&self, context: &mut RendererContext<f32>) {
        match context.input.clone().lock() {
            Ok(is) => {
                /* gather some variables */
                let ccd = is.current_client_dimensions.clone();

                /* handle window resize for grid */
                if is.screen_resized {
                    context.g2d.models.entry(M2D_XY_PURPLE.parse().unwrap()).and_modify(|e| *e = create_2d_axes(&context.camera));
                    context.g2d.models.entry(M2D_X_HORIZ.parse().unwrap()).and_modify(|e| *e = create_2d_grid_x_lines(&context.camera));
                    context.g2d.models.entry(M2D_Y_VERT.parse().unwrap()).and_modify(|e| *e = create_2d_grid_y_lines(&context.camera));

                    log(LogLevel::Debug, &|| String::from(format!("window size changed ({}x{}); 2d storage count is [{}]", ccd.width, ccd.height, context.g2d.models.len())));
                }

                /* camera controls */
                if let Some(wk) = is.states.get(&KeyName::KeyW) && wk.current.is_down() {
                    context.camera.orientation.move_forward(&context.config, context.delta_time as f32);
                }
                if let Some(sk) = is.states.get(&KeyName::KeyS) && sk.current.is_down() {
                    context.camera.orientation.move_backward(&context.config, context.delta_time as f32);
                }
                if let Some(ak) = is.states.get(&KeyName::KeyA) && ak.current.is_down() {
                    context.camera.orientation.move_left(&context.config, context.delta_time as f32);
                }
                if let Some(dk) = is.states.get(&KeyName::KeyD) && dk.current.is_down() {
                    context.camera.orientation.move_right(&context.config, context.delta_time as f32);
                }
            },
            Err(_) => {
                panic!("todo: handle mutex lock failure")
            }
        }

        context.first_frame_rendered = true;
        context.frame_count += 1;
    }
}

impl Demo1WorldController {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

fn create_2d_axes(camera: &Camera<f32>) -> Model2D<f32> {
    Model2DBuilder::new()
        .with_primitive(Primitive2DBuilder::new()
            .with_type(PrimitiveType::Line {thickness: 10.0})
            .with_color(Color::from_rgb(0.498, 0.0, 1.0))
            .with_vertex(Point2D::origin())
            .with_vertex(Point2D::new(0.0, camera.height))
            .with_vertex(Point2D::origin())
            .with_vertex(Point2D::new(camera.width, 0.0))
            .build())
        .with_primitive(Primitive2DBuilder::new()
            .with_type(PrimitiveType::Point {point_size: 15.0})
            .with_color(Color::GREEN)
            .with_vertex(Point2D::origin())
            .with_vertex(Point2D::new(0.0, camera.height))
            .with_vertex(Point2D::new(camera.width, 0.0))
            .build())
        .build()
}

fn create_2d_grid_x_lines(camera: &Camera<f32>) -> Model2D<f32> {
    /* storage for vertices */
    let mut vertices = vec!();

    /* define line vertices */
    let hgap = 10;
    let hiters = ((camera.height + (hgap as f32))/(hgap as f32)) as u16;
    for h in 0..hiters {
        vertices.push(Point2D::new(0.0, (h * hgap) as f32));
        vertices.push(Point2D::new(camera.width, (h * hgap) as f32));
    }

    /* done */
    Model2DBuilder::new()
        .with_primitive(Primitive2DBuilder::new()
            .with_type(PrimitiveType::Line {thickness: 1.0})
            .with_color(Color::from_rgb(0.2, 0.2, 0.2))
            .with_vertices(vertices)
            .build())
        .build()
}

fn create_2d_grid_y_lines(camera: &Camera<f32>) -> Model2D<f32> {
    /* storage for vertices */
    let mut vertices = vec!();

    /* define line vertices */
    let vgap = 10;
    let viters = ((camera.width + (vgap as f32))/(vgap as f32)) as u16;
    for v in 0..viters {
        vertices.push(Point2D::new((v * vgap) as f32, 0.0));
        vertices.push(Point2D::new((v * vgap) as f32, camera.height));
    }

    /* done */
    Model2DBuilder::new()
        .with_primitive(Primitive2DBuilder::new()
            .with_type(PrimitiveType::Line {thickness: 1.0})
            .with_color(Color::from_rgb(0.2, 0.2, 0.2))
            .with_vertices(vertices)
            .build())
        .build()
}
