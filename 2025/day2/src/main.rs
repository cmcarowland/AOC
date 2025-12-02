use std::env;
use std::fs;
use std::fmt;

struct IdPair {
    first_id : String,
    second_id : String,
    range: Vec<String>,
}

impl IdPair {
    fn new(first_id: &str, second_id: &str) -> Self {
        let first_int: i64 = first_id.parse::<i64>().unwrap();
        let second_int: i64 = second_id.parse::<i64>().unwrap();
        let range: Vec<String> = (first_int..=second_int).map(|n| n.to_string()).collect();
        IdPair { first_id: first_id.to_string(), second_id: second_id.to_string(), range }
    }
}

impl fmt::Display for IdPair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} : {}", self.first_id, self.second_id)
    }
}

fn test_id(id : &str) -> bool {
    let length: usize = id.to_string().len();

    if length % 2 != 0 {
        return false;
    }

    if id.chars().nth(0).unwrap() == '0' {
        return false;
    }

    let half_length: usize = length / 2;
    let first_half: &str = &id[0..half_length];
    let second_half: &str = &id[half_length..length];
    if first_half != second_half {
        return false;
    }

    true
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please enter file path for data set");
    }

    pt1(&args[1]);
    // pt2(&args[1]);
}

fn read_lines(filename: &str) -> Vec<IdPair> {
    let mut result: Vec<IdPair> = Vec::new();

    for line in fs::read_to_string(filename).unwrap().split(',') {
        let ids : Vec<&str> = line.split('-').collect();      
        let current_id: IdPair = IdPair::new(ids[0], ids[1]);
        result.push(current_id);
    }

    result
}

fn pt1(filename : &str) {
    let id_pairs: Vec<IdPair> = read_lines(filename);
    let mut answer : i64 = 0;

    for id in id_pairs {
        for num in &id.range {
            if test_id(num) {
                println!("{}", num);
                answer += num.parse::<i64>().unwrap();
            }
        }
    }

    println!("PT1 : {}", answer);
}

fn pt2(filename : &str) {
    let lines = read_lines(filename);
    let answer : i64 = 0;

    println!("PT2 : {}", answer);
}