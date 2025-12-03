use super::Point;

use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Clone)]
pub struct Edge {
    start : Point,
    end : Point,
}

impl Hash for Edge {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.start.hash(state);
        self.end.hash(state);
    }
}

impl Edge {
    pub fn new(start: Point, end: Point) -> Edge {
        Edge {
            start,
            end,
        }
    }

    pub fn get_start(&self) -> Point {
        self.start
    }

    pub fn get_end(&self) -> Point {
        self.end
    }

    pub fn direction(&self) -> Point {
        Point::new((self.end.x - self.start.x).abs(), (self.end.y - self.start.y).abs())
    }

    pub fn change_direction(&self, other : &Edge) -> bool {
        let diff = self.direction() + other.direction();
        // println!("{}", diff);

        if diff.x > 0 && diff.y > 0 {
            return true;
        }

        return false;
    }
}

impl Eq for Edge {}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.end == other.end
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start.cmp(&other.start).then_with(|| self.end.cmp(&other.end))
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}) ", self.start, self.end)
    }
}