use std::collections::HashMap;
use std::collections::HashSet;
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

fn pt1(filename : &str) -> i64 {
    let lines = read_lines(filename);
    let mut connections: HashMap<String, HashSet<String>> = HashMap::new();

    for line in lines {
        let mut parts = line.split("-");
        let host = parts.next().unwrap();
        let guest = parts.last().unwrap();

        if connections.contains_key(host) {
            connections.get_mut(host).unwrap().insert(guest.to_string());
        } else {
            let mut set = HashSet::new();
            set.insert(guest.to_string());
            connections.insert(host.to_string(), set);
        }
    }

    let mut threes : Vec<HashSet<String>> = Vec::new();
    for (host, guests) in &connections {
        // println!("{} : {:?}", host, guests);
        for guest in guests {
            // host < - > guest CONNECTED
            // Now look through guest connections and see if they are connected to host
            for third in connections.get(guest).unwrap() {
                // guest < - > third CONNECTED
                let mut add = false;
                
                if connections.get(host).unwrap().contains(third) || connections.get(third).unwrap().contains(host) {
                    add = true;
                }

                for x in &threes {
                    if x.contains(host) && x.contains(guest) && x.contains(third) {
                        add = false;
                        break;
                    }
                }

                if add {
                    threes.push(HashSet::from([host.clone(), guest.clone(), third.clone()]));
                }
                
            }
        }
    }

    // println!("\nResult\n=========");
    threes.retain(|x| x.iter().any(|s| s.starts_with('t')));
    
    // for x in &threes {
    //     println!("{:?}", x);
    // }

    return threes.len() as i64;
}

fn pt2(filename : &str) -> i64 {
    let lines = read_lines(filename);

    return 0;
}