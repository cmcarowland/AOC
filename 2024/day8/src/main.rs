mod map;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::hash::Hash;

use map::map_data::MapData;
use map::map_data::Vector2;

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

fn find_antenna_chars(hash: &mut HashMap<char, usize>, map : &MapData) {
    for r in 0..map.get_height() {
        for c in 0..map.get_width() {
            let c = map.grid[c as usize][r as usize];
            if c == '.' {
                continue;
            }
            if hash.contains_key(&c) {
                let count = hash.get(&c).unwrap();
                hash.insert(c, count + 1);
            } else {
                hash.insert(c, 1);
            }
        }
    }
}

fn pt1(filename : &str) -> i64 {
    let lines = read_lines(filename);
    let mut unique_characters : HashMap<char, usize>= HashMap::new();
    let mut annodeLocations : HashMap<Vector2, usize>= HashMap::new();
    let mut map = MapData::new(lines.len() as i32, lines[0].len() as i32);
    map.create_grid(&lines);
    map.print_map();
    find_antenna_chars(&mut unique_characters, &map);
    // println!("{:?}", unique_characters);
    for k in unique_characters.keys() {
        let antennas : Vec<Vector2> = map.find_vals(*k).iter().filter(|&x| map.get_item_from_location(*x) == *k).map(|&x| x).collect();
        for i in 0..antennas.len() {
            // println!("Antenna {} at : {}", *k, antennas[i]);
            for j in 0..antennas.len() {
                if i == j {
                    continue;
                }
                
                let distance = antennas[i].dist(&antennas[j]);
                // println!("Distance from {} to {} is {}", antennas[i], antennas[j], distance);
                let base =  antennas[i].add(&distance.reverse());
                // println!("Annode will be at {}", base);
                let count = annodeLocations.get(&base).unwrap_or(&0) + 1;
                annodeLocations.insert(base.clone(), count);
                let second  = antennas[j].add(&distance);
                // println!("Annode will be at {}", second);
                let count = annodeLocations.get(&second).unwrap_or(&0) + 1;
                annodeLocations.insert(second.clone(), count);
            }
        }

    }

    let mut bad_keys: Vec<Vector2> = vec![];
    for (k,v) in annodeLocations.iter() {
        // println!("{} : {}", k, v);
        if map.set_item_at_location(*k, '#') == false {
            bad_keys.push(*k);
        }
    }

    for k in bad_keys {
        annodeLocations.remove(&k);
    }
    println!();
    map.print_map();
    println!();

    return annodeLocations.keys().len() as i64;
}

fn pt2(filename : &str) -> i64{
    let lines = read_lines(filename);

    return  0;
}