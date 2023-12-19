use std::{collections::HashMap, ops::Range, time::Instant};

use advent::readlines::read_lines;

#[derive(Clone)]
enum Transition {
    Accept,
    Reject,
    Name(String),
}

enum Rule {
    None,
    LessThan(char, u64),
    HigherThan(char, u64),
}

struct Rating {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

#[derive(Clone)]
struct RatingRanged {
    x: Range<u64>,
    m: Range<u64>,
    a: Range<u64>,
    s: Range<u64>,
}

impl RatingRanged {
    pub fn split_less_at(&mut self, parameter: char, value: u64) -> RatingRanged {
        let mut rating2 = self.clone();
        match parameter {
            'x' => {
                rating2.x.end = value;
                self.x.start = value;
            }
            'm' => {
                rating2.m.end = value;
                self.m.start = value;
            }
            'a' => {
                rating2.a.end = value;
                self.a.start = value;
            }
            's' => {
                rating2.s.end = value;
                self.s.start = value;
            }
            _ => panic!(),
        };
        rating2
    }

    pub fn split_higher_at(&mut self, parameter: char, value: u64) -> RatingRanged {
        let mut rating2 = self.clone();
        match parameter {
            'x' => {
                self.x.end = value + 1;
                rating2.x.start = value + 1;
            }
            'm' => {
                self.m.end = value + 1;
                rating2.m.start = value + 1;
            }
            'a' => {
                self.a.end = value + 1;
                rating2.a.start = value + 1;
            }
            's' => {
                self.s.end = value + 1;
                rating2.s.start = value + 1;
            }
            _ => panic!(),
        };
        rating2
    }

    pub fn sum(&self) -> u64 {
        (self.x.end - self.x.start) * (self.m.end - self.m.start) * (self.a.end - self.a.start) * (self.s.end - self.s.start)
    }
}

fn split_ratings<'a>(
    mut rating: RatingRanged,
    workflow: &str,
    transitions: &HashMap<&str, Vec<(Rule, Transition)>>,
) -> Vec<(RatingRanged, Transition)> {
    let mut the_split = Vec::new();
    for (rule, transition) in transitions.get(workflow).unwrap() {
        match rule {
            Rule::None => the_split.push((rating.clone(), transition.clone())),
            Rule::LessThan(parameter, value) => match *parameter {
                'x' => {
                    if rating.x.start < *value {
                        the_split.push((rating.split_less_at('x', *value), transition.clone()));
                    }
                }
                'm' => {
                    if rating.m.start < *value {
                        the_split.push((rating.split_less_at('m', *value), transition.clone()));
                    }
                }
                'a' => {
                    if rating.a.start < *value {
                        the_split.push((rating.split_less_at('a', *value), transition.clone()));
                    }
                }
                's' => {
                    if rating.s.start < *value {
                        the_split.push((rating.split_less_at('s', *value), transition.clone()));
                    }
                }
                _ => panic!(),
            },
            Rule::HigherThan(parameter, value) => match *parameter {
                'x' => {
                    if rating.x.end > *value {
                        the_split.push((rating.split_higher_at('x', *value), transition.clone()));
                    }
                }
                'm' => {
                    if rating.m.end > *value {
                        the_split.push((rating.split_higher_at('m', *value), transition.clone()));
                    }
                }
                'a' => {
                    if rating.a.end > *value {
                        the_split.push((rating.split_higher_at('a', *value), transition.clone()));
                    }
                }
                's' => {
                    if rating.s.end > *value {
                        the_split.push((rating.split_higher_at('s', *value), transition.clone()));
                    }
                }
                _ => panic!(),
            },
        }
    }
    the_split
}

fn check_validity(
    rating: &Rating,
    workflow: &str,
    transitions: &HashMap<&str, Vec<(Rule, Transition)>>,
) -> bool {
    for (rule, transition) in transitions.get(workflow).unwrap() {
        match rule {
            Rule::None => (),
            Rule::LessThan(parameter, value) => match *parameter {
                'x' => {
                    if rating.x >= *value {
                        continue;
                    }
                }
                'm' => {
                    if rating.m >= *value {
                        continue;
                    }
                }
                'a' => {
                    if rating.a >= *value {
                        continue;
                    }
                }
                's' => {
                    if rating.s >= *value {
                        continue;
                    }
                }
                _ => panic!(),
            },
            Rule::HigherThan(parameter, value) => match *parameter {
                'x' => {
                    if rating.x <= *value {
                        continue;
                    }
                }
                'm' => {
                    if rating.m <= *value {
                        continue;
                    }
                }
                'a' => {
                    if rating.a <= *value {
                        continue;
                    }
                }
                's' => {
                    if rating.s <= *value {
                        continue;
                    }
                }
                _ => panic!(),
            },
        }
        return match transition {
            Transition::Accept => true,
            Transition::Reject => false,
            Transition::Name(new_workflow) => check_validity(rating, &new_workflow, transitions),
        };
    }
    false
}

fn main() {
    let time = Instant::now();
    let buffer: Vec<String> = read_lines("input/day19.txt");
    let mut transitions: HashMap<&str, Vec<(Rule, Transition)>> = HashMap::new();
    let mut accepted = Vec::new();
    let mut reading_transistions = true;
    let mut result = 4000 * 4000 * 4000 * 4000;

    for line in buffer.iter() {
        if line == "" {
            reading_transistions = false;
            continue;
        }
        if reading_transistions {
            let mut parsed_rules = Vec::new();
            let line = line.strip_suffix('}').unwrap();
            let mut it = line.split('{');
            let name = it.next().unwrap();
            let rules_it = it.next().unwrap().split(',');
            for rule in rules_it {
                let mut rule_it = rule.split(':');
                let part1 = rule_it.next().unwrap();
                match rule_it.next() {
                    Some(part2) => {
                        let parsed_rule = if part1.contains('>') {
                            let mut part1_it = part1.split('>');
                            let parameter = part1_it.next().unwrap().chars().next().unwrap();
                            let value = part1_it.next().unwrap();
                            Rule::HigherThan(parameter, value.parse().unwrap())
                        } else if part1.contains('<') {
                            let mut part1_it = part1.split('<');
                            let parameter = part1_it.next().unwrap().chars().next().unwrap();
                            let value = part1_it.next().unwrap();
                            Rule::LessThan(parameter, value.parse().unwrap())
                        } else {
                            Rule::None
                        };
                        let transition = match part2 {
                            "R" => Transition::Reject,
                            "A" => Transition::Accept,
                            _ => Transition::Name(part2.to_owned()),
                        };
                        parsed_rules.push((parsed_rule, transition));
                    }
                    None => {
                        match part1 {
                            "R" => parsed_rules.push((Rule::None, Transition::Reject)),
                            "A" => parsed_rules.push((Rule::None, Transition::Accept)),
                            _ => {
                                parsed_rules.push((Rule::None, Transition::Name(part1.to_owned())))
                            }
                        };
                    }
                }
            }
            transitions.insert(name, parsed_rules);
        } else {
            /* let line = line.strip_prefix('{')
                .unwrap()
                .strip_suffix('}')
                .unwrap();
            let mut rating = Rating { x: 0, m: 0, a: 0, s: 0 };
            for parameter in line.split(',') {
                let mut it_parameter = parameter.split('=');
                let name = it_parameter.next().unwrap();
                let value = it_parameter.next().unwrap().parse().unwrap();
                match name {
                    "x" => rating.x = value,
                    "m" => rating.m = value,
                    "a" => rating.a = value,
                    "s" => rating.s = value,
                    _ => panic!()
                }
            }
            if check_validity(&rating, "in", &transitions) {
                accepted.push(rating);
            } */

            let mut queue = Vec::new();
            queue.push((
                RatingRanged {
                    x: 1..4001,
                    m: 1..4001,
                    a: 1..4001,
                    s: 1..4001,
                },
                Transition::Name("in".to_owned()),
            ));
            while !queue.is_empty() {
                let (rating, transition) = queue.pop().unwrap();
                match transition {
                    Transition::Accept => accepted.push(rating),
                    Transition::Reject => result -= rating.sum(),
                    Transition::Name(workflow) => {
                        let mut binding = split_ratings(rating, &workflow, &transitions);
                        queue.append(&mut binding);
                    }
                }
            }
            break;
        }
    }

    println!("{}", Instant::now().duration_since(time).as_secs_f64());
    println!("{}", result);
}
