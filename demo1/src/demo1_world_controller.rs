use engine::fileio::image::raw::{Pixel, RawImage};
use engine::fileio::image::tex::t2d::Texture2D;
use engine::fileio::resources::memory::MemoryResource;
use engine::geometry::dim::d2d::Dimension2D;
use engine::geometry::line::l2d::Line2D;
use engine::geometry::line::l3d::Line3D;
use engine::geometry::line::ls2d::Lines2D;
use engine::geometry::line::ls3d::Lines3D;
use engine::geometry::storage::m2d::Model2D;
use engine::geometry::storage::m3d::Model3D;
use engine::geometry::vector::p2d::Point2D;
use engine::geometry::vector::p3d::Point3D;
use engine::geometry::vector::ps2d::Points2D;
use engine::geometry::vector::ps3d::Points3D;
use engine::input::model::keyboard_state::{KeyInfo, KeyPosition};
use engine::logger::log;
use engine::logger::log_level::LogLevel;
use engine::graphics::model::color::Color;
use engine::window::wc::context::RendererContext;
use engine::window::wc::world_control::WorldController;

static M2D_XY_PURPLE: &str = "2d-xy-purple";
static M2D_X_HORIZ: &str = "2d-x-horizontal";
static M2D_Y_VERT: &str = "2d-y-vertical";
static M3D_X_ABSCISSA: &str = "3d-axes-abscissa";
static M3D_Y_ORDINATE: &str = "3d-axes-ordinate";
static M3D_Z_APPLICATE: &str = "3d-axes-applicate";
static M2D_LETTER_A_TEXTURE: &str = "2d-letter-a-texture";

pub(crate) struct Demo1WorldController {}

impl WorldController<f32> for Demo1WorldController {
    fn initialize_world_helper(&self, context: &mut RendererContext<f32>) {
        /* gather variables */
        let ccd = context.copy_client_dimensions();

        /* 2d grid: axes (in thicc purple), x, and y lines */
        context.g2d.models.insert(M2D_XY_PURPLE.parse().unwrap(), create_2d_axes(&ccd));
        context.g2d.models.insert(M2D_X_HORIZ.parse().unwrap(), create_2d_grid_x_lines(&ccd));
        context.g2d.models.insert(M2D_Y_VERT.parse().unwrap(), create_2d_grid_y_lines(&ccd));

        /* 2d textures */
        context.g2d.models.insert(M2D_LETTER_A_TEXTURE.parse().unwrap(), create_2d_letter_a());

        /* 3d: origin axes */
        context.g3d.models.insert(M3D_X_ABSCISSA.parse().unwrap(), Model3D::new(
            vec!(Points3D::new(vec!(Point3D::origin(), Point3D::new(0.5, 0.0, 0.0)), Color::WHITE, 5.0)),
            vec!(Lines3D::new(vec!(Line3D::new(Point3D::origin(), Point3D::new(0.5, 0.0, 0.0))), Color::RED, 1.0)))
        );
        context.g3d.models.insert(M3D_Y_ORDINATE.parse().unwrap(), Model3D::new(
            vec!(Points3D::new(vec!(Point3D::new(0.0, 0.5, 0.0)), Color::WHITE, 5.0)),
            vec!(Lines3D::new(vec!(Line3D::new(Point3D::origin(), Point3D::new(0.0, 0.5, 0.0))), Color::GREEN, 1.0)))
        );
        context.g3d.models.insert(M3D_Z_APPLICATE.parse().unwrap(), Model3D::new(
            vec!(Points3D::new(vec!(Point3D::new(0.0, 0.0, 0.5)), Color::WHITE, 5.0)),
            vec!(Lines3D::new(vec!(Line3D::new(Point3D::origin(), Point3D::new(0.0, 0.0, 0.5))), Color::BLUE, 1.0)))
        );
    }

    fn update_world(&self, context: &mut RendererContext<f32>) {
        match context.input.clone().lock() {
            Ok(mut is) => {
                /* gather some variables */
                let ccd = is.current_client_dimensions.clone();

                /*  handle g key up/down */
                let duration = is.g_key.previous_key_state_duration();
                match &is.g_key.current {
                    KeyPosition::KeyDown { info } => {
                        if !info.handled {
                            log(LogLevel::Debug, &|| String::from(format!("G: DOWN    (up for {}ms)", duration.as_millis())));
                            is.g_key.current = KeyPosition::KeyDown { info: KeyInfo { when: info.when, handled: true, } };
                        }
                    }
                    KeyPosition::KeyUp { info } => {
                        if !info.handled {
                            log(LogLevel::Debug, &|| String::from(format!("G: UP      (down for {}ms)", duration.as_millis())));
                            is.g_key.current = KeyPosition::KeyUp { info: KeyInfo { when: info.when, handled: true } };
                        }
                    }
                }

                /* handle window resize for grid */
                if is.screen_resized {
                    context.g2d.models.entry(M2D_XY_PURPLE.parse().unwrap()).and_modify(|e| *e = create_2d_axes(&ccd));
                    context.g2d.models.entry(M2D_X_HORIZ.parse().unwrap()).and_modify(|e| *e = create_2d_grid_x_lines(&ccd));
                    context.g2d.models.entry(M2D_Y_VERT.parse().unwrap()).and_modify(|e| *e = create_2d_grid_y_lines(&ccd));

                    log(LogLevel::Debug, &|| String::from(format!("window size changed; 2d model count is [{}]", context.g2d.models.len())));
                }
                is.screen_resized = false;
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

fn create_2d_axes(ccd: &Dimension2D<f32>) -> Model2D<f32> {
    /* define points */
    let axes_points = Points2D::new(vec!(
            Point2D::origin(),
            Point2D::new(0.0, ccd.height),
            Point2D::new(ccd.width, 0.0),
        ), Color::GREEN,15.0);
    let other_points = Points2D::new(vec!(
            Point2D::new(100.0, 100.0),
        ), Color::GREEN, 3.0);
    let pointsvec = vec!(axes_points, other_points);

    /* define lines */
    let axes = vec!(Line2D::new(Point2D::origin(), Point2D::new(0.0, ccd.height)), Line2D::new(Point2D::origin(), Point2D::new(ccd.width, 0.0)));
    let axeslines = vec!(Lines2D::new(axes, Color::from_rgb(0.498, 0.0, 1.0), 10.0));

    /* done */
    Model2D::new(pointsvec, axeslines, vec!())
}

fn create_2d_grid_x_lines(ccd: &Dimension2D<f32>) -> Model2D<f32> {
    /* define lines */
    let hgap = 10;
    let hiters = ((ccd.height + (hgap as f32))/(hgap as f32)) as u16;
    let mut hlines: Vec<Line2D<f32>> = Vec::with_capacity((hiters + 10) as usize);
    for h in 0..hiters {
        hlines.push(Line2D::new(Point2D::new(0.0, (h * hgap) as f32), Point2D::new(ccd.width, (h * hgap) as f32)));
    }
    let hlinevec = vec!(Lines2D::new(hlines, Color::from_rgb(0.2, 0.2, 0.2), 1.0));

    /* done */
    Model2D::new(vec!(), hlinevec, vec!())
}

fn create_2d_grid_y_lines(ccd: &Dimension2D<f32>) -> Model2D<f32> {
    /* define lines */
    let vgap = 10;
    let viters = ((ccd.width + (vgap as f32))/(vgap as f32)) as u16;
    let mut vlines: Vec<Line2D<f32>> = Vec::with_capacity((viters + 10) as usize);
    for v in 0..viters {
        vlines.push(Line2D::new(Point2D::new((v * vgap) as f32, 0.0), Point2D::new((v * vgap) as f32, ccd.height)));
    }
    let vlinevec = vec!(Lines2D::new(vlines, Color::from_rgb(0.2, 0.2, 0.2), 1.0));

    /* done */
    Model2D::new(vec!(), vlinevec, vec!())
}

fn create_2d_letter_a() -> Model2D<f32> {
    let bitmap: [u8; 7] = [0x6f, 0x49, 0xc9, 0x7b, 0x00, 0x00, 0x00];
    let letter = MemoryResource::from_array(Vec::from(bitmap));
    let image = RawImage::one_bpp_to_raw_img(8, 8, Box::new(letter), &Color::WHITE, &Color::BLACK, Pixel::bit_per_pixel);
    let textures = vec!(Texture2D::new(image, Point2D::new(100.0, 300.0), 10.0));

    //for byte in textures[0].image.data.iter() {
    //    log(LogLevel::Info, &|| String::from(format!("{}", byte)));
    //}

    Model2D::new(vec!(), vec!(), textures)
}
