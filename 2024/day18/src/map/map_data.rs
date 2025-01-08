use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt;
use std::iter;

use super::Point;
use super::Edge;
use super::Section;

pub struct MapData {
    height : i32,
    width : i32,
    pub grid : Vec<Vec<u32>>,
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

    pub fn get(&self, vector : &Point) -> i32 {
        self.grid[vector.x as usize][vector.y as usize] as i32
    }
    
    pub fn set(&mut self, vector : &Point, value : u32) {
        self.grid[vector.x as usize][vector.y as usize] = value;
    }

    pub fn create_grid(&mut self, lines : &Vec<String>, end : usize) {
        self.grid = vec![vec![0; self.height as usize]; self.width as usize];
        let mut current = 0;
        for line in lines {
            let split_line : Vec<usize> = line.split(",").map(|x| x.trim().parse::<usize>()).flatten().collect();
            let col: usize = split_line[0];
            let row: usize = split_line[1];
            
            self.grid[col][row] = 1;
            current += 1;
            if current == end {
                break;
            }
        }
    }

    fn in_bounds(&self, p : Point) -> bool{
        return p.x >= 0 && p.x < self.width && p.y >= 0 && p.y < self.height;
    }

    pub fn print_path(&self, path : &Vec<Point>, next_byte : &Point) {
        let mut grid = self.grid.clone();
        for point in path {
            if point == next_byte {
                grid[point.x as usize][point.y as usize] = 3;
                break;
            } 

            grid[point.x as usize][point.y as usize] = 2;
        }

        for row in 0..self.height {
            for col in 0..self.width {
                let current_point = Point::new(col, row);
                if row == 0 && col == 0 {
                    print!("{}", "S");
                } else if row == self.height - 1 && col == self.width - 1 {
                    print!("{}", "E");
                } else if  grid[current_point.x as usize][current_point.y as usize] == 2 {
                    print!("{}", "O");
                } else if grid[current_point.x as usize][current_point.y as usize] == 3 {
                    print!("{}", "X");
                } else if grid[current_point.x as usize][current_point.y as usize] == 1 {
                    print!("#");
                } else {
                    print!(".");
                }
                //write!(f, "{}", self.grid[col as usize][row as usize])?;
            }
            println!();
        }
    }

    pub fn find_path(&self, start : Point, end : Point) -> Vec<Point> {
        let mut path : Vec<Point> = Vec::new();
        let mut current: Point = start;
        let mut visited : Vec<Point> = Vec::new();
        let mut stack : VecDeque<(Point, Vec<Point>)> = VecDeque::new();
        current.g = 0;
        current.h = end - start;
        
        path.push(current);
        stack.push_front((current, path.clone()));

        while stack.len() > 0 {
            (current, path) = stack.pop_front().unwrap();
            // println!("current: ({}) Iteration {} Stack: {}", current, path.len(), stack.len());
            if visited.contains(&current) || !self.in_bounds(current) {
                continue;
            }
            
            visited.push(current);
            path.push(current);
            if current == end {
                // self.print_path(&path);
                return path;
            }

            let mut right = Point::new(current.x + 1, current.y);
            if self.in_bounds(right) && !visited.contains(&right) && self.get(&right) == 0 {
                right.g = current.g + 1;
                right.h = right.pythagorean(&end);
                stack.push_back((right, path.clone()));
            }
            
            let mut left = Point::new(current.x - 1, current.y);
            if self.in_bounds(left) && !visited.contains(&left) && self.get(&left) == 0  {
                left.g = current.g + 1;
                left.h = left.pythagorean(&end);
                stack.push_back((left, path.clone()));
            }
            
            let mut up = Point::new(current.x, current.y - 1);            
            if self.in_bounds(up) && !visited.contains(&up) && self.get(&up) == 0  {
                up.g = current.g + 1;
                up.h = up.pythagorean(&end);
                stack.push_back((up, path.clone()));
            }
            
            let mut down = Point::new(current.x, current.y + 1);
            if self.in_bounds(down) && !visited.contains(&down) && self.get(&down) == 0  {
                down.g = current.g + 1;
                down.h = down.pythagorean(&end);
                stack.push_back((down, path.clone()));
            }

            let mut stack_vec: Vec<_> = stack.drain(..).collect();
            stack_vec.sort_by_key(|k| k.0.clone().calc_f());
            stack = stack_vec.into();
            // if stack.len() > 1 {
            //     println!("Stack 0: {} {} {} ", stack[0].0. x, stack[0].0.y, stack[0].2);
            //     println!("Stack 1: {} {} {} ", stack[1].0. x, stack[1].0.y, stack[1].2);
            // }
        }

        return Vec::new();
    }
}

impl fmt::Display for MapData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                if self.get(&Point::new(col, row)) == 1 {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
                //write!(f, "{}", self.grid[col as usize][row as usize])?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
