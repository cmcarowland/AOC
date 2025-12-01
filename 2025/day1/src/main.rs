use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please enter file path for data set");
    }

    // pt1(&args[1]);
    pt2(&args[1]);
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in fs::read_to_string(filename).unwrap().lines() {
        result.push(line.to_string());
    }

    result
}

fn split_first_char_and_int(s : &str) -> Option<(char, i32)> {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) => Some((c, chars.as_str().parse::<i32>().unwrap())),
        None => None,
    }
}

fn pt1(filename : &str) {
    let lines = read_lines(filename);
    let mut current_dial : i32 = 50;
    let mut zero_counter : i32 = 0;
    for line in lines.iter() {
        if let Some((dir, value)) = split_first_char_and_int(line) {
            // println!("{0} {1}", dir, value);
            if dir == 'L' {
                current_dial -= value;
            } else {
                current_dial += value;
            }    

            current_dial %= 100;
            if current_dial == 0 {
                zero_counter += 1;
            }
            // println!("Current : {}", current_dial);
        } else {
            println!("ERROR {}", line);
        }
    }

    println!("{}", zero_counter);
}

fn pt2(filename : &str) {
    let lines = read_lines(filename);
    for line in lines.iter() {
        println!("{}", line);
    }
}