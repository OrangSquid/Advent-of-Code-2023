use std::{
    collections::BinaryHeap,
    time::Instant, vec,
    cmp
};

use advent::readlines::read_lines;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
enum Heading {
    North,
    South,
    East,
    West,
}

#[derive(Clone, Debug)]
struct Node {
    x: usize,
    y: usize,
    cost: u64,
    heading: Heading,
    min_heading_count: u64,
    max_heading_count: u64
}

impl Eq for Node {}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.heading == other.heading && self.max_heading_count == other.max_heading_count
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Node {
    pub fn new(
        x: usize,
        y: usize,
        cost: u64,
        heading: Heading,
        min_heading_count: u64,
        max_heading_count: u64
    ) -> Node {
        Node {
            x,
            y,
            cost,
            heading,
            min_heading_count,
            max_heading_count
        }
    }
}

fn main() {
    const DIRECTIONS: [(Heading, isize, isize); 4] = [
        (Heading::North, -1, 0),
        (Heading::East, 0, 1),
        (Heading::West, 0, -1),
        (Heading::South, 1, 0),
    ];
    let time = Instant::now();
    let buffer: Vec<Vec<u64>> = read_lines("input/day17.txt")
        .into_iter()
        .map(|line| {
            line.chars()
                .map(|lmao| lmao.to_string().parse().unwrap())
                .collect()
        })
        .collect();
    let mut visited_nodes: Vec<Vec<Vec<u64>>> = vec![vec![vec![u64::MAX; 44]; buffer[0].len()]; buffer.len()];
    let mut queue = BinaryHeap::with_capacity(visited_nodes.len() * visited_nodes[0].len() * visited_nodes[0][0].len());
    let dest_x = buffer.len() - 1;
    let dest_y = buffer[0].len() - 1;
    let start_node = Node::new(0, 0, 0, Heading::East, 4, 10);
    queue.push(start_node);

    loop {
        let expanding_node = queue.peek().unwrap();
        let visited_nodes_index = (expanding_node.max_heading_count << 2) as usize | expanding_node.heading as usize;
        let cost = expanding_node.cost;
        let x = expanding_node.x;
        let y = expanding_node.y;
        if cost > visited_nodes[x][y][visited_nodes_index] {
            continue;
        }
        if x == dest_x && y == dest_y {
            if expanding_node.min_heading_count == 0{
                break;
            }
            queue.pop();
            continue;
        }
        let expanding_node = queue.pop().unwrap();
        for (heading, direction_x, direction_y) in DIRECTIONS {
            if heading == expanding_node.heading && expanding_node.max_heading_count == 0 {
                continue;
            }
            if heading != expanding_node.heading && expanding_node.min_heading_count != 0 {
                continue;
            }
            match (heading, expanding_node.heading) {
                (Heading::North, Heading::South) | (Heading::South, Heading::North) | 
                (Heading::East, Heading::West) | (Heading::West, Heading::East) => continue,
                _ => ()
            };
            let max_heading_count = if heading == expanding_node.heading {
                expanding_node.max_heading_count - 1
            } else {
                9
            };
            let min_heading_count = if heading == expanding_node.heading {
                cmp::max(expanding_node.min_heading_count as isize - 1, 0) as u64
            } else {
                3
            };
            let new_x = expanding_node.x as isize + direction_x;
            let new_y = expanding_node.y as isize + direction_y;
            if new_x < 0
                || new_x >= buffer.len() as isize
                || new_y < 0
                || new_y >= buffer[0].len() as isize
            {
                continue;
            }
            let new_x = new_x as usize;
            let new_y = new_y as usize;
            let cost = buffer[new_x][new_y] + expanding_node.cost;
            let visited_nodes_index = (max_heading_count << 2) as usize | heading as usize;
            if cost < visited_nodes[new_x][new_y][visited_nodes_index] {
                let new_node = Node::new(
                    new_x,
                    new_y,
                    cost,
                    heading,
                    min_heading_count,
                    max_heading_count
                );
                queue.push(new_node);
                visited_nodes[new_x][new_y][visited_nodes_index] = cost;
            }
        }
    }

    let result = queue.peek().unwrap().cost;
    println!("{}", Instant::now().duration_since(time).as_secs_f64());
    println!("{}", result);
}
