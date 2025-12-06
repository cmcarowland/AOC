use std::env;
use std::fs;

struct Range {
    start: i64,
    end: i64,
}

impl Range {
    fn new(start: i64, end: i64) -> Range {
        Range { start, end }
    }

    fn within(&self, value: i64) -> bool {
        return value >= self.start && value <= self.end;
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

fn pt1(filename : &str) -> i64 {
    let lines = read_lines(filename);
    let mut good_ingredients : Vec<Range> = Vec::new();
    let mut to_check : Vec<i64> = Vec::new();
    let mut ranges = true;
    let mut answer = 0;

    for line in lines {
        if line.is_empty() {
            ranges = false;
            continue;
        }

        if ranges {
            let parts: Vec<&str> = line.split("-").collect();
            let start: i64 = parts[0].parse().unwrap();
            let end: i64 = parts[1].parse().unwrap();
            let range = Range::new(start, end);
            good_ingredients.push(range);   
        } else {
            to_check.push(line.parse().unwrap());
        }

    }

    for check in to_check {
        for range in &good_ingredients {
            if range.within(check) {
                answer += 1;
                break;
            }
        }
    }

    return answer;
}

fn pt2(filename : &str) -> i64 {
    let lines = read_lines(filename);

    return 0;
}