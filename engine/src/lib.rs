use std::sync::{Arc, Mutex};
use crate::config::EngineConfig;
use crate::config::input_config::kc::handle_key_change;
use crate::config::input_config::mc::handle_mouse_change;
use crate::graphics::camera::Camera;
use crate::graphics::GraphicsIntermediary;
use crate::graphics::storage::g2d::Graph2D;
use crate::graphics::storage::g3d::Graph3D;
use crate::input::screen::ScreenState;
use crate::input::UserInput;
use crate::support::logger::log;
use crate::support::logger::log_level::LogLevel;
use crate::support::timing::EngineTiming;
use crate::window::key::WindowKey;

pub mod config;
pub mod graphics;
pub mod input;
pub mod window;
pub mod support;
pub mod geometry;

///
/// core context object used by the engine.
///
/// this data is passed through functions and ultimately to the end user, for access
/// to various engine configurations and states.
///
pub struct PainsawContext {
    /* scene for game statistics */
    pub first_frame_rendered: bool,
    pub frame_count: u128,

    /* timing */
    pub timing: EngineTiming,

    /* scene for world state */
    pub g2d: Graph2D,
    pub g3d: Graph3D,
    pub camera: Camera,

    /* rendering subsystem */
    pub(crate) graphics: GraphicsIntermediary,

    /* scene for input state */
    pub input: Arc<Mutex<UserInput>>,
    pub config: EngineConfig,
    pub screen: ScreenState,
}

impl PainsawContext {
    pub(crate) fn new(input: &Arc<Mutex<UserInput>>, config: EngineConfig, screen: ScreenState) -> PainsawContext {
        let dim = &screen.current_client_dimensions;
        log(LogLevel::Info, &|| String::from(format!("initializing camera with width={},height={}", &dim.width, &dim.height)));
        PainsawContext {
            first_frame_rendered: false,
            frame_count: 0,

            timing: EngineTiming::new(&config.renderer),

            g2d: Graph2D::new(),
            g3d: Graph3D::new(),
            camera: Camera::new(&dim),

            graphics: GraphicsIntermediary::new(config.renderer.graphics.clone()),

            input: input.clone(),
            config,
            screen,
        }
    }
}

///
/// Control various aspects of the world, as called by the windowing system.
///
/// Because of tight integration between the windowing system and the renderer
/// (opengl, directx, et al.), this trait becomes the interaction between
/// the two and our custom engine.  Everything our engine does can ultimately be
/// traced back to this trait.
///
/// Since this is a trait, it is/will be required that any game using the
/// Painsaw engine create their own world controller, implementing the abstract
/// unimplemented functions below.
///
pub trait WorldController {
    ///
    /// initialize the game world.
    ///
    fn initialize_world(&self, context: &mut PainsawContext) {
        self.initialize_world_helper(context);

        let graphics = &mut context.graphics;
        graphics.initialize(&mut context.g2d, &mut context.g3d);

        log(LogLevel::Debug, &|| String::from("initialization complete"));
    }

    ///
    /// initialize game world - customizer for client.
    ///
    fn initialize_world_helper(&self, context: &mut PainsawContext);

    ///
    /// update the game world state - fully controlled by client.
    ///
    fn update_world(&self, context: &mut PainsawContext, key: &WindowKey) {
        match context.input.clone().lock() {
            Ok(mut uin) => {
                /* handle key changes */
                while !uin.key_changes.is_empty() {
                    let change = uin.key_changes.pop_front().unwrap();
                    let state = uin.key_states.get_mut(&change).unwrap();
                    if !state.current.is_handled() {
                        handle_key_change(context.config.input.key_handler.clone(), &change, state, &mut context.camera, &context.config, &context.timing);
                        state.current.set_handled();
                    }
                }

                /* check key states */
                context.config.input.key_handler.clone().check_key_states(&uin.key_states, &mut context.camera, &context.config, &context.timing);

                /* handle screen resize */
                if uin.screen_resized {
                    context.screen.update(key);
                    context.camera.update_screen(&context.screen.current_client_dimensions);
                    context.graphics.resize(context);
                }

                /* handle mouse changes */
                while !uin.mouse_changes.is_empty() {
                    let change = uin.mouse_changes.pop_front().unwrap();
                    let state = uin.mouse_states.get_mut(&change).unwrap();
                    if !state.current.handled {
                        handle_mouse_change(context.config.input.mouse_handler.clone(), &change, state, &mut context.camera, &context.config, &context.timing, &mut context.screen);
                        state.current.handled = true;
                    }
                }
            }
            Err(_) => {}
        }

        self.update_world_helper(context);

        match context.input.lock() {
            Ok(mut uin) => {
                uin.screen_resized = false;
            }
            Err(_) => {panic!("todo: resetting screen_resized")}
        }
    }

    fn update_world_helper(&self, context: &mut PainsawContext);

    ///
    /// display the game world scene.
    ///
    /// fully controlled by engine; the engine is data-driven, meaning that graphics instructions
    /// come from models supplied during initialization, along with changes to those models
    /// during the update world step.
    ///
    fn display_world_scene(&self, context: &mut PainsawContext) {
        /* gather variables */
        let uin = context.input.lock().unwrap();
        let screen = &context.screen;

        /* prepare for drawing */
        context.graphics.before_scene(&context.camera);

        /* draw 3d, if desired */
        context.graphics.prepare_3d(&context);
        context.graphics.render_3d(&mut context.g3d);
        context.graphics.after_3d(&context);

        /* draw 2d, if desired */
        context.graphics.prepare_2d(&mut context.g2d, &context.camera);
        context.graphics.render_2d(&mut context.g2d, &context.timing, &context.config, &context.camera, uin, &screen);
        context.graphics.after_2d();
    }
}