use std::{time::Instant, collections::HashSet};

use advent::readlines::read_lines;

fn get_next_character(buffer: &[Vec<char>], x: usize, y: usize, direction: char) -> (usize, usize, char) {
    match buffer[x][y] {
        '-' => match direction {
            'w' => (x, y - 1, 'w'),
            'e' => (x, y + 1, 'e'),
            _ => panic!()
        },
        '|' => match direction {
            'n' => (x - 1, y, 'n'),
            's' => (x + 1, y, 's'),
            _ => panic!()
        },
        'L' => match direction {
            'w' => (x - 1, y, 'n'),
            's' => (x, y + 1, 'e'),
            _ => panic!()
        },
        'J' => match direction {
            'e' => (x - 1, y, 'n'),
            's' => (x, y - 1, 'w'),
            _ => panic!()
        },
        '7' => match direction {
            'e' => (x + 1, y, 's'),
            'n' => (x, y - 1, 'w'),
            _ => panic!()
        },
        'F' => match direction {
            'w' => (x + 1, y, 's'),
            'n' => (x, y + 1, 'e'),
            _ => panic!()
        },
        'S' => (x, y, direction),
        _ => panic!()
    }
}

fn is_corner(cell: char) -> bool {
    return cell == '|' || cell == 'L' || cell == 'J' || cell == '7' || cell == 'F' || cell == 'S';
}

fn main() {
    let time = Instant::now();
    let mut buffer: Vec<Vec<char>> = read_lines("input/day10.txt").iter().map(|i| i.chars().collect()).collect();
    let mut starting_x = 0;
    let mut starting_y = 0;
    let mut direction = ' ';
    const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    if let Some((x_pos, line)) = buffer.iter().enumerate().find(|&(_, line)| line.contains(&'S')) {
        starting_y = line.iter().position(|&column| column == 'S').unwrap();
        starting_x = x_pos;
    }

    if buffer[starting_x+1][starting_y] == '|' || buffer[starting_x+1][starting_y] == 'L' || buffer[starting_x+1][starting_y] == 'J' {
        direction = 's';
        starting_x += 1;
    } else if buffer[starting_x-1][starting_y] == '|' || buffer[starting_x-1][starting_y] == '7' || buffer[starting_x-1][starting_y] == 'F' {
        direction = 'n';
        starting_x -= 1;
    } else if buffer[starting_x][starting_y-1] == '-' || buffer[starting_x][starting_y-1] == 'L' || buffer[starting_x][starting_y-1] == 'F' {
        direction = 'w';
        starting_y -= 1;
    } else if buffer[starting_x][starting_y+1] == '-' || buffer[starting_x][starting_y+1] == '7' || buffer[starting_x][starting_y+1] == 'J' {
        direction = 'e';
        starting_y += 1;
    }

    let mut pipe_positions = HashSet::new();
    let mut candidate_nest = HashSet::new();
    pipe_positions.insert((starting_x, starting_y));
    while buffer[starting_x][starting_y] != 'S' {
        (starting_x, starting_y, direction) = get_next_character(&buffer, starting_x, starting_y, direction);
        pipe_positions.insert((starting_x,starting_y));
        for &(dx, dy) in &DIRECTIONS {
            let nx = starting_x as isize + dx;
            let ny = starting_y as isize + dy;
            if nx >= 0 && ny >= 0 && nx < buffer.len() as isize && ny < buffer[0].len() as isize {
                candidate_nest.insert((nx as usize, ny as usize));
            }
        }
    }
    // CHEATING WOOOOOOOO
    buffer[starting_x][starting_y] = '|';

    let mut result = HashSet::new();
    for (x, line) in buffer.iter().enumerate() {
        let mut last_corner = ' ';
        let mut inside_pipes = false;
        for (y, cell) in line.iter().enumerate() {
            if pipe_positions.contains(&(x, y)) && is_corner(*cell) {
                if last_corner == ' ' {
                    last_corner = *cell;
                    if *cell == '|' {
                        inside_pipes = !inside_pipes;
                    }
                } else if last_corner == 'F' && *cell == 'J' {
                    last_corner = ' ';
                    inside_pipes = !inside_pipes;
                } else if last_corner == 'F' && *cell == '7' {
                    last_corner = ' ';
                } else if last_corner == '|' && *cell != '|' {
                    last_corner = *cell;
                } else if last_corner == '|' && *cell == '|' {
                    last_corner = *cell;
                    inside_pipes = !inside_pipes;
                } else if last_corner == 'L' && *cell == 'J' {
                    last_corner = ' ';
                } else if last_corner == 'L' && *cell == '7' {
                    last_corner = ' ';
                    inside_pipes = !inside_pipes;
                }
            } else if inside_pipes && !pipe_positions.contains(&(x, y)) {
                result.insert((x, y));
            }
        }
    }

    println!("{}", Instant::now().duration_since(time).as_secs_f64());
    println!("{}", pipe_positions.len() / 2);
    println!("{}", result.len());
}