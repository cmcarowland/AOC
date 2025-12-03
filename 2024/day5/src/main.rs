use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please enter file path for data set");
    }

    println!("Part 1: {}", pt1(&args[1]));
    println!("Part 2: {}", pt2(&args[1]));
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in fs::read_to_string(filename).unwrap().lines() {
        result.push(line.to_string());
    }

    result
}

fn pt1(filename : &str) -> usize{
    let lines = read_lines(filename);
    let mut entries : HashMap<usize, Vec<usize>> = HashMap::new();
    let mut manuals : Vec<String> = Vec::new();
    let mut in_rules : bool = false;

    for line in lines {
        if line == "" {
            in_rules = true;
            continue;
        }

        if in_rules {
            manuals.push(line);
        } else {
            let split_line  : Vec<&str>= line.split('|').collect();
            let page = split_line[0].parse::<usize>().unwrap();

            if entries.contains_key(&page) {
                entries.get_mut(&page).unwrap().push(split_line[1].parse::<usize>().unwrap());
            } else {
                entries.insert(page, vec![split_line[1].parse::<usize>().unwrap()]);
            }
        }
    }

    let mut total : usize = 0;
    for me in manuals {
        // println!("{}", me);
        total += order_good(&me, &entries);
    }

    return total;
    
}

fn order_good(me : &String, entries : &HashMap<usize, Vec<usize>>) -> usize {
    let split_line  : Vec<usize> = me.split(',').map(|x| x.parse::<usize>().unwrap()).collect();
    for i in 0..split_line.len() {
        // println!("{} {} {}", i, &split_line[i], entries.contains_key(&split_line[i]));
        if entries.contains_key(&split_line[i]) {
            for entry in entries.get(&split_line[i]).unwrap() {
                // println!("{}", entry);
                if split_line.contains(entry) {
                    let me_index = split_line.iter().position(|x| *x == split_line[i]).unwrap();
                    let o_index = split_line.iter().position(|x| *x == *entry).unwrap();
                    // println!("{} < {}", o_index, me_index);
                    if o_index < me_index {
                        return 0;
                    }
                }
            }
        }
    }

    let x = split_line.len() / 2;
    return split_line[x];
}

fn order_pages(mut split_line : Vec<usize>, entries : &HashMap<usize, Vec<usize>>) -> usize {
    for i in 0..split_line.len() {
        // println!("{} {} {}", i, &split_line[i], entries.contains_key(&split_line[i]));
        if entries.contains_key(&split_line[i]) {
            for entry in entries.get(&split_line[i]).unwrap() {
                // println!("{}", entry);
                if split_line.contains(entry) {
                    let o_index = split_line.iter().position(|x| *x == *entry).unwrap();
                    // println!("{} < {}", o_index, me_index);
                    if o_index < i {
                        let tmp = split_line[i];
                        split_line[i] = *entry;
                        split_line[o_index] = tmp;
                        return order_pages(split_line, entries);
                    }
                }
            }
        }
    }

    // for i in split_line {
    //     print!("{} ", i);
    // }
    // println!();
    let x = split_line.len() / 2;
    return split_line[x];
}

fn pt2(filename : &str) -> usize{
    let _lines = read_lines(filename);

    let lines = read_lines(filename);
    let mut entries : HashMap<usize, Vec<usize>> = HashMap::new();
    let mut manuals : Vec<String> = Vec::new();
    let mut in_rules : bool = false;

    for line in lines {
        if line == "" {
            in_rules = true;
            continue;
        }

        if in_rules {
            manuals.push(line);
        } else {
            let split_line  : Vec<&str>= line.split('|').collect();
            let page = split_line[0].parse::<usize>().unwrap();

            if entries.contains_key(&page) {
                entries.get_mut(&page).unwrap().push(split_line[1].parse::<usize>().unwrap());
            } else {
                entries.insert(page, vec![split_line[1].parse::<usize>().unwrap()]);
            }
        }
    }

    let mut total : usize = 0;
    for me in manuals {
        // println!("{}", me);
        if 0 == order_good(&me, &entries) {
            let split_line  : Vec<usize> = me.split(',').map(|x| x.parse::<usize>().unwrap()).collect();
            total += order_pages(split_line, &entries);
        }
    }

    return total;
}