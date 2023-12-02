use advent::readlines::read_lines;

fn spelled_to_number_string(spelled: &[char]) -> &str {
    let collected: String = spelled.iter().collect();
    if collected.starts_with("one") {
        "1"
    } else if collected.starts_with("two") {
        "2"
    } else if collected.starts_with("three") {
        "3"
    } else if collected.starts_with("four") {
        "4"
    } else if collected.starts_with("five") {
        "5"
    } else if collected.starts_with("six") {
        "6"
    } else if collected.starts_with("seven") {
        "7"
    } else if collected.starts_with("eight") {
        "8"
    } else if collected.starts_with("nine") {
        "9"
    } else {
        ""
    }
}

fn spelled_to_number_string_rev(spelled: &[char]) -> &str {
    let collected: String = spelled.iter().collect();
    if collected.ends_with("one") {
        "1"
    } else if collected.ends_with("two") {
        "2"
    } else if collected.ends_with("three") {
        "3"
    } else if collected.ends_with("four") {
        "4"
    } else if collected.ends_with("five") {
        "5"
    } else if collected.ends_with("six") {
        "6"
    } else if collected.ends_with("seven") {
        "7"
    } else if collected.ends_with("eight") {
        "8"
    } else if collected.ends_with("nine") {
        "9"
    } else {
        ""
    }
}

fn main() {
    let buffer = read_lines("input/day1.txt");
    let mut final_result = 0;
    let mut i = 0;
    for line in buffer {
        let mut result = String::new();
        let line_chars = line.chars().collect::<Vec<char>>();
        let window_size = 
        if line.len() < 5 {
            line.len()
        } else { 5 };
        let mut it = line_chars.windows(window_size).peekable();
        while let Some(characters) = it.next() {
            if characters[0].is_digit(10) {
                result.push_str(&characters[0].to_string());
                break;
            }
            let spelled = spelled_to_number_string(characters);
            if spelled != "" {
                result.push_str(spelled);
                break;
            }
            if it.peek().is_none() {
                for i in (characters.len()-window_size+1)..window_size {
                    let characters_short = &characters[i..window_size];
                    if characters_short[0].is_digit(10) {
                        result.push_str(&characters_short[0].to_string());
                        break;
                    }
                    let spelled = spelled_to_number_string(characters_short);
                    if spelled != "" {
                        result.push_str(spelled);
                        break;
                    }
                }
            }
        }
        let mut it = line_chars.windows(window_size).rev().peekable();
        while let Some(characters) = it.next() {
            if characters[window_size - 1].is_digit(10) {
                result.push_str(&characters[window_size - 1].to_string());
                break;
            }
            let spelled = spelled_to_number_string_rev(characters);
            if spelled != "" {
                result.push_str(spelled);
                break;
            }
            if it.peek().is_none() {
                for i in (1..window_size).rev() {
                    let characters_short = &characters[0..i];
                    if characters_short[i-1].is_digit(10) {
                        result.push_str(&characters_short[i-1].to_string());
                        break;
                    }
                    let spelled = spelled_to_number_string_rev(characters_short);
                    if spelled != "" {
                        result.push_str(spelled);
                        break;
                    }
                }
            }
        }
        i += 1;
        println!("{}: {}", i, result);
        final_result += result.parse::<i32>().unwrap();
    }
    println!("{}", final_result)
}