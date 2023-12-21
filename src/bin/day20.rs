use std::{time::Instant, collections::{HashMap, VecDeque}, cell::RefCell, rc::Rc, ops::Deref, vec, borrow::Borrow, any::Any};

use advent::readlines::read_lines;
use num::{Num, integer::lcm};

#[derive(Clone, Copy, PartialEq, Eq)]
enum ElectricalPulse {
    Low,
    High
}

trait Electrical {
    fn push_input(&mut self, pulse: ElectricalPulse, name: &str);
    fn process_output(&mut self) -> Vec<(&str, ElectricalPulse)>;
    fn set_number_inputs(&mut self, inputs: Vec<&str>);
    fn as_any(&self) -> &dyn Any;
}

#[derive(PartialEq, Eq)]
struct ElectricalIO {
    output: Vec<String>,
}

#[derive(PartialEq, Eq)]

struct FlipFlop {
    io: ElectricalIO,
    queue: VecDeque<ElectricalPulse>,
    on: bool
}

#[derive(PartialEq, Eq)]

struct Conjunction {
    io: ElectricalIO,
    queue: VecDeque<(String, ElectricalPulse)>,
    last_remembered: HashMap<String, ElectricalPulse>
}

#[derive(PartialEq, Eq)]
struct Broadcaster {
    io: ElectricalIO
}

impl Electrical for FlipFlop {
    fn push_input(&mut self, pulse: ElectricalPulse, _name: &str) {
        self.queue.push_back(pulse);
    }

    fn process_output(&mut self) -> Vec<(&str, ElectricalPulse)> {
        let mut changed = false;
        match self.queue.pop_front().unwrap() {
            ElectricalPulse::High => (),
            ElectricalPulse::Low => {
                self.on = !self.on;
                changed = true;
            } 
        }
        if changed {
            let pulse_to_send = 
            match self.on {
                true => ElectricalPulse::High,
                false => ElectricalPulse::Low,
            };
            self.io.output.iter().map(|out| (out.as_str(), pulse_to_send)).collect()
        } else {  
            Vec::new()
        }
    }

    fn set_number_inputs(&mut self, _inputs: Vec<&str>) { }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Conjunction {
    pub fn new(output: Vec<String>) -> Conjunction {
        let io = ElectricalIO { output };
        Conjunction { io, queue: VecDeque::new(), last_remembered: HashMap::new() }
    }
}

impl Electrical for Conjunction {
    fn push_input(&mut self, pulse: ElectricalPulse, name: &str) {
        self.queue.push_back((name.to_owned(), pulse));
    }

    fn process_output(&mut self) -> Vec<(&str, ElectricalPulse)> {
        let (name, pulse) = self.queue.pop_front().unwrap();
        let pulse_question_mark = self.last_remembered.get_mut::<str>(name.borrow()).unwrap();
        *pulse_question_mark = pulse;
        let all_high = self.last_remembered.iter().all(|(_, pulse)| *pulse == ElectricalPulse::High);
        let pulse_to_send = 
        match all_high {
            true => ElectricalPulse::Low,
            false => ElectricalPulse::High,
        };
        self.io.output.iter().map(|out| (out.as_str(), pulse_to_send)).collect()
    }

    fn set_number_inputs(&mut self, inputs: Vec<&str>) {
        for input in inputs {
            self.last_remembered.insert(input.to_owned(), ElectricalPulse::Low);
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Electrical for Broadcaster {
    fn push_input(&mut self, _pulse: ElectricalPulse, _name: &str) { }

    fn process_output(&mut self) -> Vec<(&str, ElectricalPulse)> {
        self.io.output.iter().map(|out| (out.as_str(), ElectricalPulse::Low)).collect()
    }

    fn set_number_inputs(&mut self, _inputs: Vec<&str>) { }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl FlipFlop {
    pub fn new(output: Vec<String>) -> FlipFlop {
        let io = ElectricalIO { output };
        FlipFlop { io, queue: VecDeque::new(), on: false }
    }
}

impl Broadcaster {
    pub fn new(output: Vec<String>) -> Broadcaster {
        let io = ElectricalIO { output };
        Broadcaster { io }
    }
}

struct Stub {}

impl Electrical for Stub {
    fn push_input(&mut self, _pulse: ElectricalPulse, _name: &str) { }

    fn process_output(&mut self) -> Vec<(&str, ElectricalPulse)> { Vec::new() }

    fn set_number_inputs(&mut self, _inputs: Vec<&str>) { }

    fn as_any(&self) -> &dyn Any { 
        self
    }
}

fn main() {
    let time = Instant::now();
    let buffer: Vec<String> = read_lines("input/day20.txt");
    let mut queue: VecDeque<(String, Rc<RefCell<dyn Electrical>>)> = VecDeque::new();
    let mut electrical_map: HashMap<&str, Rc<RefCell<dyn Electrical>>> = HashMap::new();
    let mut map_vec_inputs: HashMap<String, Vec<&str>> = HashMap::new();
    let mut low_pulses = 0;
    let mut high_pulses = 0;
    let mut button_pushes = 0;

    for line in buffer.iter() {
        let mut it = line.split(" -> ");
        let mut name = it.next().unwrap();
        let output: Vec<String> = it.next().unwrap().split(", ").map(|x| x.to_owned()).collect();
        
        if name.starts_with('%') {
            name = name.strip_prefix('%').unwrap();
            for out in output.iter() {
                match map_vec_inputs.get_mut(out) {
                    Some(vec_input) => vec_input.push(name),
                    None => {map_vec_inputs.insert(out.to_owned(), vec![name]);},
                }
            }
            let flip_flop = FlipFlop::new(output);
            electrical_map.insert(name, Rc::new(RefCell::new(flip_flop)));
        } else if name.starts_with('&') {
            name = name.strip_prefix('&').unwrap();
            for out in output.iter() {
                match map_vec_inputs.get_mut(out) {
                    Some(vec_input) => vec_input.push(name),
                    None => {map_vec_inputs.insert(out.to_owned(), vec![name]);},
                }
            }
            let conjunction = Conjunction::new(output);
            electrical_map.insert(name, Rc::new(RefCell::new(conjunction)));
        } else {
            for out in output.iter() {
                match map_vec_inputs.get_mut(out) {
                    Some(vec_input) => vec_input.push(name),
                    None => {map_vec_inputs.insert(out.to_owned(), vec![name]);},
                }
            }
            let broadcaster = Rc::new(RefCell::new(Broadcaster::new(output)));
            electrical_map.insert(name, broadcaster.clone());
            queue.push_back(("broadcaster".to_string(), broadcaster));
        }
    }

    for (component_name, input_number) in map_vec_inputs.iter() {
        let component = electrical_map.get(component_name.as_str());
        match component {
            Some(inside) => {
                inside.deref().borrow_mut().set_number_inputs(input_number.clone());
            },
            None => {
                electrical_map.insert(component_name.as_str(), Rc::new(RefCell::new(Stub { })));
            },
        }
    }

    // Devido à natureza deste dia (binary counters yeeey), esta solução é exclusiva do meu input
    let mut check_repeat: Vec<u64> = vec![0; 4];

    let loop1: Rc<RefCell<dyn Electrical>> = electrical_map.get("fb").unwrap().clone();
    let loop2: Rc<RefCell<dyn Electrical>> = electrical_map.get("gp").unwrap().clone();
    let loop3: Rc<RefCell<dyn Electrical>> = electrical_map.get("jl").unwrap().clone();
    let loop4: Rc<RefCell<dyn Electrical>> = electrical_map.get("jn").unwrap().clone();

    let loops = [("fb", loop1.clone()), ("gp", loop2.clone()), ("jl", loop3.clone()), ("jn", loop4.clone())];

    for (index, (name, loop_)) in loops.iter().enumerate() {
        let borrow = loop_.deref().borrow();
        let starting_conjunction: &Conjunction = borrow.as_any().downcast_ref::<Conjunction>().unwrap();
        let mut button_presses = 0b1;
        let starting_flip_flop_name = starting_conjunction.io.output.iter().find(|out| electrical_map.get("broadcaster").unwrap().deref().borrow().as_any().downcast_ref::<Broadcaster>().unwrap().io.output.contains(*out)).unwrap();
        let mut borrow = electrical_map.get(starting_flip_flop_name.as_str()).unwrap().deref().borrow();
        let mut flip_flop = borrow.as_any().downcast_ref::<FlipFlop>().unwrap();
        let mut counter = 0;
        loop {
            if flip_flop.io.output.len() == 1 && &flip_flop.io.output[0] == name {
                button_presses |= 1 << counter;
                break;
            }
            let mut next_flip_flop = "";
            let mut increase_presses = false;
            let mut out_electrical;
            for out in flip_flop.io.output.iter() {
                out_electrical = electrical_map.get(out.as_str()).unwrap().deref().borrow();
                match out_electrical.as_any().is::<FlipFlop>() {
                    true => next_flip_flop = out,
                    false => increase_presses = true,
                }
            }
            if next_flip_flop == "" {
                panic!()
            } else {
                borrow = electrical_map.get(next_flip_flop).unwrap().deref().borrow();
                flip_flop = borrow.as_any().downcast_ref::<FlipFlop>().unwrap();
            }
            if increase_presses {
                button_presses |= 1 << counter;
            }
            counter += 1;
        }
        check_repeat[index] = button_presses;
    }

    println!("{}", Instant::now().duration_since(time).as_secs_f64());
    println!("{}", lcm(lcm(check_repeat[0], check_repeat[1]), lcm(check_repeat[2], check_repeat[3])));
}
