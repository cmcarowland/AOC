use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please enter file path for data set");
    }

    pt1(&args[1]);
    pt2(&args[1]);
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in fs::read_to_string(filename).unwrap().lines() {
        result.push(line.to_string());
    }

    result
}

fn pt1(filename : &str) {
    let mut left = Vec::<i32>::new();
    let mut right = Vec::<i32>::new();

    let lines = read_lines(filename);
    
    for line in lines.iter() {
        let parts : Vec<&str> = line.split("   ").collect();
        

        left.push(parts[0].parse::<i32>().unwrap());
        right.push(parts[1].parse::<i32>().unwrap());
    }

    left.sort();
    right.sort();
    //.sort_by(|a, b| b.cmp(a));
    let mut total_distance : i32 = 0;
    for i in 0..left.len() {
        total_distance += (right[i] - left[i]).abs();
    }

    println!("Total Distance {}", total_distance);
}

fn pt2(filename : &str) {
    let mut left = Vec::<usize>::new();
    let mut right = Vec::<usize>::new();

    let lines = read_lines(filename);
    
    for line in lines.iter() {
        let parts : Vec<&str> = line.split("   ").collect();
        

        left.push(parts[0].parse::<usize>().unwrap());
        right.push(parts[1].parse::<usize>().unwrap());
    }

    left.sort();
    right.sort();
    //.sort_by(|a, b| b.cmp(a));
    let mut total_distance : usize = 0;
    for i in 0..left.len() {
        let right_count = right.iter().filter(|&n| *n == left[i]).count();
        total_distance += left[i] * right_count;
    }

    println!("Total Distance {}", total_distance);
}
