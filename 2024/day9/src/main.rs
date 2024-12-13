use std::env;
use std::fs;
use std::ptr::null;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please enter file path for data set");
    }

    //println!("{}", pt1(&args[1]));
    println!("{}", pt2(&args[1]));
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
        if disk[i] == -1 {
            continue;
        }
        total += i as u64 * disk[i] as u64;
    }

    return total;
}

fn compact(disk : &mut Vec<i64>) {
    let mut start_null : usize = 0;
    let mut end_null : usize = 0;
    
    let mut start_data : usize = disk.len() - 1;
    let mut end_data : usize = disk.len() - 1;
    
    while end_data > 0 {
        // println!("{}", end_data);
        while disk[end_data] == -1  && end_data > 0{
            end_data -= 1;
        }
        start_data = end_data;
        while start_data > 0 && disk[start_data - 1] == disk[end_data] {
            start_data -= 1;
        }
        let data_len = end_data - start_data + 1;
        // println!("Data {} Len {}", disk[start_data], data_len);
        let mut null_len = 0;
        while null_len < data_len && start_null < start_data {
            // println!("Data I {} Null I {}", start_data, start_null);
            // println!("Lookin for null slot {} {}", disk[start_null], start_null);
            while start_null < disk.len() && disk[start_null] != -1 {
                start_null += 1;
            }
            end_null = start_null + 1;
            while end_null < disk.len() && disk[end_null] == -1 {
                end_null += 1;
            }

            null_len = end_null - start_null;
            if null_len < data_len {
                start_null += 1;
            }
        }
        // println!("{} {}", start_null, start_data);
        if null_len >= data_len && start_null < start_data {
            for i in 0..data_len {
                disk[start_null + i] = disk[start_data + i];
                disk[start_data + i] = -1;
            }
        }
        // print_disk(&disk);
        if start_data == 0 {
            break;
        }

        end_data = start_data - 1;
        start_null = 0;
        end_null = 0;
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
    //disk = disk.into_iter().filter(|x| *x != -1).collect();
    print_disk(&disk);

    return checksum(&disk);//.into_iter().map(|x| x.to_string()).into_iter().collect::<String>());
}

fn pt2(filename : &str) -> u64 {
    let _lines = read_lines(filename);

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
    // print_disk(&disk);

    return checksum(&disk);
}