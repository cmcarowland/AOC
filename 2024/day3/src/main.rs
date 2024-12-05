use std::env;
use std::fs;
use std::thread;
use std::time;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please enter file path for data set");
    }

    pt1(&args[1]);
    pt2(&args[1]);
}

fn pt1(filename : &str) {
    let input = fs::read_to_string(filename).expect("Invalid File");
    let re = Regex::new(r"(?:mul)(?:\({1})(?<firstNum>\d{1,3}),(?<secondNum>\d{1,3})(?:\){1})").unwrap();
    let mut total : u64 = 0;

    for (err, [first, second]) in re.captures_iter(&input).map(|c| c.extract()) {
        if err.eq("") {
            println!("{}", err);
            continue;
        }
        
        total += first.parse::<u64>().unwrap() * second.parse::<u64>().unwrap();
    }

    println!("{}", total);
}

fn pt2(filename : &str) {
    let mut total : u64 = 0;
    let mut enabled : bool = true;

    let input = fs::read_to_string(filename).expect("Invalid File");
    let re = Regex::new(r"(?:mul)(?:\({1})(?<firstNum>\d{1,3}),(?<secondNum>\d{1,3})(?:\){1})").unwrap();
    let do_enable = Regex::new(r"do\(\)").unwrap();
    let do_disable = Regex::new(r"don't\(\)").unwrap();
    let mut first : &str;
    let mut second : &str;

    let mut i : usize = 0;
    while i < input.len() {
        println!("i: {} total: {}", i, total);
        // println!("{}", &input[i..]);
        let next_mul = re.find(&input[i..]);
        let next_do = do_enable.find(&input[i..]);
        let next_dont = do_disable.find(&input[i..]);


        if enabled {
            if next_mul != None {
                if next_dont != None {
                    if next_mul.unwrap().start() < next_dont.unwrap().start() {
                        (_, [first, second]) = re.captures(&input[i..]).map(|c| c.extract()).unwrap();
                        println!("[*] Perform Mul {} {}", first, second);
                        total += first.parse::<u64>().unwrap() * second.parse::<u64>().unwrap();
                        i += next_mul.unwrap().end();
                        continue;
                    } else {
                        println!("[-] Disable In Mul");
                        enabled = false;
                        i += next_dont.unwrap().end();
                        continue;
                    }
                } else {
                    (_, [first, second]) = re.captures(&input[i..]).map(|c| c.extract()).unwrap();
                    println!("[*] Perform Mul {} {}", first, second);
                    total += first.parse::<u64>().unwrap() * second.parse::<u64>().unwrap();
                    i += next_mul.unwrap().end();
                    continue;
                }
            } else if next_dont != None {
                println!("[-] Disable Free");
                enabled = false;
                i += next_dont.unwrap().end();
                continue;
            }
        } else {
            if next_do != None {
                println!("[+] Enable");
                enabled = true;
                i += next_do.unwrap().end();
                continue;
            }
        }

        i += 1;
        // thread::sleep(time::Duration::from_millis(100));
    }

    println!("{}", total);
}