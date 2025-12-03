use std::env;
use std::fs;

struct Bank {
    memory : Vec<i64>,
}

impl Bank {
    fn new(battery_string : &str) -> Bank {
        Bank { 
            memory : battery_string.chars().map(|c| c.to_digit(10).unwrap() as i64).collect(),
        }
    }

    fn find_max(&self) -> i64 {
        let mut current_index = 0;
        let mut max_value = 0;
        while current_index < self.memory.len() - 1 {
            let current_value = self.memory[current_index] * 10;
            for val in self.memory[current_index + 1..].iter() {
                if current_value + *val > max_value {
                    max_value = current_value + *val;
                }
            }
            
            current_index += 1;
        }

        max_value
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
    let mut answer = 0;
    let lines = read_lines(filename);

    for line in &lines {
        let bank = Bank::new(line);
        let max_value = bank.find_max();
        answer += max_value;
        println!("Max value for {} is {}", line, max_value);
    }   

    return answer;
}

fn pt2(filename : &str) -> i64 {
    let lines = read_lines(filename);

    return 0;
}