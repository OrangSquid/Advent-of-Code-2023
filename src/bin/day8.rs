use std::{time::Instant, collections::HashMap};
use num::integer::lcm;

use advent::readlines::read_lines;

struct Directions {
    left: String,
    right: String
}

fn main() {
    let time = Instant::now();
    let buffer = read_lines("input/day8.txt");
    let mut directions = HashMap::new();
    let mut lines = buffer.iter();
    let mut guidance = lines.next().unwrap().chars().cycle();
    lines.next();

    for line in lines {
        let mut splitted = line.split(" = ");
        let origin = splitted.next().unwrap();
        let destinations = splitted.next().unwrap();
        let (left, right) = destinations.split_once(", ").unwrap();
        let (_, left_trimmed) = left.split_once('(').unwrap();
        let (right_trimmed, _) = right.split_once(')').unwrap();

        let direction = Directions {
            left: left_trimmed.to_owned(),
            right: right_trimmed.to_owned()
        };

        directions.insert(origin.to_owned(), direction);
    }

    let mut cycles: Vec<u64> = Vec::new();

    for mut node in directions.keys().map(|i| i.clone()) {
        if node.ends_with('A') {
            let mut result = 0;
            while !node.ends_with('Z') {
                let turn = guidance.next().unwrap();
                if turn == 'L' {
                    node = directions.get(&node).unwrap().left.clone();
                } else {
                    node = directions.get(&node).unwrap().right.clone();
                }
                result += 1;
            }
            cycles.push(result);
        }
    }

    println!("{:#?}", cycles);

    let mut result = 1;
    for cycle in cycles {
        result = lcm(result, cycle);
    }

    println!("{}", Instant::now().duration_since(time).as_secs_f64());
    println!("{}", result);
}