mod map;
use map::map_data::MapData;
use map::map_data::Point;
use std::collections::HashMap;
use std::env;
use std::fs;

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
    let mut plots : HashMap<char, i64> = HashMap::new();
    let mut map : MapData = MapData::new(lines.len() as i32, lines[0].len() as i32);
    map.create_grid(&lines);
    // println!("{:?}", map.grid);
    // println!("{}", map);

    for i in 0..map.get_height() * map.get_width() {
        let current_pos = Point::new(i % map.get_height(), i / map.get_width());
        let (area, perimeter) = map.check_neighbors(&current_pos);
        let current = map.get(&current_pos);
        let val = plots.entry(current).or_insert(0);
        *val = *val + area * perimeter;
        // println!("Current : {} Area: {}, Perimeter: {} Price: ${}", current, area, perimeter, plots[&current]);
    }

    println!("{:?}", plots);
    return plots.values().sum();
}

fn pt2(filename : &str) -> i64 {
    let lines = read_lines(filename);

    let lines = read_lines(filename);
    let mut plots : HashMap<char, i64> = HashMap::new();
    let mut map : MapData = MapData::new(lines.len() as i32, lines[0].len() as i32);
    map.create_grid(&lines);
    // println!("{:?}", map.grid);
    // println!("{}", map);

    for i in 0..map.get_height() * map.get_width() {
        let current_pos = Point::new(i % map.get_height(), i / map.get_width());
        let (area, perimeter) = map.check_neighbors(&current_pos);
        let current = map.get(&current_pos);
        let val = plots.entry(current).or_insert(0);
        *val = *val + area * perimeter;
        // println!("Current : {} Area: {}, Perimeter: {} Price: ${}", current, area, perimeter, plots[&current]);
    }

    println!("{:?}", plots);
    return plots.values().sum();
}