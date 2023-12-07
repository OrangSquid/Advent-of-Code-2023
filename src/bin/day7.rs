use std::{time::Instant, collections::{HashMap, BTreeSet}};

use advent::readlines::read_lines;

fn priority(label: char) -> u32 {
    match label {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 1,
        'T' => 10,
        _ => label.to_digit(10).unwrap()
    }
}

#[derive(Debug)]
struct HandBid {
    hand: String,
    bid: u32
}

impl HandBid {
    pub fn count_labels(&self) -> HashMap<char, u32>{
        let mut temp = HashMap::new(); 
        for label in self.hand.chars() {
            if temp.contains_key(&label) {
                *temp.get_mut(&label).unwrap() += 1;
            } else {
                temp.insert(label, 1);
            }
        }
        if temp.contains_key(&'J') && temp.keys().count() > 1 {
            let j = temp.remove(&'J').unwrap();
            let i = temp.values_mut().max().unwrap();
            *i += j;
        }
        temp
    }
}

impl PartialEq for HandBid {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}

impl Eq for HandBid {
    
}

impl PartialOrd for HandBid {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandBid {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self == other {
            return std::cmp::Ordering::Equal;
        }
        let labels_self = self.count_labels();
        let labels_other = other.count_labels();
        let mut values_self = labels_self.values().map(|i| i.to_owned()).collect::<Vec<u32>>();
        let mut values_other = labels_other.values().map(|i| i.to_owned()).collect::<Vec<u32>>();
        values_self.sort_by(|a, b| b.cmp(a));
        values_other.sort_by(|a, b| b.cmp(a));
        for i in 0..5 {
            if values_self.len() <= i || values_other.len() <= i {
                break;
            } else if values_self[i] < values_other[i] {
                return std::cmp::Ordering::Less;
            } else if values_self[i] > values_other[i] {
                return std::cmp::Ordering::Greater;
            }
        }
        for i in 0..5 {
            let priority_self = priority(self.hand.chars().nth(i).unwrap());
            let priority_other = priority(other.hand.chars().nth(i).unwrap());
            if priority_self == priority_other {
                continue;
            }
            return priority_self.cmp(&priority_other);
        }
        std::cmp::Ordering::Equal
    }
}

fn main() {
    let time = Instant::now();
    let buffer = read_lines("input/day7.txt");
    let mut hands: BTreeSet<HandBid> = BTreeSet::new();

    for mut line in buffer.iter().map(|line| line.split_whitespace()) {
        let lmao = HandBid {
            hand: line.next().unwrap().to_string(),
            bid: line.next().unwrap().parse().unwrap()
        };
        hands.insert(lmao);
    }

    println!("{}", Instant::now().duration_since(time).as_secs_f64());
    
    let mut result = 0;
    for (index, hand) in hands.iter().enumerate() {
        result += hand.bid * (index + 1) as u32;
    }
    println!("{:#?}", hands);
    println!("{}", result);
}