use std::cmp::Ordering;
use std::fmt;
use std::hash::Hash;
use std::ops::{self, Add};

#[derive(Hash)]
pub struct Point {
    pub x : i32,
    pub y : i32,
    pub g : i32,
    pub h : i32,
}

impl Copy for Point { }

impl Clone for Point {
    fn clone(&self) -> Point {
        *self
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Eq for Point {}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.x.cmp(&other.x).then_with(|| self.y.cmp(&other.y))
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            g: self.g, 
            h: self.h
        }
    }
}

impl ops::Sub<Point> for Point {
    type Output = i32;

    fn sub(self, other: Point) -> i32 {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as i32
    }
}

impl Point {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y, g: i32::MAX, h:i32::MAX }
    }

    pub fn calc_f(&mut self) -> i32{
        self.g + self.h
    }

    pub fn pythagorean (&mut self, other: &Point) -> i32 {
        (((self.x - other.x).abs().pow(2) + (self.y - other.y).abs().pow(2)) as f32).sqrt().floor() as i32
    }
}