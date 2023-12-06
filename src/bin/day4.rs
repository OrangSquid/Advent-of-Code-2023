use advent::readlines::read_lines;

fn main() {
    let buffer = read_lines("input/day4.txt");
    let mut final_score = 0;

    let mut copies = vec![1; buffer.len()];
    let mut it = buffer.iter().enumerate();

    while let Some((index, game)) = it.next() {
        let ignore_start = game.split(": ").collect::<Vec<&str>>()[1];
        let all_numbers = ignore_start.split(" | ").collect::<Vec<&str>>();
        let winning_numbers = all_numbers[0].split_whitespace().map(|number| number.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        let mut bet_numbers = all_numbers[1].split_whitespace().map(|number| number.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        bet_numbers.sort();

        let mut score = 0;
        let mut to_copy = index + 1;
        for number in &winning_numbers {
            if bet_numbers.binary_search(number).is_ok() {
                if score == 0 {
                    score += 1
                } else {
                    score *= 2;
                }
                if to_copy < copies.len() {
                    copies[to_copy] += 1 * copies[index];
                    to_copy += 1
                }
            }
        }
        final_score += score;
    }

    println!("{}", final_score);
    println!("{}", copies.iter().sum::<i32>());
}