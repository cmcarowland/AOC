use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please enter file path for data set");
    }

    pt1(&args[1]);
    pt2(&args[1]);
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in fs::read_to_string(filename).unwrap().lines() {
        result.push(line.to_string());
    }

    result
}

fn is_safe(values: &Vec<i64>) -> bool {
    let mut down : bool = false;
    if values[0] > values[1] {
        down = true;
    }

    for i in 0..values.len() - 1 {
        //println!("{} {}", values[i], values[i + 1]);
        let diff : i64 = (values[i] - values[i + 1]).try_into().unwrap();
        //println!("Diff: {}", diff);

        if down {
            if diff <= 0 || diff > 3 {
                return false
            }
        } else {
            if diff >= 0 || diff < -3 {
                return false
            }
        }
    }

    true
}

fn pt1(filename : &str) {
    let lines = read_lines(filename);
    let mut safe_lines : usize = 0;

    for line in lines.iter() {
        let parts : Vec<i64> = line.split(" ").map(|n| n.parse::<i64>().unwrap()).collect();
        if is_safe(&parts) {
            safe_lines += 1;
            //println!("Safe : {}", line);
        }
    }

    println!("Safe Lines Pt 1 : {}", safe_lines);
}

fn pt2(filename : &str) {
    let lines = read_lines(filename);
    let mut safe_lines : usize = 0;

    for line in lines.iter() {
        let parts : Vec<i64> = line.split(" ").map(|n| n.parse::<i64>().unwrap()).collect();
        if is_safe(&parts) {
            safe_lines += 1;
            // println!("Safe : {}", line);
            continue;
        }
        for i in 0..parts.len() {
            let mut local = parts.clone();
            local.remove(i);
            // for x in local.iter() {
            //     print!("{} ", x);
            // }
            // println!();
            if is_safe(&local) {
                safe_lines += 1;
                //println!("Safe : {}", line);
                break;
            }
        }
    }

    println!("Safe Lines Pt 2 : {}", safe_lines);
}