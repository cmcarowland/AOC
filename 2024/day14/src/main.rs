use std::env;
use std::fs;

mod vector2;
use vector2::Vector2;

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

fn pt1(filename : &str) -> i64 {
    let lines = read_lines(filename);
    let width = 101;
    let height = 103;
    const item_count : usize = 10403;
    let mut items: [i64; item_count] = [0; item_count];

    for line in lines {
        let split = line.split(" ").collect::<Vec<&str>>();
        let pos : Vec<&str> = split[0].split("=").collect::<Vec<&str>>()[1].split(",").collect();
        let vel :Vec<&str>   = split[1].split("=").collect::<Vec<&str>>()[1].split(",").collect();
        let mut position = Vector2::new(pos[0].parse().unwrap(), pos[1].parse().unwrap());
        let mut velocity = Vector2::new(vel[0].parse().unwrap(), vel[1].parse().unwrap());
        // println!("{}", position);
        velocity = velocity.multiply(100);
        // println!("{}", velocity);
        // println!("Vel : {}", velocity.x as f64 % width as f64);
        let x_movement = (velocity.x as f64 % width as f64).floor() as i64;
        // println!("{}", x_movement);
        let y_movement = (velocity.y as f64 % height as f64).floor() as i64;
        // println!("{}", y_movement);
        position.x += x_movement;
        // println!("Pos X : {}", position.x);
        if position.x < 0 {
            position.x = width + position.x;
        }
        if position.x >= width {
            position.x = position.x - width;
        }
        position.y += y_movement;
        // println!("Pos Y : {}", position.y);
        if position.y < 0 {
            position.y = height + position.y;
        }
        if position.y >= height {
            position.y = position.y - height;
        }

        // println!("New Position : {}", position);
        items[(width * position.y) as usize + position.x as usize] += 1;
    }

    print_grid(&items, width, height);
    
    return get_quads(&mut items, width, height);
}

fn get_quads(items: &mut [i64; 10403], width: i64, height: i64) -> i64 {
    let mid_x = width / 2;
    let mid_y = height / 2;
    let mut quads: Vec<i64> = vec![0, 0, 0, 0];

    for i in 0..items.len() as i64 {
        let current: i64 = items[(i % width + (i / width) * width) as usize];
        // if current == 0 {
        //     continue;
        // }

        let x = i % width;
        let y = i / width;

        if x == mid_x || y == mid_y {
            continue;
        } else if x < mid_x && y < mid_y {
            quads[0] += current;
        } else if x > mid_x && y < mid_y {
            quads[1] += current;
        } else if x < mid_x && y > mid_y {
            quads[2] += current;
        } else {
            quads[3] += current;
        }
    }
    println!("{:?}", quads);

    return quads.into_iter().product();
}

fn print_grid(items: &[i64; 10403], width: i64, height: i64) {
    for i in 0..items.len() as i64 {
        let current = items[(i % width + (i / width) * width) as usize];
        if current == 0 {
            print!(".");
        } else {
            print!("{}", current);
        }
        
        if i % width == width - 1 {
            println!();
        }
    }
}

fn pt2(filename : &str) -> i64 {
    let lines = read_lines(filename);

    return 0;
}