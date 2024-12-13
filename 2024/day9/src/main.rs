use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please enter file path for data set");
    }

    println!("{}", pt1(&args[1]));
    pt2(&args[1]);
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in fs::read_to_string(filename).unwrap().lines() {
        result.push(line.to_string());
    }

    result
}

fn checksum(disk : &Vec<i64>) -> u64 {
    let mut total : u64 = 0;

    for i in 0..disk.len() {
        total += i as u64 * disk[i] as u64;
    }

    return total;
}

fn compact(disk : &mut Vec<i64>) {
    let mut start: usize = 0;
    let mut end : usize = disk.len() - 1;
    while start < end {
        while disk[start] != -1 && start < end {
            start += 1;
        }
        while disk[end] == -1  && end > start{
            end -= 1;
        }

        disk[start] = disk[end];
        disk[end] = -1;
    }
}

fn print_disk(disk : &Vec<i64>) {
    for i in disk {
        if *i == -1 {
            print!(".", );
        } else {
            print!("{}", *i);
        }
    }
    println!();
}

fn pt1(filename : &str) -> u64 {
    let lines = read_lines(filename);
    let mut id = 0;
    let line = lines[0].clone();

    let mut disk: Vec<i64> = Vec::new();
    for i in 0..(line.len() as i64) {
        for _ in 0..line.chars().nth(i as usize).unwrap().to_string().parse::<i64>().unwrap() {
            if i % 2 == 0 {
                disk.push(id);
            } else {
                disk.push(-1);
            }
        }
        if i % 2 == 0 {
            id += 1;
        }
    }

    // print_disk(&disk);
    compact(&mut disk);
    disk = disk.into_iter().filter(|x| *x != -1).collect();
    print_disk(&disk);

    return checksum(&disk);//.into_iter().map(|x| x.to_string()).into_iter().collect::<String>());
}

fn pt2(filename : &str) {
    let _lines = read_lines(filename);
}