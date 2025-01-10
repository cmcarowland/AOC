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

fn check_line(line : &str, patterns : &Vec<&str>, possible_designs : &mut Vec<String>) -> bool {
    let mut end = line.len();
    for pattern in patterns.iter() {
        if line.to_string().ends_with(pattern) {
            if (end - pattern.len()) as i64 > 0 {
                if check_line(&line[..end - pattern.len()], patterns, possible_designs) {
                    return true;
                }
            } else {
                return true;
            }
        }
    }

    return false;
}
    
fn pt1(filename : &str) -> i64 {
    let lines = read_lines(filename);
    let mut patterns = lines[0].split(",").map(|x| x.trim()).collect::<Vec<&str>>();
    patterns.sort_by(|a, b| b.len().cmp(&a.len()));
    println!("{:?}", patterns);
    // return 0;
    let mut possible_designs : Vec<String> = Vec::new();
    let mut total = 0;

    for line in lines[2..].iter() {
        let max = patterns[0].len();

        if !check_line(&line, &patterns, &mut possible_designs) {
            println!("**** INVALID {}", line);
            // let mut last = 0;
            // for pd in possible_designs.iter() {
            //     // println!("{}{}", String::from_utf8(vec![b' '; last as usize]).unwrap(), pd);
            //     last += pd.len();
            // }
        } else {
            total += 1;
            println!("+++++ VALID: {}", line);
        }
        // possible_designs.clear();
    }

    return total;
}

fn pt2(filename : &str) -> i64 {
    let lines = read_lines(filename);

    return 0;
}