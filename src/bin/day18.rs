use std::time::Instant;

use advent::readlines::read_lines;

fn fill_area(lagoon: Vec<(i64, i64)>) -> i64 {
    let mut result = 0;
    for i in 0..lagoon.len() {
        let (x_current, y_current) = lagoon[i];
        let (x_next, y_next) = lagoon[(i + 1) % lagoon.len()];
        result += (y_current - y_next) * (x_current + x_next);
    }
    result / 2
}

fn main() {
    let time = Instant::now();
    let buffer: Vec<String> = read_lines("input/day18.txt");
    let mut lagoon = Vec::new();
    let mut perimeter = 0;
    let mut current_location = (0, 0);

    for line in buffer {
        let mut it = line.split_whitespace();
        let direction = it.next().unwrap();
        let steps: i64 = it.next().unwrap().parse().unwrap();
        let color = i64::from_str_radix(it.next().unwrap().strip_prefix("(#").unwrap().strip_suffix(')').unwrap(), 16).unwrap();
        let direction = color & 0x3;
        let steps = color >> 4;

        let stepper_tuple = match direction {
            0 => (0, 1),
            1 => (1, 0),
            2 => (0, -1),
            3 => (-1, 0),
            _ => panic!()
        };

        current_location.0 += stepper_tuple.0 * steps;
        current_location.1 += stepper_tuple.1 * steps;
        lagoon.push(current_location);
        perimeter += steps;
    }

    let result = fill_area(lagoon) + (perimeter / 2 + 1);

    println!("{}", Instant::now().duration_since(time).as_secs_f64());
    println!("{}", result);
}
