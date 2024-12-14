mod map;
use std::collections::HashSet;
use std::env;
use std::fs;

use map::map_data::MapData;
use map::map_data::Vector2;

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

fn find_paths(map : &MapData, points : &Vec<Vector2>) -> HashSet<Vector2> {
    let mut total: HashSet<Vector2> = HashSet::new();

    for point in points {
        // println!("{} {}", point, map.grid[point.x as usize][point.y as usize]);
        if map.grid[point.x as usize][point.y as usize] == 9 {
            total.insert(point.clone());
            continue;
        }
        let mut points : Vec<Vector2> = vec![];
        // Look Left
        if point.x - 1 >= 0 {
            if map.grid[(point.x - 1) as usize][point.y as usize] - map.grid[point.x as usize][point.y as usize] == 1 {
                points.push(Vector2::new(point.x - 1, point.y));
            }
        }
        // Look Right
        if point.x + 1 < map.get_width() {
            if map.grid[(point.x + 1) as usize][point.y as usize] - map.grid[point.x as usize][point.y as usize] == 1 {
                points.push(Vector2::new(point.x + 1, point.y));
            }
        }
        // Look Up
        if point.y - 1 >= 0 {
            if map.grid[point.x as usize][(point.y - 1) as usize] - map.grid[point.x as usize][point.y as usize] == 1 {
                points.push(Vector2::new(point.x, point.y - 1));
            }
        }
        // Look Down
        if point.y + 1 < map.get_height() {
            if map.grid[point.x as usize][(point.y + 1) as usize] - map.grid[point.x as usize][point.y as usize] == 1 {
                points.push(Vector2::new(point.x, point.y + 1));
            }
        }

        total.extend(find_paths(map, &points));
    }

    // println!("{:?}", total);
    return total;
}

fn find_paths2(map : &MapData, points : &Vec<Vector2>) -> Vec<Vector2> {
    let mut total: Vec<Vector2> = Vec::new();

    for point in points {
        // println!("{} {}", point, map.grid[point.x as usize][point.y as usize]);
        if map.grid[point.x as usize][point.y as usize] == 9 {
            total.push(point.clone());
            continue;
        }
        let mut points : Vec<Vector2> = vec![];
        // Look Left
        if point.x - 1 >= 0 {
            if map.grid[(point.x - 1) as usize][point.y as usize] - map.grid[point.x as usize][point.y as usize] == 1 {
                points.push(Vector2::new(point.x - 1, point.y));
            }
        }
        // Look Right
        if point.x + 1 < map.get_width() {
            if map.grid[(point.x + 1) as usize][point.y as usize] - map.grid[point.x as usize][point.y as usize] == 1 {
                points.push(Vector2::new(point.x + 1, point.y));
            }
        }
        // Look Up
        if point.y - 1 >= 0 {
            if map.grid[point.x as usize][(point.y - 1) as usize] - map.grid[point.x as usize][point.y as usize] == 1 {
                points.push(Vector2::new(point.x, point.y - 1));
            }
        }
        // Look Down
        if point.y + 1 < map.get_height() {
            if map.grid[point.x as usize][(point.y + 1) as usize] - map.grid[point.x as usize][point.y as usize] == 1 {
                points.push(Vector2::new(point.x, point.y + 1));
            }
        }

        total.append(&mut find_paths2(map, &points));
    }

    // println!("{:?}", total);
    return total;
}

fn pt1(filename : &str) -> u64 {
    let mut answer : u64 = 0;
    let lines = read_lines(filename);
    let mut map = MapData::new(lines.len() as i32, lines[0].len() as i32);
    map.create_grid(&lines);
    println!("{}", map.find_vals(0).len());
    for x in map.find_vals(0) {
        let points : Vec<Vector2> = vec![x];
        answer += find_paths(&map, &points).len() as u64;
    }

    return answer;
}

fn pt2(filename : &str) -> u64 {
    let lines = read_lines(filename);
    let mut answer : u64 = 0;
    let lines = read_lines(filename);
    let mut map = MapData::new(lines.len() as i32, lines[0].len() as i32);
    map.create_grid(&lines);
    for x in map.find_vals(0) {
        let points : Vec<Vector2> = vec![x];
        answer += find_paths2(&map, &points).len() as u64;
    }

    return answer;

    return 0;
}