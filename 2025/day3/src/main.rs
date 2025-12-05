use std::env;
use std::fs;

struct Cell{
    value : char,
    marked : bool,
}

impl Cell {
    fn new(value : char) -> Cell {
        Cell { value, marked : false }
    }
}

struct Bank {
    memory : Vec<Cell>,
}

impl Bank {
    fn new(battery_string : &str) -> Bank {
        Bank { 
            memory : battery_string.chars().map(|c| Cell::new(c)).collect(),
        }
    }

    fn find_max(&mut self) -> i64 {
        let mut current_index = 0;
        let mut other_index = 1;
        let mut max_value = 0;
        while current_index < self.memory.len() {
            self.memory[current_index].marked = true;
            other_index = current_index + 1;
            while other_index < self.memory.len() {
             
                self.memory[other_index].marked = true;
                let current_value: i64 = self.memory.iter()
                    .filter(|c| c.marked)
                    .map(|c| c.value)
                    .collect::<String>()
                    .parse::<i64>()
                    .unwrap_or(0);
                // println!("Current Value: {}", current_value);
                if current_value > max_value {
                    max_value = current_value;
                }

                self.memory[other_index].marked = false;
                other_index += 1;
            }
            self.memory[current_index].marked = false;
            current_index += 1;
        }

        max_value
    }
// 987 654 321 111 111
// 876 543 211 111
// 87 654 321 111
// 811 1 11111111119
    fn mark_highest_leftmost(&mut self, start_index: usize) -> usize{
        let mut highest_value = -1;
        let mut highest_leftmost = start_index;
        let mut current_index = self.memory.len() - 1;

        println!("Finding max pt2 for memory: {}", 
            self.memory[start_index..].iter()
                .map(|c| c.value)
                .collect::<String>());
        

        while current_index > start_index {
            let current_value = self.memory[current_index].value.to_digit(10).unwrap() as i64;
            if current_value >= highest_value as i64 {
                highest_value = current_value;
                highest_leftmost = current_index;
            }
            current_index -= 1;
        }

        self.memory[highest_leftmost].marked = true;
        highest_leftmost
    }

    fn find_max_pt2(&mut self) -> i64 {
        println!("Finding max pt2 for memory: {} {}", 
            self.memory.iter()
                .map(|c| c.value)
                .collect::<String>(), self.memory.len());
        
        let mut current_index = self.memory.len() - 12;
        let mut max_value = 0;
        let mut highest_value = -1;
        let mut highest_leftmost = self.memory.len();
        while current_index >= 0 {
            let current_value = self.memory[current_index].value.to_digit(10).unwrap() as i64;
            // println!("Current Index: {}, Current Value: {}", current_index, current_value);
            if current_value >= highest_value as i64 {
                highest_value = current_value;
                highest_leftmost = current_index;
            }

            if current_index == 0 {
                break;
            }
            current_index -= 1;
        }
        println!("Highest leftmost index: {} {}", highest_leftmost, highest_value);
        current_index = highest_leftmost;
        // Iterate from current_index down to len() - 1 and find the highest 11 other values
        self.memory[current_index].marked = true;
        let mut marked_count = 1;
        let mut next_max = 9;
        // while marked_count < 12 && next_max > 1 {
        //     let mut search_index = current_index + 1;
        //     // println!("Seach Index {}", search_index);
        //     while search_index < self.memory.len() {
        //         let search_value = self.memory[search_index].value.to_digit(10).unwrap() as i64;
        //         // println!("Searching for {} value {}", next_max, search_value);
        //         if search_value == next_max {
        //             self.memory[search_index].marked = true;
        //             // println!("Marked index {} with value {}", search_index, self.memory[search_index].value);
        //             marked_count += 1;
        //             if marked_count == 12 {
        //                 break;
        //             }
        //         }
                
        //         search_index += 1;
        //     }

        //     next_max -= 1;
        // }
        // println!("Marked count after first pass: {} {}", marked_count, next_max);
        // if marked_count < 12 && next_max == 1 {
        while marked_count < 12 {
            let mut search_index = self.memory.len() - 1;
            while search_index > current_index {
                let search_value = self.memory[search_index].value.to_digit(10).unwrap() as i64;
                if search_value == next_max {
                    // println!("Marked index {} with value {}", search_index, self.memory[search_index].value);
                    
                    self.memory[search_index].marked = true;
                    marked_count += 1;
                    if marked_count == 12 {
                        break;
                    }
                }

                search_index -= 1;
            }
            next_max -= 1;
        }
        // }

        max_value = self.memory.iter()
            .filter(|c| c.marked)
            .map(|c| c.value)
            .collect::<String>()
            .parse::<i64>()
            .unwrap_or(0);
       
        println!("Max Value Pt2: {}", max_value);
        // let mut other_index = 0;
        // let mut max_value = 0;
        // while current_index >= 11 {
        //     self.memory[current_index].marked = true;
        //     println!("Current Index: {}", current_index);
        //     other_index = current_index - 1;
        //     println!("Other Index Start: {}", other_index);
        //     if other_index - 10 >= 0 {
        //         while other_index >= current_index - 10 {
        //             println!("{}", other_index);
        //             self.memory[other_index].marked = true;
        //             if other_index == 0 {
        //                 println!("Reached zero");
        //                 break;
        //             }
        //             other_index -= 1;
        //         }
        //         println!("Marked up to index {}", self.memory.iter()
        //             .filter(|c| c.marked)
        //             .map(|c| c.value)
        //             .collect::<String>().len());

        //         let current_value: i64 = self.memory.iter()
        //             .filter(|c| c.marked)
        //             .map(|c| c.value)
        //             .collect::<String>()
        //             .parse::<i64>()
        //             .unwrap_or(0);

        //         println!("Current Value: {}", current_value);

        //         if current_value > max_value {
        //             max_value = current_value;
        //         }

        //         other_index = current_index - 1;
        //         while other_index > current_index - 11 {
        //             self.memory[other_index].marked = false;
        //             if other_index == 0 {
        //                 break;
        //             }

        //             other_index -= 1;
        //         }
        //     }

        //     self.memory[current_index].marked = false;
        //     current_index -= 1;
        //     if current_index == 10 {
        //         println!("Breaking out");
        //         break;
        //     }
        // }

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
        let mut bank = Bank::new(line);
        let max_value = bank.find_max();
        // println!("Max value for line {} is {}", line, max_value);
        answer += max_value;
    }   

    return answer;
}

fn pt2(filename : &str) -> u64 {
    let mut answer : u64= 0;
    let lines = read_lines(filename);

    for line in &lines {
        let mut bank = Bank::new(line);
        let max_value = bank.find_max_pt2();
        answer += max_value as u64;
    }   

    return answer;
}