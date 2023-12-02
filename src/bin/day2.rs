use std::cmp::{Ordering, max};

use advent::readlines::read_lines;

#[derive(PartialEq, Debug, Clone)]
struct Set {
    red: u32,
    green: u32,
    blue: u32
}

impl Set {
    pub fn new(cubes: &str) -> Set {
        let mut set = Set { red: 0, green: 0, blue: 0};
        for cube in cubes.split(", ") {
            let i: Vec<&str> = cube.split(" ").collect();
            match i[1] {
                "red" => set.red = i[0].parse::<u32>().unwrap(),
                "green" => set.green = i[0].parse::<u32>().unwrap(),
                "blue" => set.blue= i[0].parse::<u32>().unwrap(),
                _ => ()
            }
        }
        set
    }
}

impl PartialOrd for Set {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        max(self.blue.partial_cmp(&other.blue), 
        max(self.red.partial_cmp(&other.red), 
            self.green.partial_cmp(&other.green)))
    }
}

const COMPARISION_SET: Set = Set { red: 12, green: 13, blue: 14 };

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<Set>,
    minimum_set: Option<Set>,
    is_possible: bool
}

impl Game {
    pub fn new(id: u32) -> Game {
        Game { id, sets: Vec::new(), minimum_set: None, is_possible: true }
    }

    pub fn add_set(&mut self, set: Set) {
        self.is_possible = set <= COMPARISION_SET;
        match &mut self.minimum_set {
            None => self.minimum_set = Some(set.clone()),
            Some(other) => {
                other.red = max(other.red, set.red);
                other.green = max(other.green, set.green);
                other.blue = max(other.blue, set.blue);
            }
        }
        self.sets.push(set);
    }

    pub fn power_set(&self) -> u32 {
        match &self.minimum_set {
            None => 0,
            Some(other) => other.red * other.green * other.blue
        }
    }
}

fn main() {
    let buffer = read_lines("input/day2.txt");
    let mut games: Vec<Game> = Vec::new();

    for line in buffer {
        let splited = line.split(": ").collect::<Vec<&str>>();
        let mut game = Game::new(splited[0].split(" ").collect::<Vec<&str>>()[1].parse::<u32>().unwrap());

        for cubes in splited[1].split("; ") {
            game.add_set(Set::new(cubes));
        }
        games.push(game);
    }
    println!("{}", games.iter().filter(|game| game.is_possible).map(|game| game.id).sum::<u32>());
    println!("{}", games.iter().map(|game| game.power_set()).sum::<u32>());
}