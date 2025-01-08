use std::env;
use std::fs;

mod map;
use map::map_data::MapData;
use map::point::Point;

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

const GRID_SIZE : i32 = 71;
const LAST_BYTE : i32 = 1024;

fn pt1(filename : &str) -> i64 {
    let lines = read_lines(filename);

    let mut grid = MapData::new(GRID_SIZE, GRID_SIZE);
    grid.create_grid(&lines, LAST_BYTE as usize);
    println!("{}", grid);
    let path = grid.find_path(Point::new(0, 0),Point::new(grid.get_width() - 1, grid.get_height() - 1));
    // grid.print_path(&path);

    return (path.len() - 2) as i64;

}

fn parse_line(line : &str) -> (i32, i32) {
    let split_line : Vec<i32> = line.split(",").map(|x| x.trim().parse::<i32>()).flatten().collect();
    (split_line[0], split_line[1])
}

fn pt2(filename : &str) -> i64 {
    let lines = read_lines(filename);
    let mut last_byte = 2530;
    let mut grid = MapData::new(GRID_SIZE, GRID_SIZE);
    let mut split : (i32, i32) = (0, 0);
    grid.create_grid(&lines, last_byte as usize);
    let mut last_path : Vec<Point> = Vec::new();

    loop {
        let path = grid.find_path(Point::new(0, 0),Point::new(grid.get_width() - 1, grid.get_height() - 1));
        if path.len() == 0 {
            println!("Split : {:?}", split);
            grid.print_path(&last_path, &Point::new(split.0, split.1));
            return last_byte as i64;
        }
        last_byte += 1;
        split = parse_line(&lines[last_byte - 1 as usize]);
        grid.set(&Point::new(split.0, split.1), 1);
        last_path = path;
    }

    return 0;
}