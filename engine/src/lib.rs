use crate::config::input_config::kc::{handle_key_change, KeyHandler};
use crate::config::input_config::mc::{handle_mouse_change, MouseHandler};
use crate::config::EngineConfig;
use crate::graphics::camera::Camera;
use crate::graphics::storage::gxd::Models;
use crate::graphics::RendererWrapper;
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
pub trait WorldController: KeyHandler + MouseHandler + Sized where Self: 'static {
    ///
    /// initialize the game world.
    ///
    fn initialize_world(
        &self,
        camera: &Camera,
        renderer: &mut RendererWrapper,
        models: &mut Models,
    ) {
        self.initialize_world_helper(camera, models);

        renderer.initialize(models);

        log(LogLevel::Debug, &|| String::from("initialization complete"));
    }

    ///
    /// initialize game world - customizer for client.
    ///
    fn initialize_world_helper(&self, camera: &Camera, models: &mut Models);

    ///
    /// update the game world state - fully controlled by client.
    ///
    fn update_world(
        &mut self,
        config: &EngineConfig,
        input: Arc<Mutex<UserInput>>,
        key: &WindowKey,
        camera: &mut Camera,
        timing: &mut EngineTiming,
        renderer: &RendererWrapper,
        models: &mut Models,
    ) {
        match input.clone().lock() {
            Ok(mut uin) => {
                /* handle key changes */
                while !uin.key_changes.is_empty() {
                    let change = uin.key_changes.pop_front().unwrap();
                    let state = uin.key_states.get_mut(&change).unwrap();
                    if !state.current.is_handled() {
                        handle_key_change(&change, state, self, config, camera, timing, models);
                        state.current.set_handled();
                    }
                }

                /* check key states */
                self.check_key_states(&uin.key_states, &config, camera, &timing, models);

                /* handle screen resize */
                if uin.screen_resized {
                    camera.screen.update(key);
                    camera.update_screen();
                    renderer.resize(camera);
                }

                /* handle mouse changes */
                while !uin.mouse_changes.is_empty() {
                    let change = uin.mouse_changes.pop_front().unwrap();
                    let state = uin.mouse_states.get_mut(&change).unwrap();
                    if !state.current.handled {
                        handle_mouse_change(&change, state, self, &config, camera, &timing, models);
                        state.current.handled = true;
                    }
                }

                /* handle mouse deltas */
                if !uin.mouse_deltas.is_empty() {
                    self.handle_mouse_deltas(&mut uin.mouse_deltas, &config, camera, &timing, models);
                    uin.mouse_deltas.clear();
                }
            }
            Err(_) => {}
        }

        self.update_world_helper(input.clone(), camera, timing, models);

        match input.lock() {
            Ok(mut uin) => { uin.screen_resized = false; }
            Err(_) => { panic!("todo: resetting screen_resized") }
        }
    }

    fn update_world_helper(
        &self,
        input: Arc<Mutex<UserInput>>,
        camera: &Camera,
        timing: &mut EngineTiming,
        models: &mut Models
    );

    ///
    /// display the game world scene.
    ///
    /// fully controlled by engine; the engine is data-driven, meaning that graphics instructions
    /// come from models supplied during initialization, along with changes to those models
    /// during the update world step.
    ///
    fn display_world_scene(
        &self,
        config: &EngineConfig,
        input: Arc<Mutex<UserInput>>,
        camera: &mut Camera,
        timing: &EngineTiming,
        renderer: &mut RendererWrapper,
        models: &mut Models
    ) {
        /* gather variables */
        let uin = input.lock().unwrap();

        /* prepare for drawing */
        renderer.before_scene(&camera);

        /* draw 3d, if desired */
        renderer.prepare_3d(camera);
        renderer.render_3d(&mut models.g3d);
        renderer.after_3d();

        /* draw 2d, if desired */
        renderer.prepare_2d(&camera, &mut models.g2d);
        renderer.render_2d(&config, uin, &camera, &timing, &mut models.g2d);
        renderer.after_2d();
    }
}
