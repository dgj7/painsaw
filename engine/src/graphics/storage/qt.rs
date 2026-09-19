//!
//! this module represents the engine's implementation of a quadtree.
//!

use crate::graphics::storage::qt::view::View;
use std::collections::HashMap;
use std::hash::Hash;
use crate::graphics::storage::g2d::Graph2D;
use crate::input::screen::ScreenState;

pub mod panel;
pub mod widget;
pub mod view;
pub mod attrib;

///
/// manager for 2d ui screens.
///
/// allows for a maximum of one screen to be "active" at any one time.
///
pub struct UIManager<K: Eq + Hash> {
    active: Option<K>,
    views: HashMap<K, View>,
}

impl<K: Eq + Hash> UIManager<K> {
    ///
    /// create a new instance.
    ///
    pub fn new<K1: Eq + Hash>() -> UIManager<K1> {
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
    pub fn resize(&mut self, screen: &ScreenState, _g2d: &mut Graph2D) {
        self.views
            .values_mut()
            .for_each(|view| view.resize(screen));
    }

    ///
    /// add an ui.
    ///
    pub fn add(&mut self, key: K, view: View) {
        self.views.insert(key, view);
    }

    ///
    /// get the active ui, if one is available.
    ///
    pub fn check(&self) -> Option<&View> {
        match self.active {
            None => None,
            Some(ref active) => {
                self.views.get(active)
            }
        }
    }
}
