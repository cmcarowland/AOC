mod vector2;
use std::env;
use vector2::Vector2;
use std::fs;

struct GameRules {
    button_a : Vector2,
    button_b : Vector2,
    end : Vector2,
}

impl GameRules {
    fn new(button_a: Vector2, button_b: Vector2, end: Vector2) -> Self {
        Self { button_a, button_b, end }
    }

    fn Play(&self) -> u64 {
        let a_mul = 3;
        let b_mul = 1;
        let mut a_pressed = 0;
        let mut b_pressed = 0;
        if self.button_a.x > self.button_b.x {
            a_pressed = (self.end.x as f32 / self.button_a.x as f32).floor() as i32;
            b_pressed = ((self.end.x - (a_pressed * self.button_a.x)) as f32 / self.button_b.x as f32).ceil() as i32;
            let mut attempts = 0;
            while attempts < 100 {
                // println!("A : {} B : {}", a_pressed * self.button_a.x, self.button_b.x * b_pressed);
                if self.button_a.x * a_pressed + self.button_b.x * b_pressed == self.end.x && 
                   self.button_a.y * a_pressed + self.button_b.y * b_pressed == self.end.y {
                    println!("A : {} B : {}", a_pressed * a_mul, b_pressed * b_mul);
                    return (a_pressed * a_mul + b_pressed * b_mul) as u64;
                } else {
                    attempts += 1;
                    a_pressed -= 1;
                    b_pressed = ((self.end.x - (a_pressed * self.button_a.x)) as f32 / self.button_b.x as f32).ceil() as i32;
                }
            }
        } else {
            b_pressed = (self.end.x as f32 / self.button_b.x as f32).floor() as i32;
            a_pressed = ((self.end.x - (b_pressed * self.button_b.x)) as f32 / self.button_a.x as f32).ceil() as i32;
            let mut attempts = 0;
            while attempts < 100 {
                // println!("A : {} B : {}", a_pressed * self.button_a.x, self.button_b.x * b_pressed);
                if self.button_a.x * a_pressed + self.button_b.x * b_pressed == self.end.x && 
                   self.button_a.y * a_pressed + self.button_b.y * b_pressed == self.end.y {
                    println!("A : {} B : {}", a_pressed * a_mul, b_pressed * b_mul);
                    return (a_pressed * a_mul + b_pressed * b_mul) as u64;
                } else {
                    attempts += 1;
                    b_pressed -= 1;
                    a_pressed = ((self.end.x - (b_pressed * self.button_b.x)) as f32 / self.button_a.x as f32).ceil() as i32;
                }
            }
        }
        
        return 0;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please enter file path for data set");
        return;
    }

    println!("Star 1 : {}", pt1(&args[1]));
    println!("Star 2 : {}", pt2(&args[1]));
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in fs::read_to_string(filename).unwrap().lines() {
        result.push(line.to_string());
    }

    result
}

fn get_coord_value(coord : &str, search : &str) -> Vector2 {
    let mut x = 0;
    let mut y = 0;
    let mut iter = coord.find(&format!("X{}", search));
    match iter {
        Some(_) => x = coord[iter.unwrap() + 2..].split(',').collect::<Vec<&str>>()[0].parse::<i32>().unwrap(),
        None => x = coord[iter.unwrap() + 2..].split(',').collect::<Vec<&str>>()[0].parse::<i32>().unwrap(),
    }

    iter = coord.find(&format!("Y{}", search));
    match iter {
        Some(_) => y = coord[iter.unwrap() + 2..].split(',').collect::<Vec<&str>>()[0].parse::<i32>().unwrap(),
        None => y = coord[iter.unwrap() + 2..].split(',').collect::<Vec<&str>>()[0].parse::<i32>().unwrap(),
    }

    Vector2::new(x, y)
}

fn pt1(filename : &str) -> i64 {
    let lines = read_lines(filename);
    let mut total = 0;
    for line in (0..lines.len()).step_by(4) {
        println!("{}\n{}\n{}", lines[line], lines[line + 1], lines[line + 2]);

        let a : Vector2 = get_coord_value(&lines[line], "+");
        let b : Vector2 = get_coord_value(&lines[line + 1], "+");
        let end : Vector2 = get_coord_value(&lines[line + 2], "=");
        let game : GameRules = GameRules::new(a, b, end);
        total += game.Play();
    }

    return total as i64;
}

fn pt2(filename : &str) -> i64 {
    let lines = read_lines(filename);

    return 0;
}