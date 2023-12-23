use std::{time::Instant, cell::RefCell, rc::Rc, collections::HashMap};

use advent::readlines::read_lines;
use bitmaps::Bitmap;

fn find_path(x: usize, y: usize, cost: u64, buffer: &Vec<Vec<char>>, dest_x: usize, dest_y: usize) -> u64 {
    const DIRECTIONS: [(isize, isize, char); 4] = [(0, -1, '>'), (-1, 0, 'v'), (1, 0, ' '), (0, 1, ' ')];
    if x == dest_x && y == dest_y {
        return cost;
    }
    let visited_nodes: Rc<RefCell<Vec<Bitmap<141>>>> = Rc::new(RefCell::new(vec![Bitmap::new(); buffer.len()]));
    let mut highest_cost = 0;
    let mut queue = vec![(x, y, cost, visited_nodes)];
    while let Some((x, y, cost, visited_nodes)) = queue.pop() {
        let mut diverging = false;
        for (direction_x, direction_y, slope) in DIRECTIONS {
            let new_x = x as isize + direction_x;
            let new_y = y as isize + direction_y;
            let cost = num::abs(direction_x) as u64 + num::abs(direction_y) as u64 + cost;
            if new_x < 0
                || new_x >= buffer.len() as isize
                || new_y < 0
                || new_y >= buffer[0].len() as isize
            {
                continue;
            }
            
            let new_x = new_x as usize;
            let new_y = new_y as usize;
            if new_x == dest_x && new_y == dest_y {
                highest_cost = highest_cost.max(cost);
            }
            let position = buffer[new_x][new_y];
            if position == '#' {
                continue;
            }
            if visited_nodes.borrow()[new_x].get(new_y) {
                continue;
            }
            let new_visited_nodes = 
            if diverging {
                Rc::new(RefCell::new(visited_nodes.borrow().clone()))
            } else {
                visited_nodes.clone()
            };
            diverging = true;
            new_visited_nodes.borrow_mut()[x].set(y, true);
            queue.push((new_x, new_y, cost, new_visited_nodes));
        }
    }
    highest_cost
}

fn main() {
    let time = Instant::now();
    let buffer: Vec<Vec<char>> = read_lines("input/day23.txt").into_iter().map(|line| line.chars().collect()).collect();
    let dest_x = buffer.len() - 1;
    let dest_y = buffer[0].len() - 2;

    let result = find_path(0, 1, 0, &buffer, dest_x, dest_y);
    println!("{}", Instant::now().duration_since(time).as_secs_f64());
    println!("{}", result);
}
