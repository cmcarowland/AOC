use core::hash;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::hash::{Hash, Hasher, DefaultHasher};

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

struct Key {
    bumps : Vec<i64>
}

impl Hash for Key {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for bump in self.bumps.iter().rev() {
            bump.hash(state);
        }
    }
}

#[derive(Eq, PartialEq)]
struct Lock {
    pins : Vec<i64>
}

impl Hash for Lock {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for pin in &self.pins {
            let pin_offset = 7 - *pin;
            pin_offset.hash(state);
        }
    }
}


fn find_next_lock(lines: &Vec<String>, mut start: usize) -> (Lock, usize) {
    let mut result = Lock { pins : Vec::new() };
    while lines[start].chars().filter(|x| *x == '#').count() != lines[start].chars().count() {
        start += 8;
        if start >= lines.len() {
            return (result, start);
        }
    }

    result = Lock { pins : vec!(0; lines[start].chars().count()) };
    start += 1;
    let mut processed = 0;
    while processed < 5 {
        for pin in 0..lines[start].chars().count() {
            if lines[start].chars().nth(pin).unwrap() == '#' {
                result.pins[pin] += 1;
            } 
        }
        start += 1;
        processed += 1;
        if start >= lines.len() {
            break;
        }
    }

    (result, start + 2)
}

fn find_next_key(lines: &Vec<String>, mut start: usize) -> (Key, usize) {
    let mut result = Key { bumps : Vec::new() };
    while lines[start].chars().filter(|x| *x == '.').count() != lines[start].chars().count() {
        start += 8;
        if start >= lines.len() {
            return (result, start);
        }
    }

    result = Key { bumps : vec!(0; lines[start].chars().count()) };
    start += 1;
    let mut processed = 0;
    while processed < 5 {
        for bump in 0..lines[start].chars().count() {
            if lines[start].chars().nth(bump).unwrap() == '#' {
                result.bumps[bump] += 1;
            } 
        }
        start += 1;
        processed += 1;
        if start >= lines.len() {
            break;
        }
    }

    (result, start + 2)
}

fn pt1(filename : &str) -> i64 {
    let lines = read_lines(filename);
    let mut locks: HashSet<Lock> = HashSet::new();
    let mut start: usize = 0;
    let mut lock = Lock { pins : Vec::new() };
    let mut answer = 0;

    loop {
        if start >= lines.len() {
            break;
        }
        (lock, start) = find_next_lock(&lines, start);
        // println!("Start {}", start);
        // println!("Lock {:?}", lock.pins);
        if lock.pins.len() > 0 {
            locks.insert(lock);
        } else {
            break;
        }
    }
    
    let mut hasher = DefaultHasher::new();
    start = 0;
    loop {
        if start >= lines.len() {
            break;
        }

        let (key, new_start) = find_next_key(&lines, start);
        start = new_start;
        // println!("Start {}", start);
        // println!("Key {:?}", key.bumps);
        if key.bumps.len() > 0 {
            key.hash(&mut hasher);
            let key_hash = hasher.finish();
            for lock in &locks {
                // hasher = DefaultHasher::new();
                // lock.hash(&mut hasher);
                // let lock_hash = hasher.finish();
                // println!("Key hash : {}", key_hash);
                // println!("Lock hash : {}", lock_hash);
                // println!("EQ? : {}", key_hash == lock_hash);
                // println!("Testing {:?} {:?}", lock.pins, key.bumps);
                let mut correct = true;
                for i in 0..key.bumps.len() {
                    if key.bumps[i] + lock.pins[i] >= 6 {
                        correct = false;
                        break;
                    }
                }

                if correct {
                    // println!("Found match {:?} {:?}", lock.pins, key.bumps);
                    answer += 1;
                }
            }
        } else {
            break;
        }
    }
    
    // let key = Key { bumps : [8, 7, 6, 5, 4, 3, 2, 1].to_vec() };
    // let lock = Lock { pins : [1, 2, 3, 4, 5, 6, 7, 8].to_vec() };
    // key.hash(&mut hasher);
    // let key_hash = hasher.finish();
    // let mut hasher = DefaultHasher::new();
    // lock.hash(&mut hasher);
    // let lock_hash = hasher.finish();
    // println!("Key hash : {}", key_hash);
    // println!("Lock hash : {}", lock_hash);
    // println!("EQ? : {}", key_hash == lock_hash);


    return answer;
}

fn pt2(filename : &str) -> i64 {
    let lines = read_lines(filename);

    return 0;
}