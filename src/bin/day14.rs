use std::{time::Instant, collections::HashMap};

use advent::readlines::read_lines;

fn roll_north(square: &mut Vec<Vec<char>>) {
    for row in 2..(square.len() - 1) {
        for column in 1..(square[0].len() - 1) {
            if square[row][column] != 'O' {
                continue;
            }
            for i in (1..=row).rev() {
                if square[i - 1][column] != '.' {
                    square[row][column] = '.';
                    square[i][column] = 'O';
                    break;
                }
            }
        }
    }
}

fn roll_south(square: &mut Vec<Vec<char>>) {
    for row in (1..(square.len() - 2)).rev() {
        for column in 1..(square[0].len() - 1) {
            if square[row][column] != 'O' {
                continue;
            }
            for i in row..(square.len() - 1) {
                if square[i + 1][column] != '.' {
                    square[row][column] = '.';
                    square[i][column] = 'O';
                    break;
                }
            }
        }
    }
}

fn roll_west(square: &mut Vec<Vec<char>>) {
    for column in 2..(square[0].len() - 1) {
        for row in 1..square.len() {
            if square[row][column] != 'O' {
                continue;
            }
            for i in (1..=column).rev() {
                if square[row][i - 1] != '.' {
                    square[row][column] = '.';
                    square[row][i] = 'O';
                    break;
                }
            }
        }
    }
}

fn roll_east(square: &mut Vec<Vec<char>>) {
    for column in (1..(square[0].len() - 2)).rev() {
        for row in 1..square.len() {
            if square[row][column] != 'O' {
                continue;
            }
            for i in column..(square[0].len() - 1) {
                if square[row][i + 1] != '.' {
                    square[row][column] = '.';
                    square[row][i] = 'O';
                    break;
                }
            }
        }
    }
}

fn main() {
    let time = Instant::now();
    let mut buffer: Vec<Vec<char>> = read_lines("input/day14.txt").iter().map(|x| x.chars().collect()).collect();
    buffer.insert(0, "#".repeat(buffer[0].len()).chars().collect());
    buffer.push("#".repeat(buffer[0].len()).chars().collect());
    for line in buffer.iter_mut() {
        line.insert(0, '#');
        line.push('#');
    }
    let mut repeat = HashMap::new();

    let mut diff = 1;
    let mut finished = 0;
    for i in 1..=1000000000 { 
        roll_north(&mut buffer);
        roll_west(&mut buffer);
        roll_south(&mut buffer);
        roll_east(&mut buffer);
        match repeat.insert(buffer.clone(), i) {
            Some(lmao) => {
                repeat.insert(buffer.clone(), lmao);
                finished = lmao;
                diff = i - lmao;
                break;
            },
            _ => ()
        }
    }

    for i in 0..((1000000000 - finished) % diff) { 
        roll_north(&mut buffer);
        roll_west(&mut buffer);
        roll_south(&mut buffer);
        roll_east(&mut buffer);
    }

    let mut result = 0;
    let mut load: isize = (buffer.len() - 1) as isize;
    for line in buffer.iter() {
        result += line.iter().filter(|x| **x == 'O').count() * load as usize;
        load -= 1;
    }

    println!("{}", Instant::now().duration_since(time).as_secs_f64());
    println!("{}", result);
}
