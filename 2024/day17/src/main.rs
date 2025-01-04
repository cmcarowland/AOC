use std::env;
use std::fmt;
use std::fs;
use std::collections::VecDeque;

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

struct Memory {
    pub reg_a : u64,
    pub reg_b : u64,
    pub reg_c : u64,
    pub ip : usize,
    pub instructions : Vec<u64>,
    pub out : Vec<u64>,
}

impl Clone for Memory {
    fn clone(&self) -> Memory {
        Memory {
            reg_a : self.reg_a,
            reg_b : self.reg_b,
            reg_c : self.reg_c,
            ip: self.ip,
            instructions : self.instructions.clone(),
            out : self.out.clone(),
        }
    }
}

impl Memory {
    fn new() -> Memory {
        Memory {
            reg_a : 0,
            reg_b : 0,
            reg_c : 0,
            ip: 0,
            instructions : Vec::new(),
            out : Vec::new(),
        }
    }
    
    fn parse(lines : &Vec<String>) -> Memory {
        let mut memory = Memory::new();

        for line in lines {
            if line == "" {
                continue;
            } else if line.contains("Register") {
                let split = line.split(":").collect::<Vec<&str>>();
                if split[0].contains('A') {
                    memory.reg_a = split[1].trim().parse::<u64>().unwrap();
                } else if split[0].contains('B') {
                    memory.reg_b = split[1].trim().parse::<u64>().unwrap();
                } else if split[0].contains('C') {
                    memory.reg_c = split[1].trim().parse::<u64>().unwrap();
                }
            } else {
                let split = line.split(":").collect::<Vec<&str>>();
                memory.instructions = split[1].trim().split(",")
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>();
            }
        }

        memory
    }

    pub fn step(&mut self) -> bool {
        if self.ip >= self.instructions.len() {
            return false;
        }

        let opcode = self.instructions[self.ip];
        match opcode {
            0 => self.adv(),
            1 => self.bxl(),
            2 => self.bst(),
            3 => self.jnz(),
            4 => self.bxc(),
            5 => self.out(),
            6 => self.bdv(),
            7 => self.cdv(),
            _ => { 
                println!("Invalid opcode {}", opcode);
                return false;
            }
        }

        return true;
    }

    pub fn run_to_jmp(&mut self, val : u64) {
        self.reg_a = val;
        while self.ip < self.instructions.len() {
            let opcode = self.instructions[self.ip];

            match opcode {
                0 => self.adv(),
                1 => self.bxl(),
                2 => self.bst(),
                3 => return,
                4 => self.bxc(),
                5 => self.out(),
                6 => self.bdv(),
                7 => self.cdv(),
                _ => { 
                    println!("Invalid opcode {}", opcode);
                }
            }
        }
    }

    fn get_combo(&self) -> u64 {
        if self.instructions[self.ip + 1] < 4{
            return self.instructions[self.ip + 1];
        } else if self.instructions[self.ip + 1] < 7 {
            if self.instructions[self.ip + 1] == 4 {
                return self.reg_a;
            } else if self.instructions[self.ip + 1] == 5 {
                return self.reg_b;
            } else {
                return self.reg_c;
            }
        } else {
            return 0;
        }
    }

    //performs division with reg A as the numerator and denominator is 2 to the power of the combo operand
    fn adv(&mut self) {
        let combo = self.get_combo();
        let denominator = 2u64.pow(combo as u32);
        self.reg_a = self.reg_a / denominator;
        self.ip += 2;
    }

    /*The bxl instruction (opcode 1) calculates the bitwise XOR of register B and the instruction's literal operand, then stores the result in register B. */
    fn bxl(&mut self) {
        let combo = self.instructions[self.ip + 1];
        self.reg_b = self.reg_b ^ combo;
        self.ip += 2;
    }

    /*The bst instruction (opcode 2) calculates the value of its combo operand modulo 8 (thereby keeping only its lowest 3 bits), then writes that value to the B register. */
    fn bst(&mut self) {
        let combo = self.get_combo();
        self.reg_b = combo % 8;
        self.ip += 2;
    }

    /*The jnz instruction (opcode 3) does nothing if the A register is 0. However, if the A register is not zero, it jumps by setting the instruction pointer to the value of its literal operand; if this instruction jumps, the instruction pointer is not increased by 2 after this instruction. */
    fn jnz(&mut self) {
        if self.reg_a != 0 {
            self.ip = self.instructions[self.ip + 1] as usize;
        } else {
            self.ip += 2;
        }
    }

    /*The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C, then stores the result in register B. (For legacy reasons, this instruction reads an operand but ignores it.) */
    fn bxc(&mut self) {
        self.reg_b = self.reg_b ^ self.reg_c;
        self.ip += 2;
    }

    /* The out instruction (opcode 5) calculates the value of its combo operand modulo 8, then outputs that value. (If a program outputs multiple values, they are separated by commas.) */
    fn out(&mut self) {
        let combo = self.get_combo();
        self.out.push(combo % 8);
        self.ip += 2;
    }

    /* The bdv instruction (opcode 6) works exactly like the adv instruction except that the result is stored in the B register. (The numerator is still read from the A register.) */
    fn bdv(&mut self) {
        let combo = self.get_combo();
        let denominator = 2u64.pow(combo as u32);
        self.reg_b = self.reg_a / denominator;
        self.ip += 2;
    }

    /* The cdv instruction (opcode 7) works exactly like the adv instruction except that the result is stored in the C register. (The numerator is still read from the A register.) */
    fn cdv(&mut self) {
        let combo = self.get_combo();
        let denominator = 2u64.pow(combo as u32);
        self.reg_c = self.reg_a / denominator;
        self.ip += 2;
    }
}

impl fmt::Display for Memory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "A: {}, B: {}, C: {}, IP: {} Inst: {:?}", self.reg_a, self.reg_b, self.reg_c, self.ip, self.instructions)
    }
}

/*
Combo operands 0 through 3 represent literal values 0 through 3.
Combo operand 4 represents the value of register A.
Combo operand 5 represents the value of register B.
Combo operand 6 represents the value of register C.
Combo operand 7 is reserved and will not appear in valid programs.
 */

fn pt1(filename : &str) -> String {
    let lines = read_lines(filename);
    let mut memory = Memory::parse(&lines);

    while memory.step() {}    

    return memory.out.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",");
}

fn check_value(val : u64, mut memory : Memory) -> bool {
    memory.reg_a = val;
    while memory.step() {
        if memory.out.len() > 0 {
            
            for i in 0..memory.instructions.len() {
                if i >= memory.out.len() {
                    break;
                }
                
                if memory.out[i] != memory.instructions[i] {
                    // if memory.out.len() >= 5 {
                        // let mut s = String::new();
                        println!("Val {} {:#018b} {:?}", val, val, memory.out);
                        // let _ = stdin().read_line(&mut s);
                    // }
                    return false;
                }
            }
        }
    }

    if memory.out.len() != memory.instructions.len() {
        return false;
    }

    println!("Program complete");
    println!("{}", memory);
    println!("{:?}", memory.out);
    return true;
}

fn calc(va  : u64, cur : u64) -> Option<u64> {
    let mut reg_a = va;
    // 2,4 : reg_b = reg_a % 8
    let mut reg_b = reg_a % 8;
    // 1,1 : reg_b XOR 1
    reg_b ^= 1;
    // 7,5 : reg_c = reg_a / 2 ^ reg_b
    let reg_c = reg_a / 2u64.pow(reg_b as u32);
    // 1,5 : reg_b XOR 5
    reg_b ^= 5;
    // 4,0 : reg_b XOR reg_c
    reg_b ^= reg_c;

    // 0,3 : reg_a / 2 ^ 3
    reg_a /= 8;

    // 5,5 : add reg_b % 8 to output
    if reg_b % 8 == cur {
        return Some(reg_a);
    }

    return None;
}

fn pt2(filename : &str) -> u64 {
    let lines = read_lines(filename);
    let memory = Memory::parse(&lines);
    let mut i = 0;
    let mut stack : VecDeque<(u64, u64)> = VecDeque::new();
    let mut possibles : Vec<u64> = Vec::new();
    
    stack.push_front((0, 15));
    while stack.len() > 0 {
        i = 0;
        // println!("Poppng {} ", stack.len());
        let (val, gate) = stack.pop_front().unwrap();
        while i < 8 {
        //  Running throught the memory program
            // let mut current = memory.clone();
            // current.run_to_jmp(val + i);
            // if current.reg_b % 8 == memory.instructions[gate as usize] {
            //     if gate == 0 {
            //         possibles.push(val + i);
            //     }
            //     if gate > 0 {
            //         stack.push_front(((val + i) << 3,gate - 1));
            //     }
            // }

        // Using reverse engineered function
            let current = calc(val + i, memory.instructions[gate as usize]);
            if current != None {
                if gate == 0  {
                    possibles.push(val + i);
                    
                } else {
                    stack.push_front(((val + i) << 3, gate - 1));
                }                
            }

            i += 1;
        }
    }

    possibles.sort();
    println!("{:?}", possibles);
    return possibles[0];
}

