use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please enter file path for data set");
    }

    println!("Star 1: {}", pt1(&args[1]));
    println!("Star 2: {}", pt2(&args[1]));
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in fs::read_to_string(filename).unwrap().lines() {
        result.push(line.to_string());
    }

    result
}

fn pt1(filename : &str) -> usize {
    let lines = read_lines(filename);
    let height = lines.len() as i32;
    let width = lines[0].len() as i32;

    println!("W: {} H: {}", width, height);
    let mut grid: Vec<Vec<i32>> = vec![vec![0; width as usize]; height as usize];
    let mut row = 0;
    let mut col = 0;
    // X, Y
    let mut guard :[i32; 2] = [0, 0];

    for line in lines {
        col = 0;
        for c in line.chars() {
            if c == '^' {
                guard[1] = row;
                guard[0] = col;
                grid[col as usize][row as usize] = 1;
            } else if c == '#' {
                grid[col as usize][row as usize] = 2;
            }

            col += 1;
        }
        row += 1;
    }

    let mut direction = 1;
    while guard[0] >= 0 && guard[0] < width - 1 && guard[1] >= 0 && guard[1] < height - 1 {
        match(direction) {
            1 => {
                println!("{} {:?}", grid[guard[0] as usize][(guard[1] - 1) as usize], guard);
                if grid[guard[0] as usize][(guard[1] - 1) as usize] == 2 {
                    println!("Rotate Right");
                    direction += 1;
                } else {
                    guard[1] -= 1;
                }
            },
            2 => {
                println!("{} {:?}", grid[(guard[0] + 1) as usize][guard[1] as usize], guard);
                if grid[(guard[0] + 1) as usize][guard[1] as usize] == 2 {
                    println!("Rotate Down");
                    direction += 1;
                } else {
                    guard[0] += 1;
                }
            },
            3 => {
                println!("{} {:?}", grid[guard[0] as usize][(guard[1] + 1) as usize], guard);
                if grid[guard[0] as usize][(guard[1]  + 1) as usize] == 2 {
                    println!("Rotate Left");
                    direction += 1;
                } else {
                    guard[1] += 1;
                }
            },
            4 => {
                println!("{} {:?}", grid[(guard[0] - 1) as usize][guard[1] as usize], guard);
                if grid[(guard[0] - 1) as usize][guard[1] as usize] == 2 {
                    println!("Rotate Up");
                    direction = 1;
                } else {
                    guard[0] -= 1;
                }
            },
            _ => println!("Failure!!!")
        }
        grid[guard[0] as usize][guard[1] as usize] = 1;
    }

    let mut count = 0;
    row = 0;
    while row < height {
        col = 0;
        while col < width {
            // print!("{} ", grid[col as usize][row as usize]);
            if grid[col as usize][row as usize] == 1 {
                count += 1;
            }
            col += 1;
        }
        // println!();
        row += 1;
    }

    return count;
}

fn pt2(filename : &str) -> usize {
    let _lines = read_lines(filename);

    return 0;
}