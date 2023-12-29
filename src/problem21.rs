use std::collections::HashSet;

use itertools::Itertools;
use queues::{Queue, IsQueue};

use crate::input::read_lines;

pub fn solve_a() {
    println!("{}", solve_a_input("src/test_files/problem21/input.txt"));
}

fn parse_input(lines: Vec<String>) -> Vec<Vec<u8>> {
    lines.iter()
        .map(|x| x.clone().into_bytes())
        .collect_vec()
}

fn solve_a_input(file : &str) -> i64 {
    let lines: Vec<String> = read_lines(file);
    let grid = parse_input(lines);
    let s = 
        (0..grid.len())
            .cartesian_product(0..grid[0].len())
            .find(|(x,y)| grid[*x][*y] == b'S').unwrap();

    let mut q : Queue<((i64, i64), i64)> = Queue::new();
    let _ = q.add(((s.0 as i64, s.1 as i64), 64));

    let mut vis : HashSet<(i64, i64)> = HashSet::new();
    let mut reachable : HashSet<(i64, i64)> = HashSet::new();

    while let Ok((v@(vx, vy), dist)) = q.remove() {
        if vis.contains(&v) {
            continue;
        } else {
            vis.insert(v);
        }
        if dist % 2 == 0 {
            reachable.insert(v);
        }
        if dist == 0 {
            continue;
        }
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let i@(ix, iy) = (vx+dx, vy+dy);
            if ix < 0 || ix >= grid.len() as i64 || iy < 0 || iy >= grid.len() as i64 || grid[ix as usize][iy as usize] == b'#' || vis.contains(&i) {
                continue;
            }
            let _ = q.add((i, dist-1));
        }
    }

    return reachable.len() as i64;
}

pub fn solve_b() {
    println!("{}", solve_b_input("src/test_files/problem21/input.txt"));
}

fn solve_b_input(file : &str) -> i64 {
    let _lines: Vec<String> = read_lines(file);
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_a_example() {
        let ans = solve_a_input("src/test_files/problem21/a_example.txt");
        assert_eq!(ans, 16);
    }

    #[test]
    fn test_solve_b_example() {
        let ans = solve_b_input("src/test_files/problem21/b_example.txt");
        assert_eq!(ans, 281);
    }
}

