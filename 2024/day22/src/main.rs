use std::env;
use std::fs;

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

/*
In particular, each buyer's secret number evolves into the next secret number in the sequence via the following process:

Calculate the result of multiplying the secret number by 64. Then, mix this result into the secret number. Finally, prune the secret number.
Calculate the result of dividing the secret number by 32. Round the result down to the nearest integer. Then, mix this result into the secret number. Finally, prune the secret number.
Calculate the result of multiplying the secret number by 2048. Then, mix this result into the secret number. Finally, prune the secret number.
*/

fn mix(val : i64, mix : i64) -> i64 {
    return val ^ mix;
}

fn prune(val : i64) -> i64 {
    return val % 16777216;
}

fn calculate_next(val : i64) -> i64 {
    //0x1000000
    // 0xffffff
    let times64 = val * 64;
    let mixed = mix(val, times64);
    let pruned = prune(mixed);
    let div32 = pruned / 32;
    let mixed = mix(pruned, div32);
    let pruned = prune(mixed);
    let times2048 = pruned * 2048;
    let mixed = mix(pruned, times2048);
    let pruned = prune(mixed);

    return pruned;
}

/*
1: 8685429
10: 4700978
100: 15273692
2024: 8667524
*/

fn pt1(filename : &str) -> i64 {
    let lines = read_lines(filename);
    let mut calculated_values : Vec<i64> = Vec::new();

    for line in lines {
        let mut val = line.parse::<i64>().unwrap();
        // print!("{} -> ", val);
        for _ in 0..2000 {
            let next = calculate_next(val);
            val = next;
        }
        calculated_values.push(val);
        // println!("{}", val);
    }

    return calculated_values.iter().sum();
}

fn pt2(filename : &str) -> i64 {
    let lines = read_lines(filename);

    return 0;
}