use std::collections::{hash_set, HashSet};
use std::fmt::{self, Debug};
use std::hash::{Hash, Hasher};
use std::ops;
use std::cmp::Ordering;

#[derive(Hash)]
pub struct Point {
    pub x : i32,
    pub y : i32,
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
            y: self.y + other.y
        }
    }
}

impl Point {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

pub struct MapData {
    height : i32,
    width : i32,
    pub grid : Vec<Vec<char>>,
    pub checked : Vec<Point>,

}

impl MapData {
    pub fn new(height : i32, width : i32) -> Self {
        Self{ 
            height : height,
            width : width,
            grid : Vec::new(),
            checked : Vec::new(),
        }
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }

    pub fn get_width(&self) -> i32 {
        self.width
    }

    pub fn get(&self, vector : &Point) -> char {
        self.grid[vector.x as usize][vector.y as usize] as char
    }

    pub fn create_grid(&mut self, lines : &Vec<String>) {
        self.grid = vec![vec![0 as char; self.height as usize]; self.width as usize];
        let mut row : usize = 0;
        for line in lines {
            let mut col: usize = 0;
            for c in line.chars() {
                self.grid[col][row] = c;
                col += 1;
            }
            row += 1;
        }
    }

    pub fn contains_checked(&self, pos : &Point) -> bool {
        self.checked.contains(pos)
    }

    pub fn draw_edges(&self, edges : &HashSet<Edge>) {     
        let y = self.height * 2 + 1;
        let x = self.width * 2 + 1;
        for row in 0..y {
            for col in 0..x {
                if row % 2 == 0 {
                    if col % 2 == 0 {
                        if row == 0 || row == y - 1 {
                            print!("{}", col / 2);
                        } else {
                            print!("+");
                        }
                    } else {
                        if self.find_edge(edges, &Edge::new(Point::new(col / 2, row / 2), Point::new(col / 2 + 1, row / 2))) {
                            print!("-");
                        } else {
                            print!(" ");
                        }
                    }
                } else {
                    if col % 2 == 0 {
                        if self.find_edge(edges, &Edge::new(Point::new(col / 2, row / 2), Point::new(col / 2, row / 2 + 1))) {
                            print!("|");
                        } else {
                            print!(" ");
                        }
                    } else {
                        print!("{}", self.get(&Point::new(col / 2, row / 2)));
                    }
                }
            }
            println!();
        }

        println!();
    }

    fn find_edge(&self, edges : &HashSet<Edge>, other_edge : &Edge) -> bool {
        // println!("Checking for edge {}", other_edge);
        for edge in edges {
            // println!("Checking edge {}", edge);
            if *edge == *other_edge {
                return true;
            }
        }

        return false;
    }

    pub fn check_neighbors(&mut self, pos : &Point, section : &mut Section) -> HashSet<Edge> {
        if self.checked.contains(pos) {
            // println!("Already checked {}", pos);
            return HashSet::new();
        }
        
        let mut members : Vec<Point> = vec![*pos];
        let mut edges : HashSet<Edge> = HashSet::new();
        // self.draw_edges(&edges);

        // println!("Checking neighbors for {} {}", pos, self.get(pos));
        while members.len() > 0 {
            let current_pos = members.pop().unwrap();
            if self.checked.contains(&current_pos) {
                continue;
            }
                       
            // println!("Current Char: {} {}", self.get(&current_pos), current_pos);
            if self.look_right(&current_pos) {
                // println!("Looked right");
                let right = Point::new(current_pos.x + 1, current_pos.y);
                             
                // self.draw_edges(&edges);
                if !self.checked.contains(&right) {
                    members.push(right);
                    section.members.insert(right);
                }
            } else {
                let mut e = Edge::new(Point::new(current_pos.x + 1, current_pos.y), Point::new(current_pos.x + 1, current_pos.y + 1));
                e.add_id(unsafe { get_id() });
                edges.insert(e);
            }

            if self.look_left(&current_pos) {
                // println!("Looked left");
                let left = Point::new(current_pos.x - 1, current_pos.y);
                
                // self.draw_edges(&edges);
                if !self.checked.contains(&left) {
                    members.push(left);
                    section.members.insert(left);
                }
            } else {
                let mut e = Edge::new(Point::new(current_pos.x, current_pos.y), Point::new(current_pos.x, current_pos.y + 1));
                e.add_id(unsafe { get_id() });
                edges.insert(e);
            }

            if self.look_up(&current_pos) {
                // println!("Looked Up");
                let up = Point::new(current_pos.x, current_pos.y - 1);
                
                // self.draw_edges(&edges);
                if !self.checked.contains(&up) {
                    members.push(up);
                    section.members.insert(up);
                }
            } else {
                let mut e = Edge::new(Point::new(current_pos.x, current_pos.y), Point::new(current_pos.x + 1, current_pos.y));
                e.add_id(unsafe { get_id() });
                edges.insert(e);

            }

            if self.look_down(&current_pos) {
                // println!("Looked Down");
                let down = Point::new(current_pos.x, current_pos.y + 1);
                        
                // self.draw_edges(&edges);
                if !self.checked.contains(&down) {
                    members.push(down);
                    section.members.insert(down);
                }
            } else {
                let mut e = Edge::new(Point::new(current_pos.x, current_pos.y + 1), Point::new(current_pos.x + 1, current_pos.y + 1));
                e.add_id(unsafe { get_id() });
                edges.insert(e);
            }

            self.checked.push(current_pos);
        }

        // self.draw_edges(&edges);
        unsafe { inc_id(); }
        section.edges = edges.clone();
        return edges;
    }

    pub fn look_right(&self, pos : &Point) -> bool {
        let right = Point::new(pos.x + 1, pos.y);
        if right.x >= self.width {
            return false;
        }
        
        let current: char = self.grid[pos.x as usize][pos.y as usize];
        if self.grid[right.x as usize][right.y as usize] == current {
            return true;
        }

        return false;
    }
   
    pub fn look_left(&self, pos : &Point) -> bool {
        let left = Point::new(pos.x - 1, pos.y);
        if left.x < 0 {
            return false;
        }

        let current: char = self.grid[pos.x as usize][pos.y as usize];
        if self.grid[left.x as usize][left.y as usize] == current {
            return true;
        }

        return false;
    }
    
    pub fn look_up(&self, pos : &Point) -> bool {
        let up = Point::new(pos.x, pos.y - 1);
        if up.y < 0 {
            return false;
        }

        let current: char = self.grid[pos.x as usize][pos.y as usize];
        if self.grid[up.x as usize][up.y as usize] == current {
            return true;
        }

        return false;
    }

    pub fn look_down(&self, pos : &Point) -> bool {
        let down = Point::new(pos.x, pos.y + 1);
        if down.y >= self.height {
            return false;
        }

        let current: char = self.grid[pos.x as usize][pos.y as usize];
        if self.grid[down.x as usize][down.y as usize] == current {
            return true;
        }

        return false;
    }
}

impl fmt::Display for MapData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                write!(f, "{}", self.grid[col as usize][row as usize])?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

static mut ID : u64 = 0;

unsafe fn get_id() -> u64 {
    ID
}

unsafe fn inc_id() {
    ID += 1;
}

#[derive(Clone)]
pub struct Edge {
    start : Point,
    end : Point,
    gid : u64,
}

impl Hash for Edge {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.start.hash(state);
        self.end.hash(state);
    }
}

impl Edge {
    fn new(start: Point, end: Point) -> Edge {
        Edge {
            start,
            end,
            gid: 0,
        }
    }


    pub fn get_gid(&self) -> u64 {
        self.gid
    }

    pub fn add_id(&mut self, id : u64) {
        self.gid = id;
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
        write!(f, "({}, {}) ID: {:?}", self.start, self.end, self.gid)
    }
}

pub struct Section {
    c : char,
    location : Point,
    pub members : HashSet<Point>,
    pub edges : HashSet<Edge>,
}

impl Section {
    pub fn new(c : char, location : Point) -> Section {
        Section {
            c,
            location,
            members : {
                let mut set = HashSet::new();
                set.insert(location);
                set
            },
            edges : HashSet::new(),
        }
    }

    pub fn get_plot(&self) -> char {
        self.c
    }

    pub fn calculate_area(&self) -> i64 {
        self.members.len() as i64
    }

    pub fn calculate_perimeter(&self) -> i64 {
        self.edges.len() as i64
    }

    pub fn calculate_sides(&self) -> i64 {
        let mut edges : Vec<&Edge> = self.edges.iter().collect();
        edges.sort();

        let mut sides = 0;
        let mut processed: Vec<&Edge> = Vec::new();
        let mut index = 0;
        let mut current_edge = edges[index];

        while processed.len() < edges.len() {
            let mut pos : Option<usize> = None;
            if processed.contains(&current_edge) {
                pos = edges.iter().position(|x| !processed.contains(x));
                if pos.is_none() {
                    println!("Disconnected Edge {}", current_edge);
                    break;
                }
                
                current_edge = edges[pos.unwrap()];
                continue;
            }

            pos = edges.iter().position(|x| 
                *x != current_edge && 
                (x.get_start() == current_edge.get_end() || x.get_end() == current_edge.get_end())
            );

            let i = pos.unwrap();
        
            if current_edge.change_direction(edges[i]) {
                sides += 1;
            }    
            
            processed.push(current_edge);
            index = i;
            current_edge = edges[i];
        }
        
        return sides;
    }
}

impl Debug for Section {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Section: {} Area: {}", self.c, self.calculate_area())
    }
}