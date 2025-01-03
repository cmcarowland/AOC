use std::env;
use std::fmt;
use std::fs;
use std::io::stdin;

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
    pub reg_a : i64,
    pub reg_b : i64,
    pub reg_c : i64,
    pub ip : usize,
    pub instructions : Vec<i64>,
    pub out : Vec<i64>,
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

    fn get_combo(&self) -> i64 {
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
        let denominator = 2i64.pow(combo as u32);
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
        let denominator = 2i64.pow(combo as u32);
        self.reg_b = self.reg_a / denominator;
        self.ip += 2;
    }

    /* The cdv instruction (opcode 7) works exactly like the adv instruction except that the result is stored in the C register. (The numerator is still read from the A register.) */
    fn cdv(&mut self) {
        let combo = self.get_combo();
        let denominator = 2i64.pow(combo as u32);
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
    let mut memory = Memory::new();

    for line in lines {
        if line == "" {
            continue;
        } else if line.contains("Register") {
            let split = line.split(":").collect::<Vec<&str>>();
            if split[0].contains('A') {
                memory.reg_a = split[1].trim().parse::<i64>().unwrap();
            } else if split[0].contains('B') {
                memory.reg_b = split[1].trim().parse::<i64>().unwrap();
            } else if split[0].contains('C') {
                memory.reg_c = split[1].trim().parse::<i64>().unwrap();
            }
        } else {
            let split = line.split(":").collect::<Vec<&str>>();
            memory.instructions = split[1].trim().split(",")
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
        }
    }

    println!("{}", memory);
    // let mut s = String::new();
    while memory.step() {
        // println!("{}", memory);
        // stdin().read_line(&mut s);
    }    

    println!("Program complete");
    println!("{}", memory);
    println!("{:?}", memory.out);

    return memory.out.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",");
}

fn pt2(filename : &str) -> i64 {
    let lines = read_lines(filename);

    return 0;
}