use std::env;
use std::fs;
use std::collections::HashMap;
use std::ptr::null;

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

// fn check_registers(reg : &str, registers : &mut HashMap<String, u8>, filename : &str, firstcommand : usize) {
//     if registers.contains_key(reg) {
//         return;
//     }

//     let lines = read_lines(filename);
//     for line in lines[firstcommand..].iter() {
//         let split_line = line.split(" -> ").collect::<Vec<&str>>();
//         // println!("{:?}", split_line);
//         if split_line[1] == reg {
//             let command = split_line[0].split(" ").collect::<Vec<&str>>();
//             if command[0].starts_with('x') || command[0].starts_with('y') {
//                 if command[2].starts_with('x') || command[2].starts_with('y') {
//                     process_command(&line, registers);                }
//             }
//         }
//     }
// }

fn process_command(op : &Operation) -> u8 {
    // let a = registers.get(command[0]).cloned().unwrap_or(0);
    // let b = registers.get(command[2]).cloned().unwrap_or(0);
    let mut value = 0;
    match op.operator.as_str() {
        "AND" => {
            value = op.left_value & op.right_value;
        },
        "OR" => {
            value = op.left_value | op.right_value;
        },
        "XOR" => {
            value = op.left_value ^ op.right_value;
        },
        _ => {
            println!("Unknown command : {}", op.operator);
            return 0;
        }
    }
    // registers.insert(split_line[1].to_string(), value);
    // println!("{}:{} {} {}:{} = {} as {}", a, command[0], command[1], b, command[2], value, split_line[1]);
    return value;
}

struct Node {
    name : String,
    value : u8,
    left : String,
    right : String
}

impl Node {
    fn new(name : String, value : u8, left : String, right : String) -> Node {
        Node {
            name,
            value,
            left,
            right
        }
    }
}

struct Operation {
    left : String,
    left_value : u8,
    operator : String,
    right : String,
    right_value : u8,
    result : String
}

impl Operation {
    fn new(left : String, operator : String, right : String, result : String) -> Operation {
        Operation {
            left,
            left_value : 0,
            operator,
            right,
            right_value : 0,
            result
        }
    }

    fn get_result(&mut self, registers : &HashMap<String, u8>, instructions : &Vec<Operation>) -> u8 {
        // println!("Processing Operation : {} {} {} -> {}", self.left, self.operator, self.right, self.result);
        if registers.contains_key(&self.left) && registers.contains_key(&self.right) {
            self.left_value = registers.get(&self.left).cloned().unwrap();
            self.right_value = registers.get(&self.right).cloned().unwrap();
            return process_command(&self);
        }

        if !registers.contains_key(&self.left) {
            self.left_value = find_instruction(&self.left, &instructions).get_result(registers, instructions);
        }
        
        if !registers.contains_key(&self.right) {
            self.right_value = find_instruction(&self.right, &instructions).get_result(registers, instructions);
        }
        // println!("Processing Operation : {} as {} {} {} as {} -> {}", self.left, self.left_value, self.operator, self.right, self.right_value, self.result);
        return process_command(&self);
    }
}

impl std::fmt::Debug for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {} -> {}", self.left, self.operator, self.right, self.result)
    }
}


impl Clone for Operation {
    fn clone(&self) -> Operation {
        Operation {
            left : self.left.clone(),
            left_value : self.left_value,
            operator : self.operator.clone(),
            right : self.right.clone(),
            right_value : self.right_value,
            result : self.result.clone()
        }
    }
}

fn find_instruction(instruction : &str, instructions : &Vec<Operation>) -> Operation {
    for i in instructions {
        if i.result == instruction {
            return i.clone();
        }
    }

    return Operation::new("NONE".to_string(), "NONE".to_string(), "NONE".to_string(), "NONE".to_string());
}

fn pt1(filename : &str) -> i64 {
    let lines = read_lines(filename);
    let mut registers : HashMap<String, u8> = HashMap::new();

    let mut instructions : Vec<Operation> = Vec::new();
    let mut loading = true;
    for line in lines {
        if loading {
            if line == "" {
                loading = false;
                // println!("Registers : {:?}", registers);
                continue;
            }

            let split_line = line.split(":").collect::<Vec<&str>>();
            // println!("{:?}", split_line);
            registers.insert(split_line[0].to_string(), split_line[1].trim().parse::<u8>().unwrap());
        } else {
            let split_line = line.split(" -> ").collect::<Vec<&str>>();
            let command = split_line[0].split(" ").collect::<Vec<&str>>();
            instructions.push(Operation::new(command[0].to_string(), command[1].to_string(), command[2].to_string(), split_line[1].to_string()));
        }
    }

    let mut answer : u64 = 0;
    
    let base: &str = "z";
    let mut current_bit = 0;
    loop {
        let current = format!("{}{:02}", base, current_bit);
        let mut op = find_instruction(current.as_str(), &instructions);
        if op.operator == "NONE" {
            break;
        }

        let x : u8 = op.get_result(&registers, &instructions);
        // println!("{}", x);
        answer += (x as u64) << current_bit;
        // println!("{}", answer);
        current_bit += 1;
    }

    // let x = registers.into_iter().filter(|(k, _)| k.starts_with("z")).collect::<HashMap<String, u8>>();
    // Sort the x hashmap by key so that z00, z01, z02, z03, z04, z05, z06, z07, z08, z09, z10, z11, z12, z13, z14, z15
    // let mut x = x.into_iter().collect::<Vec<(String, u8)>>();
    // x.sort_by(|a, b| a.0.cmp(&b.0));
    // println!("Output : {:?}", x);
    
    // for bit in x {
    //     answer += (bit.1 as u64) << current_bit;
    //     current_bit += 1;
    // }
    
    return answer as i64;
}

fn pt2(filename : &str) -> i64 {
    let lines = read_lines(filename);

    return 0;
}