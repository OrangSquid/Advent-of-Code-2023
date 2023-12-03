use std::collections::HashMap;

use advent::readlines::read_lines;

fn check_neighbours(x: i32, y: i32, buffers: &[Vec<char>]) -> bool {
    let max_x = buffers.len() as i32;
    let max_y = buffers[0].len() as i32;

    (-1..=1).flat_map(|dx| (-1..=1).map(move |dy| (dx, dy)))
        .any(|(dx, dy)| {
            let nx = x + dx;
            let ny = y + dy;
            if dx == 0 && dy == 0 || nx <= 0 || ny <= 0 || nx >= max_x - 1 || ny >= max_y - 1 {
                false
            } else {
                let char = buffers[nx as usize][ny as usize];
                char.is_ascii_punctuation() && char != '.'
            }
        })
}

fn check_gears(x: i32, y: i32, buffers: &[Vec<char>]) -> (i32, i32) {
    let max_x = buffers.len() as i32;
    let max_y = buffers[0].len() as i32;

    for dx in -1..=1 {
        for dy in -1..=1 {
            let nx = x + dx;
            let ny = y + dy;
            if dx == 0 && dy == 0 || nx <= 0 || ny <= 0 || nx >= max_x - 1 || ny >= max_y - 1 {
                continue;
            } else if buffers[nx as usize][ny as usize] == '*' {
                return (nx, ny);
            }
        }
    }
    (-1, -1)
} 

struct Gear {
    pub vec: Vec<u32>
}

impl Gear {
    pub fn check_gear(&self) -> u32 {
        if self.vec.len() != 2 {
            0
        } else {
            self.vec[0] * self.vec[1]
        }
    }

    pub fn add_to_gear(&mut self, lmao: u32) {
        self.vec.push(lmao);
    }
}

fn main() {
    let binding = read_lines("input/day3.txt");
    let buffers = binding.iter().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let mut gears: HashMap<(i32, i32), Gear> = HashMap::new();
    let mut sum = 0;
    let mut it = buffers.iter().enumerate();

    while let Some((i, character_vec)) = it.next() {
        let mut valid_number = false;
        let mut number_string = String::new();
        let mut gear_pos = (-1, -1);
        let mut it2 = character_vec.iter().enumerate().peekable();

        while let Some((j, character)) = it2.next() {
            if character.is_digit(10) {
                number_string.push(*character);
                valid_number = valid_number || check_neighbours(i as i32, j as i32, &buffers);
                if gear_pos == (-1, -1) {
                    gear_pos = check_gears(i as i32, j as i32, &buffers);
                }
                if it2.peek().is_none() && valid_number {
                    sum += number_string.parse::<u32>().unwrap();
                    println!("{}", number_string.parse::<u32>().unwrap());
                    valid_number = false;
                    if gears.contains_key(&gear_pos) && gear_pos != (-1, -1) {
                        gears.get_mut(&gear_pos).unwrap().add_to_gear(number_string.parse::<u32>().unwrap());
                    } else if gear_pos != (-1, -1) {
                        gears.insert(gear_pos, Gear { vec: vec![number_string.parse::<u32>().unwrap()] });
                    }
                    gear_pos = (-1, -1);
                }
            }
            else if !character.is_digit(10) {
                if valid_number {
                    sum += number_string.parse::<u32>().unwrap();
                    println!("{}", number_string.parse::<u32>().unwrap());
                    valid_number = false;
                    if gears.contains_key(&gear_pos) && gear_pos != (-1, -1) {
                        gears.get_mut(&gear_pos).unwrap().add_to_gear(number_string.parse::<u32>().unwrap());
                    } else if gear_pos != (-1, -1) {
                        gears.insert(gear_pos, Gear { vec: vec![number_string.parse::<u32>().unwrap()] });
                    }
                    gear_pos = (-1, -1);
                }
                number_string.clear();
            }
        }
    }
    println!("{}", sum);
    println!("{}", gears.values().map(|gear| gear.check_gear()).sum::<u32>())
}