use core::panic;
use std::{time::Instant, collections::HashMap, ops::Add};

use advent::readlines::read_lines;

fn count_valid_spring(springs: &str, numbers: &[u64], cache: &mut HashMap<(String, Vec<u64>, u64), u64>, spring_count: u64) -> u64 {
    match cache.get(&(springs.to_owned(), numbers.to_vec(), spring_count)) {
        Some(i) => *i,
        None => {
            let sliced_springs = springs.get(1..);
            match springs.chars().next() {
                Some('?') => {
                    let mut result = 0;
                    if numbers.len() != 0 {
                        if spring_count == numbers[0] {
                            result = count_valid_spring(sliced_springs.unwrap(), numbers.get(1..).unwrap(), cache, 0);
                        } else if spring_count == 0 {
                            result = count_valid_spring(sliced_springs.unwrap(), numbers, cache, spring_count);
                            result += count_valid_spring(sliced_springs.unwrap(), numbers, cache, spring_count + 1);
                        } else if spring_count < numbers[0] {
                            result = count_valid_spring(sliced_springs.unwrap(), numbers, cache, spring_count + 1);
                        }
                    } else {
                        result = count_valid_spring(sliced_springs.unwrap(), numbers, cache, spring_count);
                    }
                    cache.insert((springs.to_owned(), numbers.to_vec(), spring_count), result);
                    result
                }
                Some('#') => {
                    let mut result = 0;
                    if numbers.len() != 0 && spring_count < numbers[0] {
                        result = count_valid_spring(sliced_springs.unwrap(), numbers, cache, spring_count + 1);
                    }
                    cache.insert((springs.to_owned(), numbers.to_vec(), spring_count), result);
                    result
                },
                Some('.') => {
                    let mut result = 0;
                    if spring_count != 0 {
                        if numbers.len() > 0 && numbers[0] == spring_count {
                            result = count_valid_spring(sliced_springs.unwrap(), numbers.get(1..).unwrap(), cache, 0);
                        }
                    } else {
                        result = count_valid_spring(sliced_springs.unwrap(), numbers, cache, 0);
                    }
                    cache.insert((springs.to_owned(), numbers.to_vec(), spring_count), result);
                    result
                },
                None => {
                    if numbers.len() == 0 && spring_count == 0 {
                        1
                    } else if numbers.len() == 1 && spring_count == numbers[0] {
                        1
                    } else {
                        0
                    }
                }
                _ => panic!()
            }
        },
    }
}



fn main() {
    let time = Instant::now();
    let buffer: Vec<String> = read_lines("input/day12.txt");
    let mut result = 0;
    let mut cache = HashMap::new();

    for line in buffer {
        let mut it = line.split_whitespace();
        let mut springs = it.next().unwrap().to_owned().add("?").repeat(5);
        springs.remove(springs.len() - 1);
        let numbers: Vec<u64> = it
            .next()
            .unwrap()
            .split(',')
            .map(|number| number.parse().unwrap())
            .collect();

        let numbers = numbers.repeat(5);

        result += count_valid_spring(&springs, &numbers, &mut cache, 0);
    }

    println!("{}", Instant::now().duration_since(time).as_secs_f64());
    println!("{}", result);
}
