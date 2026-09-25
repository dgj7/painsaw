//!
//! this module represents the engine's implementation of a quadtree.
//!

use crate::graphics::storage::ui::view::View;
use std::collections::HashMap;
use crate::game::GameState;
use crate::geometry::primitive::v2d::Vertex2D;
use crate::graphics::storage::g2d::Graph2D;
use crate::graphics::storage::g3d::Graph3D;
use crate::input::screen::ScreenState;

pub mod view;

///
/// manager for 2d ui screens.
///
/// allows for a maximum of one screen to be "active" at any one time.
///
pub struct UIManager {
    active: Option<u32>,
    views: HashMap<u32, View>,
}

impl UIManager {
    ///
    /// create a new instance.
    ///
    pub fn new() -> UIManager {
        UIManager {
            active: None,
            views: HashMap::new(),
        }
    }

    ///
    /// activate a screen.
    ///
    /// automatically deactivates any currently activated screen.
    ///
    pub fn activate(&mut self, key: u32) {
        self.active = Some(key);
    }

    ///
    /// deactivate the currently active screen.
    ///
    pub fn deactivate(&mut self) {
        self.active = None;
    }

    ///
    /// handle window/container resize.
    ///
    pub fn resize(&mut self, screen: &ScreenState, _g2d: &mut Graph2D) {
        self.views
            .values_mut()
            .for_each(|view| view.resize(screen));
    }

    ///
    /// add an ui.
    ///
    pub fn add(&mut self, key: u32, view: View) {
        self.views.insert(key, view);
    }

    ///
    /// get the active ui, if one is available.
    ///
    pub fn check(&mut self) -> Option<&mut View> {
        match self.active {
            None => None,
            Some(ref active) => {
                self.views.get_mut(active)
            }
        }
    }

    ///
    /// handle click.
    ///
    pub fn click(&mut self, state: &mut GameState, point: &Vertex2D, g2d: &mut Graph2D, g3d: &mut Graph3D) {
        if let Some(key) = self.active {
            if let Some(view) = self.views.remove(&key) {
                view.click(state, point, self, g2d, g3d);
                self.views.insert(key, view);
            }
        }
    }
}
