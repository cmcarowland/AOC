use std::collections::HashSet;
use std::fmt::Debug;
use std::fmt;

use super::Point;
use super::Edge;

pub struct Section {
    c : char,
    pub members : HashSet<Point>,
    pub edges : HashSet<Edge>,
}

impl Section {
    pub fn new(c : char, location : Point) -> Section {
        Section {
            c,
            members : HashSet::from([location]),
            edges : HashSet::new(),
        }
    }

    pub fn get_plot(&self) -> char {
        self.c
    }

    pub fn calculate_area(&self) -> i64 {
        self.members.len() as i64
    }

    pub fn calculate_perimeter(&self) -> i64 {
        self.edges.len() as i64
    }

    pub fn calculate_sides(&self) -> i64 {
        let mut edges : Vec<&Edge> = self.edges.iter().collect();
        edges.sort();

        let mut sides = 0;
        let mut processed: Vec<&Edge> = Vec::new();
        let mut current_edge = edges[0];

        while processed.len() < edges.len() {
            let pos : Option<usize>;
            if processed.contains(&current_edge) {
                pos = edges.iter().position(|x| !processed.contains(x));
                if pos.is_none() {
                    println!("Disconnected Edge {}", current_edge);
                    break;
                }
                
                current_edge = edges[pos.unwrap()];
                continue;
            }

            pos = edges.iter().position(|x| 
                *x != current_edge && 
                (x.get_start() == current_edge.get_end() || x.get_end() == current_edge.get_end())
            );

            let i = pos.unwrap();
        
            if current_edge.change_direction(edges[i]) {
                sides += 1;
            }    
            
            processed.push(current_edge);
            current_edge = edges[i];
        }
        
        return sides;
    }
}

impl Debug for Section {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Section: {} Area: {}", self.c, self.calculate_area())
    }
}