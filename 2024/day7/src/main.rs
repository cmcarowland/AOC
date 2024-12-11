use std::collections::VecDeque;
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

fn pt1(filename : &str) -> i64 {
    let lines = read_lines(filename);
    let mut answer = 0;

    for line in lines {
        let split : Vec<&str> = line.split(':').collect();
        let total = split[0].parse::<i64>().unwrap();
        let mut bit_map: Vec<i8> = vec![0; split[1].trim().split(' ').count()];

        while *bit_map.last().unwrap() == 0 {
            let mut operands : VecDeque<i64> = split[1].trim().split(' ').map(|x| x.parse::<i64>().unwrap()).collect();
            let mut index = 1;
            let mut operation = 0;
            while operands.len() > 1 {
                let first = operands.pop_front().unwrap();
                
                if bit_map[operation] == 0 {
                    operands[0] = operands[0] * first;
                } else if bit_map[operation] == 1 {
                    operands[0] = operands[0] + first;
                }
                operation += 1;
            }
            
            // println!("{} {:?} {:?}", total, operands, bit_map);    
            
            if operands[0] == total {
                // println!("Ans: {} {:?}", total, operands);                
                answer += operands[0];
                break;
            } else {
                let mut index = 0;
                loop {
                    bit_map[index] += 1;
                    if bit_map[index] == 1 {
                        break;
                    } else if bit_map[index] == 2 {
                        bit_map[index] = 0;
                    }
                    index += 1;
                }
            }
        }
    }

    return answer;
}

fn pt2(filename : &str) -> i64 {
    let lines = read_lines(filename);
    let mut answer = 0;

    for line in lines {
        // println!("{}", line);
        let split : Vec<&str> = line.split(':').collect();
        let total = split[0].parse::<i64>().unwrap();
        let mut bit_map: Vec<i8> = vec![0; split[1].trim().split(' ').count()];

        while *bit_map.last().unwrap() == 0 {
            let mut operands : VecDeque<i64> = split[1].trim().split(' ').map(|x| x.parse::<i64>().unwrap()).collect();
            let mut operation = 0;

            while operands.len() > 1 {
                let first = operands.pop_front().unwrap();
                
                if bit_map[operation] == 0 {
                    operands[0] = operands[0] * first;
                } else if bit_map[operation] == 1 {
                    operands[0] = operands[0] + first;
                }
                operation += 1;
            }
            
            // println!("{} {:?} {:?}", total, operands, bit_map);    
            
            if operands[0] == total {
                // println!("Ans: {} {:?}", total, operands);                
                answer += operands[0];
                break;
            } else {
                let mut index = 0;
                loop {
                    bit_map[index] += 1;
                    if bit_map[index] == 1 {
                        break;
                    } else if bit_map[index] == 2 {
                        bit_map[index] = 0;
                    }
                    index += 1;
                }
            }
        }
    }

    return answer;
}