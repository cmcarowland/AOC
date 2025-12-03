use std::fmt;

#[derive(Hash)]
pub struct Vector2 {
    pub x : i32,
    pub y : i32,
}

impl Vector2 {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn dist(a : &Vector2, b : &Vector2) -> Vector2{
        return Vector2{
            x: a.x + b.x, 
            y: a.y + b.y
        };
    }
}

impl fmt::Display for Vector2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl PartialEq for Vector2 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Vector2 {}

impl Copy for Vector2 { }

impl Clone for Vector2 {
    fn clone(&self) -> Vector2 {
        *self
    }
}

pub struct MapData {
    height : i32,
    width : i32,
    pub grid : Vec<Vec<char>>,
    pub robot : Vector2,
}

impl MapData {
    pub fn new(height : i32, width : i32) -> Self {
        Self{ 
            height : height,
            width : width,
            grid : Vec::new(),
            robot : Vector2 { x: 0, y: 0 }
        }
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }

    pub fn get_width(&self) -> i32 {
        self.width
    }

    pub fn create_grid(&mut self, lines : &Vec<String>) {
        self.grid = vec![vec![0 as char; self.width as usize]; self.height as usize];
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

    pub fn find_vals(&self, other : char) -> Vec<Vector2> {
        let mut vals : Vec<Vector2> = vec![];
        for r in 0..self.height {
            for c in 0..self.width {
                if self.grid[c as usize][r as usize] == other {
                    vals.push(Vector2::new(c, r));
                }
            }
        }

        return vals;
    }
}