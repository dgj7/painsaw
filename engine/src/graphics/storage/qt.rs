//!
//! this module represents the engine's implementation of a quadtree.
//!

use crate::graphics::storage::qt::screen::Screen;
use std::collections::HashMap;
use std::hash::Hash;

mod panel;
mod widget;
mod layout;
mod sizing;
mod screen;

///
/// manager for 2d ui screens.
///
/// allows for a maximum of one screen to be "active" at any one time.
///
pub struct UIManager<K: Eq + Hash> {
    active: Option<K>,
    screens: HashMap<K, Screen>,
}

impl<K: Eq + Hash> UIManager<K> {
    ///
    /// create a new instance.
    ///
    pub fn new<K1: Eq + Hash>() -> UIManager<K1> {
        UIManager {
            active: None,
            screens: HashMap::new(),
        }
    }

    ///
    /// activate a screen.
    ///
    /// automatically deactivates any currently activated screen.
    ///
    pub fn activate(&mut self, key: K) {
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
    pub fn resize(&mut self) {
        self.screens
            .values_mut()
            .for_each(|screen| screen.resize());
    }

    ///
    /// add an ui.
    ///
    pub fn add(&mut self, key: K, screen: Screen) {
        self.screens.insert(key, screen);
    }
}
