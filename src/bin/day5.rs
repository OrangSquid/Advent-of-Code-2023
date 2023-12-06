use std::{ops::Range, time::Instant};

use advent::readlines::read_lines;

fn match_seeds(seeds: &Vec<Range<i64>>, soil_map: &Vec<(Range<i64>, i64)>) -> Vec<Range<i64>>{
    if seeds.is_empty() {
        return Vec::new();
    }
    let mut it = seeds.iter();
    let mut result = Vec::new();
    let mut iterate_again = Vec::new();
    
    'outer: while let Some(seed) = it.next() {
        for (source, diff) in soil_map {
            let mut i = 0;
            let mut j = 0;
            if source.contains(&seed.start) && source.contains(&(seed.end - 1)) {
                i = seed.start + diff;
                j = seed.end + diff;
                result.push(i..j);
                continue 'outer;
            } else if source.contains(&seed.start){
                i = source.end;
                j = seed.end;
                iterate_again.push(i..j);
                i = seed.start + diff;
                j = source.end + diff;
                result.push(i..j);
                continue 'outer;
            } else if source.contains(&(seed.end - 1)) {
                i = seed.start;
                j = source.start;
                iterate_again.push(i..j);
                i = source.start + diff;
                j = seed.end + diff;
                result.push(i..j);
                continue 'outer;
            } else if source.start > seed.start && source.end < seed.end {
                i = seed.start;
                j = source.start;
                iterate_again.push(i..j);
                i = source.end;
                j = seed.end;
                iterate_again.push(i..j);
                i = source.start + diff;
                j = source.end + diff;
                result.push(i..j);
                continue 'outer;
            }
        }
        result.push(seed.clone());
    }
    result = [result, match_seeds(&iterate_again, soil_map)].concat();
    result
}

fn main() {
    let time = Instant::now();
    let buffer = read_lines("input/day5.txt");
    let mut seeds: Vec<Range<i64>> = Vec::new();
    let mut soil_map: Vec<(Range<i64>, i64)> = Vec::new();    

    let seeds_str = buffer[0].split_whitespace().collect::<Vec<&str>>();
    let mut it = seeds_str.iter();
    it.next();

    while let (Some(seed), Some(seed2)) = (it.next(), it.next()) {
        let i = seed.parse::<i64>().unwrap();
        let j = seed2.parse::<i64>().unwrap();
        seeds.push(i..i+j);
    }

    let mut it2 = buffer.iter();
    it2.next();
    it2.next();
    it2.next();

    while let Some(line) = it2.next() {
        if line.ends_with("map:") {
            seeds = match_seeds(&seeds, &soil_map);
            soil_map.clear();
            continue;
        }
        let mut numbers = line.split_ascii_whitespace().peekable();
        if numbers.peek().is_none() {
            continue;
        }
        let destination = numbers.next().unwrap().parse::<i64>().unwrap();
        let source = numbers.next().unwrap().parse::<i64>().unwrap();
        let range = numbers.next().unwrap().parse::<i64>().unwrap();
        let diff = destination - source;

        soil_map.push((source..source+range, diff));
    }

    seeds = match_seeds(&seeds, &soil_map);
    println!("{}", Instant::now().duration_since(time).as_secs_f64());
    println!("{}", seeds.iter().map(|range| range.start).min().unwrap());
}