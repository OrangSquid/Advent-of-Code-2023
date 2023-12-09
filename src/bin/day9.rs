use std::time::Instant;

use advent::readlines::read_lines;


fn main() {
    let time = Instant::now();
    let buffer = read_lines("input/day9.txt");
    let mut result1 = 0;
    let mut result2 = 0;

    for line in buffer {
        let mut numbers: Vec<Vec<i64>> = vec![Vec::new(); 1];
        for number in line.split_whitespace() {
            numbers[0].push(number.parse().unwrap());
        }
        while numbers.last().unwrap().iter().any(|x| *x != 0) {
            let mut temp = Vec::new();
            let last_vec = numbers.last().unwrap();
            for i in 0..last_vec.len() - 1 {
                temp.push(last_vec[i + 1] - last_vec[i]);
            }
            numbers.push(temp);
        }
        let mut temp_result2 = 0;
        for line in numbers.iter() {
            result1 += line.last().unwrap();
        }
        for line in numbers.iter().rev() {
            temp_result2 = line.first().unwrap() - temp_result2; 
        }
        result2 += temp_result2;
    }

    println!("{}", Instant::now().duration_since(time).as_secs_f64());
    println!("{}", result1);
    println!("{}", result2);
}