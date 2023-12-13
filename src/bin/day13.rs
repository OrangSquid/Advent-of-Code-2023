use std::time::Instant;

use advent::readlines::read_lines;

fn check_reflection_column(square: &[String], column1: isize, column2: usize, smudge: bool) -> bool {
    if column1 < 0 || column2 >= square[0].len() {
        return smudge;
    }
    let column1_vec: String = square.iter().map(|line| line.chars().nth(column1 as usize).unwrap()).collect();
    let column2_vec: String = square.iter().map(|line| line.chars().nth(column2).unwrap()).collect();

    let count_diff = column1_vec.chars().zip(column2_vec.chars()).filter(|&(a, b)| a != b).count();

    if count_diff == 1 {
        if smudge {
            false
        } else {
            check_reflection_column(square, column1 - 1, column2 + 1, true)
        }
    } else {
        count_diff == 0 && check_reflection_column(square, column1 - 1, column2 + 1, smudge)
    }
}

fn check_reflection_line(square: &[String], line1: isize, line2: usize, smudge: bool) -> bool {
    if line1 < 0 || line2 >= square.len() {
        return smudge;
    }

    let count_diff = square[line1 as usize].chars().zip(square[line2].chars()).filter(|&(a, b)| a != b).count();

    match count_diff {
        1 => !smudge && check_reflection_line(square, line1 - 1, line2 + 1, true),
        0 => check_reflection_line(square, line1 - 1, line2 + 1, smudge),
        _ => false,
    }
}

fn main() {
    let time = Instant::now();
    let buffer: Vec<String> = read_lines("input/day13.txt");
    let mut buffer2: Vec<Vec<String>> = Vec::new();
    let mut buffer3: Vec<String> = Vec::new();
    let mut result = 0;

    for line in buffer {
        if line == "" {
            buffer2.push(buffer3.to_owned());
            buffer3.clear();
        } else {
            buffer3.push(line);
        }
    }

    'outer: for square in buffer2 {
        for i in 0..(square.len() - 1) {
            if check_reflection_line(&square, i as isize, i+1, false) {
                result += (i + 1) * 100;
                continue 'outer;
            }
        }
        for i in 0..(square[0].len() - 1) {
            if check_reflection_column(&square, i as isize, i+1, false) {
                result += i + 1;
                continue 'outer;
            }
        }
    }

    println!("{}", Instant::now().duration_since(time).as_secs_f64());
    println!("{}", result);
}
