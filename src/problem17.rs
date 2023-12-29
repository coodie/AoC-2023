use itertools::Itertools;
use pathfinding::directed::dijkstra::dijkstra_all;

use crate::input::read_lines;

pub fn solve_a() {
    println!("{}", solve_a_input("src/test_files/problem17/input.txt"));
}

fn parse_input(lines: Vec<String>) -> Vec<Vec<i64>> {
    lines.iter()
        .map(|row|{
            row.as_bytes().iter()
                .map(|c| (*c as char).to_digit(10).unwrap() as i64)
                .collect_vec()
        }).collect_vec()
}


#[derive(Clone, PartialEq, Hash, Eq, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    NoDir,
}

impl Direction {
    fn opposite(self) -> Direction {
        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
            NoDir => NoDir,
        }
    }
}

const ALL_DIRS  : [Direction; 4] =  [Up, Down, Left, Right];

use crate::problem17::Direction::*;

type Point = (usize, usize);

fn get_shift(dir: Direction) -> (i32, i32) {
    match dir {
        Down => (1, 0),
        Up => (-1, 0),
        Left => (0, -1),
        Right => (0, 1),
        _ => unimplemented!("error"),
    }
}

fn go_n(dir: Direction, pt: Point, n: usize, mx: Point) -> Option<Point> {
    let shift_1 = get_shift(dir);
    let shift = (shift_1.0 * n as i32, shift_1.1 * n as i32);
    let pt_i32 = (pt.0 as i32, pt.1 as i32);
    let new_pt = (pt_i32.0 + shift.0, pt_i32.1 + shift.1);

    if new_pt.0 < 0 || new_pt.0 >= mx.0 as i32 || new_pt.1 < 0 || new_pt.1 >= mx.1 as i32 {
        return None;
    }

    return Some((new_pt.0 as usize, new_pt.1 as usize));
}

#[derive(Clone, PartialEq, Hash, Eq, Copy, Debug)]
struct Node{
    pt: Point,
    dir: Direction
}

fn solve(least: usize, most: usize, heatmap: &Vec<Vec<i64>>) -> i64 {
    let mx = (heatmap.len(), heatmap[0].len());
    let start = Node{ 
        pt : (0, 0),
        dir: NoDir,
    };

    let cost_map =
         dijkstra_all(&start, |node: &Node|{
            let res =
                 ALL_DIRS.iter()
                .filter(|dir| node.dir != **dir && node.dir != dir.opposite())
                .map(|step_dir |{
                    let mut neighbours = Vec::<(Node, i64)>::new();
                    let mut cost = 0;
                    for dist in 1..most {
                        if let Some(new_pt) = go_n(*step_dir, node.pt, dist, mx) {
                            cost += heatmap[new_pt.0][new_pt.1];
                            if dist >= least {
                                neighbours.push((
                                    Node{
                                        pt: new_pt,
                                        dir: *step_dir,
                                    },
                                    cost
                                ));
                            }
                        } else {
                            break;
                        }
                    }
                    return neighbours;
                })
                .flatten().collect_vec();
                return res;
        });

    let finish_pt = (mx.0-1, mx.1-1);
    let min_last : i64 = ALL_DIRS.iter()
            .filter_map(|dir |{
                let node = Node {
                    pt: finish_pt,
                    dir: *dir,
                };
                return cost_map.get(&node);
            }).map(|x| {
                return x.1;
            }).min().unwrap();

    return min_last;
}

fn solve_a_input(file : &str) -> i64 {
    let lines: Vec<String> = read_lines(file);
    let heatmap = parse_input(lines);
    return solve(1, 4, &heatmap);
}


pub fn solve_b() {
    println!("{}", solve_b_input("src/test_files/problem17/input.txt"));
}

fn solve_b_input(file : &str) -> i64 {
    let lines: Vec<String> = read_lines(file);
    let heatmap = parse_input(lines);
    return solve(4, 11, &heatmap);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_a_example() {
        let ans = solve_a_input("src/test_files/problem17/a_example.txt");
        assert_eq!(ans, 102);
    }

    #[test]
    fn test_solve_b_example() {
        let ans = solve_b_input("src/test_files/problem17/a_example.txt");
        assert_eq!(ans, 94);
    }
}

