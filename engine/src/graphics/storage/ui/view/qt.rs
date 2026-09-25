use std::collections::HashMap;
use crate::geometry::primitive::v2d::Vertex2D;
use crate::geometry::rect::Rectangle2D;
use crate::support::id::Identifier;
use crate::support::logger::log;
use crate::support::logger::log_level::LogLevel;

pub struct QuadTree {
    boundary: Rectangle2D,
    capacity: usize,
    items: HashMap<Identifier, Rectangle2D>,
    nested: Option<Box<[QuadTree; 4]>>,
    level: usize,
}

impl QuadTree {
    pub fn new(boundary: Rectangle2D) -> QuadTree {
        QuadTree {
            boundary,
            capacity: 4,
            items: HashMap::new(),
            nested: None,
            level: 1,
        }
    }

    pub fn insert(&mut self, id: &Identifier, rectangle: &Rectangle2D) -> bool {
        /* if this qt boundary doesn't contain any part of the rectangle, no work to do */
        if !self.boundary.contains_rect_inclusive(&rectangle) {
            log(LogLevel::Trace, &|| format!("insert(): skipping because {:?} is not contained in {:?}", rectangle, self.boundary));
            return false;
        }

        /* insert here if there's capacity */
        if self.items.len() < self.capacity && self.nested.is_none() {
            log(LogLevel::Trace, &|| format!("insert(): id={:?}, rectangle={:?}, level={}", id, rectangle, self.level));
            self.items.insert(id.clone(), rectangle.clone());
            return true;
        }

        /* otherwise subdivide */
        self.subdivide();

        /* insert into as many quadrants as the rectangle intersects */
        let mut inserted = false;
        if let Some(ref mut trees) = self.nested {
            for tree in trees.iter_mut() {
                if tree.insert(&id.clone(), &rectangle.clone()) {
                    inserted = true;
                }
            }
        }
        inserted
    }

    fn subdivide(&mut self) {
        let x = self.boundary.origin.x;
        let y = self.boundary.origin.y;
        let hw = self.boundary.antipode.x;
        let hh = self.boundary.antipode.y;

        let nw = QuadTree {
            boundary: Rectangle2D {
                origin: Vertex2D { x, y, },
                antipode: Vertex2D { x: hw, y: hh }
            },
            capacity: 4,
            items: HashMap::new(),
            nested: None,
            level: self.level + 1,
        };
        let ne = QuadTree {
            boundary: Rectangle2D {
                origin: Vertex2D { x: x + hw, y, },
                antipode: Vertex2D { x: hw, y: hh }
            },
            capacity: 4,
            items: HashMap::new(),
            nested: None,
            level: self.level + 1,
        };
        let sw = QuadTree {
            boundary: Rectangle2D {
                origin: Vertex2D { x, y: y + hh },
                antipode: Vertex2D { x: hw, y: hh }
                },
            capacity: 4,
            items: HashMap::new(),
            nested: None,
            level: self.level + 1,
        };
        let se = QuadTree {
            boundary: Rectangle2D {
                origin: Vertex2D { x: x + hw, y: y + hh, },
                antipode: Vertex2D { x: hw, y: hh }
            },
            capacity: 4,
            items: HashMap::new(),
            nested: None,
            level: self.level + 1,
        };

        self.nested = Some(Box::new([nw, ne, sw, se]));

        let existing = std::mem::take(&mut self.items);
        for (id,rect) in existing {
            if let Some(ref mut trees) = self.nested {
                for tree in trees.iter_mut() {
                    tree.insert(&id.clone(), &rect.clone());
                }
            }
        }
    }

    pub fn query(&self, point: &Vertex2D) -> Vec<Identifier> {
        let mut results = vec!();
        self.query_helper(point, &mut results);
        results
    }

    fn query_helper(&self, point: &Vertex2D, results: &mut Vec<Identifier>) {
        if !self.boundary.contains_pt_inclusive(point) {
            return;
        }

        for (id, rect) in &self.items {
            if rect.contains_pt_inclusive(point) {
                if !results.contains(id) {
                    results.push(id.clone());
                }
            }
        }

        if let Some(ref nested) = self.nested {
            for tree in nested.iter() {
                if tree.boundary.contains_pt_inclusive(point) {
                    tree.query_helper(point, results);
                    break;
                }
            }
        }
    }
}
