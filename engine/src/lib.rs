use crate::config::input_config::kc::{handle_key_change, KeyHandler};
use crate::config::input_config::mc::{handle_mouse_change, MouseHandler};
use crate::config::EngineConfig;
use crate::graphics::camera::Camera;
use crate::graphics::storage::g2d::Graph2D;
use crate::graphics::storage::g3d::Graph3D;
use crate::graphics::GraphicsIntermediary;
use crate::input::screen::ScreenState;
use crate::input::UserInput;
use crate::support::logger::log;
use crate::support::logger::log_level::LogLevel;
use crate::support::timing::EngineTiming;
use crate::window::key::WindowKey;
use std::sync::{Arc, Mutex};

pub mod config;
pub mod geometry;
pub mod graphics;
pub mod input;
pub mod support;
pub mod window;

///
/// core context object used by the engine.
///
/// this data is passed through functions and ultimately to the end user, for access
/// to various engine configurations and states.
///
pub struct PainsawContext {
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
    pub(crate) fn new(
        input: &Arc<Mutex<UserInput>>,
        config: EngineConfig,
        screen: ScreenState,
    ) -> PainsawContext {
        let dim = &screen.current_client_dimensions;
        log(LogLevel::Info, &|| {
            String::from(format!(
                "initializing camera with width={},height={}",
                &dim.width, &dim.height
            ))
        });
        PainsawContext {
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
    fn initialize_world(
        &self,
        graphics: &mut GraphicsIntermediary,
        camera: &Camera,
        g2d: &mut Graph2D,
        g3d: &mut Graph3D,
    ) {
        self.initialize_world_helper(camera, g2d, g3d);

        graphics.initialize(g2d, g3d);

        log(LogLevel::Debug, &|| String::from("initialization complete"));
    }

    ///
    /// initialize game world - customizer for client.
    ///
    fn initialize_world_helper(&self, camera: &Camera, g2d: &mut Graph2D, g3d: &mut Graph3D);

    ///
    /// update the game world state - fully controlled by client.
    ///
    fn update_world<T: KeyHandler + MouseHandler + WorldController + 'static>(
        &self,
        game: &T,
        input: Arc<Mutex<UserInput>>,
        camera: &mut Camera,
        config: &EngineConfig,
        screen: &mut ScreenState,
        timing: &mut EngineTiming,
        graphics: &GraphicsIntermediary,
        g2d: &mut Graph2D,
        g3d: &mut Graph3D,
        key: &WindowKey,
    ) {
        match input.clone().lock() {
            Ok(mut uin) => {
                /* handle key changes */
                while !uin.key_changes.is_empty() {
                    let change = uin.key_changes.pop_front().unwrap();
                    let state = uin.key_states.get_mut(&change).unwrap();
                    if !state.current.is_handled() {
                        handle_key_change(&change, state, game, camera, config, timing);
                        state.current.set_handled();
                    }
                }

                /* check key states */
                game.check_key_states(&uin.key_states, camera, &config, &timing);

                /* handle screen resize */
                if uin.screen_resized {
                    screen.update(key);
                    camera.update_screen(&screen.current_client_dimensions);
                    graphics.resize(camera);
                }

                /* handle mouse changes */
                while !uin.mouse_changes.is_empty() {
                    let change = uin.mouse_changes.pop_front().unwrap();
                    let state = uin.mouse_states.get_mut(&change).unwrap();
                    if !state.current.handled {
                        handle_mouse_change(game, &change, state, camera, &config, &timing, screen);
                        state.current.handled = true;
                    }
                }

                /* handle mouse deltas */
                if !uin.mouse_deltas.is_empty() {
                    game.handle_mouse_deltas(
                        &mut uin.mouse_deltas,
                        camera,
                        &config,
                        &timing,
                        screen,
                    );
                    uin.mouse_deltas.clear();
                }
            }
            Err(_) => {}
        }

        self.update_world_helper(input.clone(), screen, camera, timing, g2d, g3d);

        match input.lock() {
            Ok(mut uin) => {
                uin.screen_resized = false;
            }
            Err(_) => {
                panic!("todo: resetting screen_resized")
            }
        }
    }

    fn update_world_helper(
        &self,
        input: Arc<Mutex<UserInput>>,
        screen: &ScreenState,
        camera: &Camera,
        timing: &mut EngineTiming,
        g2d: &mut Graph2D,
        g3d: &mut Graph3D,
    );

    ///
    /// display the game world scene.
    ///
    /// fully controlled by engine; the engine is data-driven, meaning that graphics instructions
    /// come from models supplied during initialization, along with changes to those models
    /// during the update world step.
    ///
    fn display_world_scene<T: KeyHandler + MouseHandler + WorldController + 'static>(
        &self,
        _game: &T,
        input: Arc<Mutex<UserInput>>,
        camera: &mut Camera,
        config: &EngineConfig,
        screen: &mut ScreenState,
        timing: &EngineTiming,
        graphics: &mut GraphicsIntermediary,
        g2d: &mut Graph2D,
        g3d: &mut Graph3D,
    ) {
        /* gather variables */
        let uin = input.lock().unwrap();

        /* prepare for drawing */
        graphics.before_scene(&camera);

        /* draw 3d, if desired */
        graphics.prepare_3d(camera);
        graphics.render_3d(g3d);
        graphics.after_3d();

        /* draw 2d, if desired */
        graphics.prepare_2d(g2d, &camera);
        graphics.render_2d::<T>(g2d, &timing, &config, &camera, uin, &screen);
        graphics.after_2d();
    }
}
