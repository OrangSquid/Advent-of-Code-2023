use std::time::Instant;

use advent::readlines::read_lines;

fn hash(sequence: &str) -> u64 {
    sequence
        .chars()
        .fold(0, |acc, i| (acc + i as u64) * 17 % 256)
}

#[derive(Clone, Debug)]
struct Lens<'a> {
    name: &'a str,
    focus: u64,
}

fn main() {
    let time = Instant::now();
    let binding = read_lines("input/day15.txt").into_iter().next().unwrap();
    let buffer: Vec<&str> = binding.split(',').collect();
    let result = buffer.iter().fold(0, |acc, sequence| acc + hash(sequence));

    let mut boxes: Vec<Vec<Lens>> = vec![Vec::new(); 256];

    for lens in buffer {
        if lens.contains('=') {
            let mut it = lens.split('=');
            let lens_name = it.next().unwrap();
            let lens_hash = hash(lens_name);
            let lens_focus = it.next().unwrap().parse().unwrap();
            match boxes[lens_hash as usize]
                .iter_mut()
                .find(|lens| lens.name == lens_name)
            {
                Some(lens) => (*lens).focus = lens_focus,
                None => boxes[lens_hash as usize].push(Lens {
                    name: lens_name,
                    focus: lens_focus,
                }),
            }
        } else if lens.contains('-') {
            let lens_name = lens.trim_end_matches('-');
            let lens_hash = hash(lens_name);
            match boxes[lens_hash as usize]
                .iter()
                .enumerate()
                .find(|(_, lens)| lens.name == lens_name)
            {
                Some((index, _)) => {
                    boxes[lens_hash as usize].remove(index);
                }
                None => (),
            }
        }
    }

    let result2 = boxes.iter().enumerate().fold(0, |acc, (index1, vec)| {
        acc + vec.iter().enumerate().fold(0, |acc, (index2, lens)| acc + (index2 as u64 + 1) * (index1 as u64 + 1) * lens.focus)
    });

    println!("{}", Instant::now().duration_since(time).as_secs_f64());
    println!("{}", result);
    println!("{}", result2);
}
