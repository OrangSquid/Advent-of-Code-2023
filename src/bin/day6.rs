use std::time::Instant;

use advent::readlines::read_lines;

fn main() {
    let time = Instant::now();
    let buffer = read_lines("input/day6.txt");

    let mut times_str = buffer[0].split_whitespace();
    let mut distances_str = buffer[1].split_whitespace();

    let mut times = Vec::new();
    times_str.next();
    for time in times_str {
        times.push(time.parse::<u64>().unwrap());
    }

    let mut distances = Vec::new();
    distances_str.next();
    for distance in distances_str {
        distances.push(distance.parse::<u64>().unwrap());
    }

    let mut final_result = 1;
    for (index, time) in times.iter().enumerate() {
        let mut ways_of_beating = 0;
        for i in 0..*time {
            let boating_time = time - i;
            let distance = i * boating_time;
            if distance > distances[index] {
                ways_of_beating += 1;
            }
        }
        final_result *= ways_of_beating;
    }    

    println!("{}", Instant::now().duration_since(time).as_secs_f64());
    println!("{}", final_result);
}