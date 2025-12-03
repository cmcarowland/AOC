mod map;
use std::env;
use std::fs;

use map::map_data::MapData;
use map::map_data::Vector2;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Please enter file path for data set");
        return;
    }

    println!("Star 1 : {}", pt1(&args[1]));
    println!("Star 2 : {}", pt2(&args[2]));
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in fs::read_to_string(filename).unwrap().lines() {
        result.push(line.to_string());
    }

    result
}

fn calculate(map : &MapData) -> u64 {
    let mut total : u64 = 0;
    for location in map.find_vals('O') {
        total += (location.y * 100 + location.x) as u64;
    }
    print_map(map);
    return total;
}

fn move_up(map : &mut MapData) {
    let mut pos = map.robot;
    let mut count = 0;

    if map.grid[pos.x as usize][(pos.y - 1) as usize] == '#' {
        return;
    }

    if map.grid[pos.x as usize][(pos.y - 1) as usize] == '.' {
        count = 1;
        pos.y -= 1;
    } else {
        count += 1;
        pos.y -= 1;
        while map.grid[pos.x as usize][pos.y as usize] == 'O' {
            pos.y -= 1;
            count += 1;
            if map.grid[pos.x as usize][pos.y as usize] == '#' {
                return;
            }
        }
    }
    
    while count > 0 {
        let tmp = map.grid[pos.x as usize][pos.y as usize];
        map.grid[pos.x as usize][pos.y as usize] = map.grid[pos.x as usize][(pos.y + 1) as usize];
        map.grid[pos.x as usize][(pos.y + 1) as usize] = tmp;
        pos.y += 1;
        count -= 1;
    }

    map.robot = Vector2 { x: map.robot.x, y: map.robot.y - 1 };
}

fn move_left(map : &mut MapData) {
    let mut pos = map.robot;
    let mut count = 0;

    if map.grid[(pos.x - 1) as usize][pos.y as usize] == '#' {
        return;
    }

    if map.grid[(pos.x - 1) as usize][pos.y as usize] == '.' {
        count = 1;
        pos.x -= 1;
    } else {
        count += 1;
        pos.x -= 1;
        while map.grid[pos.x as usize][pos.y as usize] == 'O' {
            pos.x -= 1;
            count += 1;
            if map.grid[pos.x as usize][pos.y as usize] == '#' {
                return;
            }
        }
    }
    
    while count > 0 {
        let tmp = map.grid[pos.x as usize][pos.y as usize];
        map.grid[pos.x as usize][pos.y as usize] = map.grid[(pos.x + 1) as usize][pos.y as usize];
        map.grid[(pos.x + 1) as usize][pos.y as usize] = tmp;
        pos.x += 1;
        count -= 1;
    }

    map.robot = Vector2 { x: map.robot.x - 1, y: map.robot.y };
}

fn move_down(map : &mut MapData) {
    let mut pos = map.robot;
    let mut count = 0;

    if map.grid[pos.x as usize][(pos.y + 1) as usize] == '#' {
        return;
    }

    if map.grid[pos.x as usize][(pos.y + 1) as usize] == '.' {
        count = 1;
        pos.y += 1;
    } else {
        count += 1;
        pos.y += 1;
        while map.grid[pos.x as usize][pos.y as usize] == 'O' {
            pos.y += 1;
            count += 1;
            if map.grid[pos.x as usize][pos.y as usize] == '#' {
                return;
            }
        }
    }
        
    while count > 0 {
        let tmp = map.grid[pos.x as usize][pos.y as usize];
        map.grid[pos.x as usize][pos.y as usize] = map.grid[pos.x as usize][(pos.y - 1) as usize];
        map.grid[pos.x as usize][(pos.y - 1) as usize] = tmp;
        pos.y -= 1;
        count -= 1;
    }

    map.robot = Vector2 { x: map.robot.x, y: map.robot.y + 1 };
}

fn move_right(map : &mut MapData) {
    let mut pos = map.robot;
    let mut count = 0;

    if map.grid[(pos.x + 1) as usize][pos.y as usize] == '#' {
        return;
    }

    if map.grid[(pos.x + 1) as usize][pos.y as usize] == '.' {
        count = 1;
        pos.x += 1;
    } else {
        count += 1;
        pos.x += 1;
        while map.grid[pos.x as usize][pos.y as usize] == 'O' {
            pos.x += 1;
            count += 1;
            if map.grid[pos.x as usize][pos.y as usize] == '#' {
                return;
            }
        }
    }
    
    while count > 0 {
        let tmp = map.grid[pos.x as usize][pos.y as usize];
        map.grid[pos.x as usize][pos.y as usize] = map.grid[(pos.x - 1) as usize][pos.y as usize];
        map.grid[(pos.x - 1) as usize][pos.y as usize] = tmp;
        pos.x -= 1;
        count -= 1;
    }

    map.robot = Vector2 { x: map.robot.x + 1, y: map.robot.y };
}

fn print_map(map : &MapData) {
    for r in 0..map.get_height() as usize {
        for c in 0..map.get_width() as usize {
            print!("{}", map.grid[c][r]);    
        }
        println!();
    }
}

fn pt1(filename : &str) -> u64 {
    let mut lines = read_lines(filename);
    let s : String = lines.last().unwrap().clone();
    lines.remove(lines.len() - 1);
    lines.remove(lines.len() - 1);
    
    let mut map : MapData = MapData::new(lines.len() as i32, lines[0].len() as i32);
    map.create_grid(&lines);
    map.robot = map.find_vals('@')[0];
    print_map(&map);
    
    for c in s.chars() {
        match c {
            '^' => {
                move_up(&mut map);
            },
            'v' => {
                move_down(&mut map);
            },
            '>' => {
                move_right(&mut map);
            },
            '<' => {
                move_left(&mut map);
            },
            _ => { 
                println!("Invalid Direction");
            }
        }
        // print_map(&map);
    }

    return calculate(&map);
}

fn pt2(filename : &str) -> u64 {
    let mut lines = read_lines(filename);
    let s : String = lines.last().unwrap().clone();
    lines.remove(lines.len() - 1);
    lines.remove(lines.len() - 1);
    
    let mut map : MapData = MapData::new(lines.len() as i32, 2 * lines[0].len() as i32);
    map.create_grid(&lines);
    map.robot = map.find_vals('@')[0];
    print_map(&map);


    return 0;
}

fn calculate2(map : &MapData) -> u64 {
    let mut total : u64 = 0;
    for location in map.find_vals('[') {
        total += (location.y * 100 + location.x) as u64;
    }
    print_map(map);
    return total;
}