use std::collections::HashSet;
use rayon::prelude::*;

use itertools::Itertools;
use queues::{Queue, IsQueue};

use crate::input::read_lines;

pub fn solve_a() {
    println!("{}", solve_a_input("src/test_files/problem16/input.txt"));
}

#[derive(Clone, PartialEq, Hash, Eq, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

use crate::problem16::Direction::*;

type Point = (usize, usize);

fn get_shift(dir: Direction) -> (i32, i32) {
    match dir {
        Down => (1, 0),
        Up => (-1, 0),
        Left => (0, -1),
        Right => (0, 1),
    }
}

fn go(dir: Direction, pt: Point, mx: Point) -> Option<(Point, Direction)> {
    let shift = get_shift(dir);
    let pt_i32 = (pt.0 as i32, pt.1 as i32);
    let new_pt = (pt_i32.0 + shift.0, pt_i32.1 + shift.1);

    if new_pt.0 < 0 || new_pt.0 >= mx.0 as i32 || new_pt.1 < 0 || new_pt.1 >= mx.1 as i32 {
        return None;
    }

    return Some(((new_pt.0 as usize, new_pt.1 as usize), dir));
}

fn calculate_reflection(tile: u8, dir: Direction) -> Vec<Direction> {
    return match dir {
        Up => match tile {
            b'/'  => vec![Right],
            b'\\' => vec![Left],
            b'|'  => vec![Up],
            b'-'  => vec![Right, Left],
            b'.' => vec![dir],
            _ => vec![],
        },
        Down => match tile {
            b'/'  => vec![Left],
            b'\\' => vec![Right],
            b'|'  => vec![Down],
            b'-'  => vec![Right, Left],
            b'.' => vec![dir],
            _ => vec![],
        }, 
        Left => match tile {
            b'/'  => vec![Down],
            b'\\' => vec![Up],
            b'|'  => vec![Up, Down],
            b'-'  => vec![Left],
            b'.' => vec![dir],
            _ => vec![],
        },
        Right => match tile {
            b'/'  => vec![Up],
            b'\\' => vec![Down],
            b'|'  => vec![Up, Down],
            b'-'  => vec![Right],
            b'.' => vec![dir],
            _ => vec![],
        },
    }
}

fn calculate_energized(array: &Vec<String>, start_pt_dir: (Point, Direction)) -> i64 {
    let mut energized = vec![vec![false; array[0].len()]; array.len()];
    let mut visited : HashSet<(Point, Direction)> = HashSet::new();
    let mut q: Queue<(Point, Direction)> = Queue::new();
    let mx = (array.len(), array[0].len());

    let _ = q.add(start_pt_dir);

    while let Ok(pt_dir@(pt@(x,y), dir)) = q.remove() {
        let tile = array[x].as_bytes()[y];
        energized[x][y] = true;

        if visited.contains(&pt_dir) {
            continue;
        }
        visited.insert(pt_dir);

        let new_nodes = calculate_reflection(tile, dir)
            .iter()
            .filter_map(|dir|{ go(*dir, pt, mx) })
            .collect_vec();

        for x in new_nodes {
            if let Err(msg) = q.add(x) {
                panic!("Queue insertion failed with {}", msg);
            }
        }
    }

    return energized.iter().map(|x| x.iter().filter(|x| **x).count()).sum::<usize>() as i64;
}

fn solve_a_input(file : &str) -> i64 {
    let array: Vec<String> = read_lines(file);
    return calculate_energized(&array, ((0,0), Right));
}

pub fn solve_b() {
    println!("{}", solve_b_input("src/test_files/problem16/input.txt"));
}

fn solve_b_input(file : &str) -> i64 {
    let array: Vec<String> = read_lines(file);

    let starting_pts = (0..array.len())
        .map(|x|{
            if x == 0 || x == array.len()-1 {
               (0..array.len()-1).collect_vec()
            } else {
              vec![0, array[x].len()-1]
            }.iter().map(|y|  vec![((x,*y),Left),((x,*y),Right),((x,*y),Up),((x,*y),Down)]).flatten().collect::<Vec<_>>()
        }).flatten()
        .collect_vec();

    return starting_pts.par_iter().map(
        |start_pt_dir| calculate_energized(&array, *start_pt_dir))
        .max().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_a_example() {
        let ans = solve_a_input("src/test_files/problem16/a_example.txt");
        assert_eq!(ans, 46);
    }

    #[test]
    fn test_solve_b_example() {
        let ans = solve_b_input("src/test_files/problem16/a_example.txt");
        assert_eq!(ans, 51);
    }
}

