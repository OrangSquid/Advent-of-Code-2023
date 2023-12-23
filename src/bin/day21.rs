use std::{time::Instant, collections::HashSet, mem::swap};

use advent::readlines::read_lines;

fn main() {
    const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (-1, 0), (1, 0), (0, 1)];
    const VALID_POSITIONS: [char; 2] = ['.', 'S'];
    let time = Instant::now();
    let buffer: Vec<Vec<char>> = read_lines("input/day21.txt").into_iter().map(|line| line.chars().collect()).collect();
    let starting_plot = buffer.iter().enumerate().filter_map(|(x, line)| {
        line.iter().enumerate().filter_map(|(y, cell)| {
            if *cell == 'S' {
                Some((x as isize, y as isize))
            } else {
                None
            }
        }).next()
    }).collect::<Vec<_>>();
    let mut current_plots = HashSet::from([starting_plot[0]]);
    let mut next_plots = HashSet::new();
    let mut point_y0: i64 = 0;
    let mut point_y1: i64 = 0;
    let mut point_y2: i64 = 0;


    for i in 0..327 {
        for (plot_x, plot_y) in current_plots.drain() {
            for (dx, dy) in DIRECTIONS {
                let iplot_x = plot_x + dx;
                let iplot_y = plot_y + dy;
                let expanded_plot_x = if iplot_x < 0 {
                    if buffer.len() as isize + iplot_x % buffer.len() as isize == buffer.len() as isize {
                        0
                    } else {
                        buffer.len() as isize + iplot_x % buffer.len() as isize
                    }
                } else {
                    iplot_x % buffer.len() as isize
                };

                let expanded_plot_y = if iplot_y < 0 {
                    if buffer.len() as isize + iplot_y % buffer.len() as isize == buffer.len() as isize {
                        0
                    } else {
                        buffer.len() as isize + iplot_y % buffer.len() as isize
                    }
                } else {
                    iplot_y % buffer[0].len() as isize
                };
                
                if !VALID_POSITIONS.contains(&buffer[expanded_plot_x as usize][expanded_plot_y as usize]) {
                    continue;
                }
                next_plots.insert((iplot_x, iplot_y));
            }
        }
        swap(&mut current_plots, &mut next_plots);
        if i == 64 {
            point_y0 = current_plots.len() as i64;
        } else if i == 195 {
            point_y1 = current_plots.len() as i64;
        } else if i == 326 {
            point_y2 = current_plots.len() as i64;
        }
    }

    const STEPS: i64 = (26501365 - 65) / 131;

    let result = point_y0 + (point_y1 - point_y0) * STEPS + (point_y0 + point_y2 - 2 * point_y1) * (STEPS * (STEPS - 1) / 2);
    println!("{}", Instant::now().duration_since(time).as_secs_f64());
    println!("{}", result);
}
