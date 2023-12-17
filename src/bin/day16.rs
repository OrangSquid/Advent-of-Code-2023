use std::{time::Instant, collections::HashSet};

use advent::readlines::read_lines;

#[derive(Eq, Hash, PartialEq, PartialOrd, Clone)]
enum Heading {
    North,
    South,
    East,
    West
}

#[derive(Eq, Hash, PartialEq, PartialOrd)]
struct VisitedNode {
    x: isize,
    y: isize,
    heading: Heading
}

fn beam(grid: &[Vec<char>], x: isize, y: isize, heading: Heading, energized: &mut [Vec<char>], visited: &mut HashSet<VisitedNode>) {
    if x < 0 || x >= grid.len() as isize || y < 0 || y >= grid[0].len() as isize {
        ()
    } else {
        energized[x as usize][y as usize] = '#';
        let new_cell = grid[x as usize][y as usize];
        if !visited.insert(VisitedNode { x, y, heading: heading.clone() }) {
            ()
        } else {
            match new_cell {
                '.' => {
                    let new_x = match heading {
                        Heading::North => x - 1,
                        Heading::South => x + 1,
                        _ => x
                    };
                    let new_y = match heading {
                        Heading::West => y - 1,
                        Heading::East => y + 1,
                        _ => y
                    };
                    beam(grid, new_x, new_y, heading, energized, visited);
                },
                '|' => match heading {
                    Heading::East | Heading::West => { 
                        beam(grid, x - 1, y, Heading::North, energized, visited); 
                        beam(grid, x + 1, y, Heading::South, energized, visited);
                    }
                    Heading::North => beam(grid, x - 1, y, heading, energized, visited),
                    Heading::South => beam(grid, x + 1, y, heading, energized, visited),
                },
                '-' => match heading {
                    Heading::East => beam(grid, x, y + 1, heading, energized, visited),
                    Heading::West => beam(grid, x, y - 1, heading, energized, visited),
                    Heading::North | Heading::South => {
                        beam(grid, x, y + 1, Heading::East, energized, visited);
                        beam(grid, x, y - 1, Heading::West, energized, visited);
                    }
                },
                '\\' => match heading {
                    Heading::East => beam(grid, x + 1, y, Heading::South, energized, visited),
                    Heading::South => beam(grid, x, y + 1, Heading::East, energized, visited),
                    Heading::West => beam(grid, x - 1, y, Heading::North, energized, visited),
                    Heading::North => beam(grid, x, y - 1, Heading::West, energized, visited),
                },
                '/' => match heading {
                    Heading::East => beam(grid, x - 1, y, Heading::North, energized, visited),
                    Heading::North => beam(grid, x, y + 1, Heading::East, energized, visited),
                    Heading::West => beam(grid, x + 1, y, Heading::South, energized, visited),
                    Heading::South => beam(grid, x, y - 1, Heading::West, energized, visited),
                },
                _ => panic!()
            }
        }
    }
}

fn main() {
    let time = Instant::now();
    let buffer: Vec<Vec<char>> = read_lines("input/day16.txt").into_iter().map(|line| line.chars().collect()).collect();

    let mut energized_grid = vec![vec!['.'; buffer[0].len()]; buffer.len()];
    let mut visited = HashSet::new();
    let mut best = 0;
    for i in 0..buffer.len() {
        beam(&buffer, i as isize, 0, Heading::East, &mut energized_grid, &mut visited);
        let left = energized_grid.iter().map(|line| line.iter().filter(|cell| **cell == '#').count()).sum();
        energized_grid = vec![vec!['.'; buffer[0].len()]; buffer.len()];
        visited.clear();
        beam(&buffer, i as isize, buffer[0].len() as isize - 1, Heading::West, &mut energized_grid, &mut visited);
        let right = energized_grid.iter().map(|line| line.iter().filter(|cell| **cell == '#').count()).sum();
        energized_grid = vec![vec!['.'; buffer[0].len()]; buffer.len()];
        visited.clear();

        if right > best {
            best = right;
        } else if left > best {
            best = left;
        }
    }
    for i in 0..buffer[0].len() {
        beam(&buffer, 0, i as isize, Heading::South, &mut energized_grid, &mut visited);
        let top = energized_grid.iter().map(|line| line.iter().filter(|cell| **cell == '#').count()).sum();
        energized_grid = vec![vec!['.'; buffer[0].len()]; buffer.len()];
        visited.clear();
        beam(&buffer, buffer.len() as isize - 1, i as isize, Heading::North, &mut energized_grid, &mut visited);
        let bottom = energized_grid.iter().map(|line| line.iter().filter(|cell| **cell == '#').count()).sum();
        energized_grid = vec![vec!['.'; buffer[0].len()]; buffer.len()];
        visited.clear();

        if bottom > best {
            best = bottom;
        } else if top > best {
            best = top;
        }
    }

    println!("{}", Instant::now().duration_since(time).as_secs_f64());
    println!("{}", best);
}
