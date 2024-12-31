use std::collections::HashSet;
use std::fmt;
use std::ops;

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

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
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

    fn draw_edges(&self, edges : &HashSet<Edge>) {     
        let y = self.height * 2 + 1;
        let x = self.width * 2 + 1;
        for row in 0..y {
            for col in 0..x {
                if row % 2 == 0 {
                    if col % 2 == 0 {
                        print!("+");
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

    pub fn check_neighbors(&mut self, pos : &Point) -> (i64, i64) {
        if self.checked.contains(pos) {
            // println!("Already checked {}", pos);
            return (0, 0);
        }
        
        let mut members : Vec<Point> = vec![*pos];
        let mut area = 0;
        let mut edges : HashSet<Edge> = HashSet::new();
        // self.draw_edges(&edges);

        // println!("Checking neighbors for {} {}", pos, self.get(pos));
        while members.len() > 0 {
            let current_pos = members.pop().unwrap();
            if self.checked.contains(&current_pos) {
                continue;
            }
            
            area += 1;
            
            // println!("Current Char: {} {}", self.get(&current_pos), current_pos);
            if self.look_right(&current_pos) {
                // println!("Looked right");
                let right = Point::new(current_pos.x + 1, current_pos.y);
                             
                // self.draw_edges(&edges);
                if !self.checked.contains(&right) {
                    members.push(right);
                }
            } else {
                edges.insert(Edge::new(Point::new(current_pos.x + 1, current_pos.y), Point::new(current_pos.x + 1, current_pos.y + 1)));
            }

            if self.look_left(&current_pos) {
                // println!("Looked left");
                let left = Point::new(current_pos.x - 1, current_pos.y);
                
                // self.draw_edges(&edges);
                if !self.checked.contains(&left) {
                    members.push(left);
                }
            } else {
                edges.insert(Edge::new(Point::new(current_pos.x, current_pos.y), Point::new(current_pos.x, current_pos.y + 1)));
            }

            if self.look_up(&current_pos) {
                // println!("Looked Up");
                let up = Point::new(current_pos.x, current_pos.y - 1);
                
                // self.draw_edges(&edges);
                if !self.checked.contains(&up) {
                    members.push(up);
                }
            } else {
                edges.insert(Edge::new(Point::new(current_pos.x, current_pos.y), Point::new(current_pos.x + 1, current_pos.y)));

            }

            if self.look_down(&current_pos) {
                // println!("Looked Down");
                let down = Point::new(current_pos.x, current_pos.y + 1);
                        
                // self.draw_edges(&edges);
                if !self.checked.contains(&down) {
                    members.push(down);
                }
            } else {
                edges.insert(Edge::new(Point::new(current_pos.x, current_pos.y + 1), Point::new(current_pos.x + 1, current_pos.y + 1)));
            }

            self.checked.push(current_pos);
        }

        // self.draw_edges(&edges);

        return (area, edges.len() as i64);
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

#[derive(Hash)]
struct Edge {
    start : Point,
    end : Point
}

impl Edge {
    fn new(start: Point, end: Point) -> Edge {
        Edge {
            start,
            end
        }
    }
}

impl Eq for Edge {}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.end == other.end
    }
}

impl Copy for Edge { }

impl Clone for Edge {
    fn clone(&self) -> Edge {
        *self
    }
}

impl ops::Add<Edge> for Edge {
    type Output = Edge;

    fn add(self, other: Edge) -> Edge {
        Edge {
            start: self.start + other.start,
            end: self.end + other.end
        }
    }
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.start, self.end)
    }
}