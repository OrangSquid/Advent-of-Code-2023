use std::time::Instant;

use advent::readlines::read_lines;
use num::abs;

#[derive(Debug)]
struct Galaxy {
    x: i64,
    y: i64
}

impl Galaxy {
    pub fn distance(&self, other: &Galaxy) -> i64 {
        return abs(self.x - other.x) + abs(self.y - other.y)
    }
}

fn main() {
    let time = Instant::now();
    let buffer: Vec<Vec<char>> = read_lines("input/day11.txt").iter().map(|i| i.chars().collect()).collect();
    let mut galaxies: Vec<Galaxy> = Vec::new();
    let mut x_compensation: Vec<i64> = Vec::new();
    let mut y_compensation: Vec<i64> = Vec::new();
    let mut cumulative_x_compensation = 0;
    let mut cumulative_y_compensation = 0;


    for line in buffer.iter() {
        if line.iter().all(|cell| *cell == '.') {
            cumulative_x_compensation += 999999;
        }
        x_compensation.push(cumulative_x_compensation);
    }

    for column in 0..buffer[0].len() {
        if buffer.iter().all(|row| row[column] == '.') {
            cumulative_y_compensation += 999999;
        }
        y_compensation.push(cumulative_y_compensation);
    }

    for (x, line) in buffer.iter().enumerate() {
        for (y, _) in line.iter().enumerate().filter(|&(_, cell)| *cell == '#') {
            galaxies.push(Galaxy { x: x as i64 + x_compensation[x], y: y as i64 + y_compensation[y] });
        }
    }

    let mut result = 0;
    for i in 0..galaxies.len() {
        for j in i+1..galaxies.len() {
            result += galaxies[i].distance(&galaxies[j]);
        }
    }

    println!("{}", Instant::now().duration_since(time).as_secs_f64());
    println!("{:#?}", result);
}