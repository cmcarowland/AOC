mod map;
use map::map_data::MapData;
use map::map_data::Point;
use map::map_data::Edge;
use map::map_data::Section;
use std::collections::HashMap;
use std::fs;
use std::env;

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
    let mut plots : HashMap<char, i64> = HashMap::new();
    let mut map : MapData = MapData::new(lines.len() as i32, lines[0].len() as i32);
    map.create_grid(&lines);
    let mut sections : Vec<Section> = Vec::new();

    for i in 0..map.get_height() * map.get_width() {
        let current_pos = Point::new(i % map.get_height(), i / map.get_width());
        let mut section = Section::new(map.get(&current_pos), current_pos.clone());
        let _ = map.check_neighbors(&current_pos, &mut section);
        sections.push(section);
    }

    for section in sections {
        let area = section.calculate_area();
        let perimeter = section.calculate_perimeter();
        let val = plots.entry(section.get_plot()).or_insert(0);
        *val = *val + area * perimeter as i64;
    }

    return plots.values().sum();
}

fn pt2(filename : &str) -> i64 {
    let lines = read_lines(filename);
    let mut plots : Vec<(char, i64)> = Vec::new();
    let mut map : MapData = MapData::new(lines.len() as i32, lines[0].len() as i32);
    map.create_grid(&lines);
   
    let mut sections : Vec<Section> = Vec::new();
    for i in 0..map.get_height() * map.get_width() {
        let current_pos = Point::new(i % map.get_height(), i / map.get_width());
        if map.contains_checked(&current_pos) {
            continue;
        }
        let mut sec = Section::new(map.get(&current_pos), current_pos.clone());

        map.check_neighbors(&current_pos, &mut sec);
        sections.push(sec);
    }

    return sections.iter().map(|x| x.calculate_sides() * x.calculate_area()).sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pt21() {
        assert_eq!(pt2("test.txt"), 80);
    }
    
    #[test]
    fn test_pt22() {
        assert_eq!(pt2("test2.txt"), 436);
    }
   
    #[test]
    fn test_pt23() {
        assert_eq!(pt2("testE.txt"), 236);
    }
   
    #[test]
    fn test_pt24() {
        assert_eq!(pt2("test3.txt"), 1206);
    }

    #[test]
    fn test_pt25() {
        assert_eq!(pt2("test4.txt"), 368);
    }
}