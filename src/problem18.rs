use itertools::Itertools;
use crate::{input::read_lines, direction::*, direction::Direction::*};

fn build_polygon(instructions: &Vec<(Direction, i64)>) ->(i64, Vec<(i64, i64)>) {
    let mut cur = (0, 0);
    let mut res : Vec<(i64, i64)> = vec![cur];
    let mut perimeter = 0;

    for (dir, dist) in instructions {
        let shift = get_shift::<i64>(*dir);
        cur.0 += shift.0 * dist;
        cur.1 += shift.1 * dist;
        res.push(cur);
        perimeter += dist;
    }

    return (perimeter, res);
}

fn calculate_area(polygon: &Vec<(i64, i64)>) -> i64 {
    return num::abs(polygon.iter()
    .tuple_windows()
    .map(|(pi, pi_next)|{
        pi_next.0*pi.1-pi.0*pi_next.1 
    })
    .sum::<i64>() / 2);
}

pub fn solve_a() {
    println!("{}", solve_a_input("src/test_files/problem18/input.txt"));
}

fn parse_input_a(lines: Vec<String>) -> Vec<(Direction, i64)> {
    lines.iter()
        .map(|line|{
            let s = line.split(" ").collect_vec();
            let dir = match s[0].as_bytes()[0] {
                b'R' => Right, 
                b'L' => Left,
                b'U' => Up,
                b'D' => Down,
                _ => unimplemented!("error"),
            };
            return (
                dir,
                s[1].parse().unwrap(),
            )
        }).collect_vec()
}

fn solve_a_input(file : &str) -> i64 {
    let lines: Vec<String> = read_lines(file);
    let instructions = parse_input_a(lines);
    let (perimeter, polygon) = build_polygon(&instructions);

    let area = calculate_area(&polygon);
    return perimeter/2 + 1 + area;
}

pub fn solve_b() {
    println!("{}", solve_b_input("src/test_files/problem18/input.txt"));
}

fn parse_input_b(lines: Vec<String>) -> Vec<(Direction, i64)> {
    lines.iter()
        .map(|line|{
            let s = line.split(" ").collect_vec();
            let dir = match s[2].as_bytes()[s[2].len()-2] {
                b'0' => Right, 
                b'1' => Down,
                b'2' => Left,
                b'3' => Up,
                _ => unimplemented!("error"),
            };
            let step = i64::from_str_radix(&s[2][2..s[2].len()-2], 16).unwrap();
            return (
                dir,
                step
            )
        }).collect_vec()
}

fn solve_b_input(file : &str) -> i64 {
    let lines: Vec<String> = read_lines(file);
    let instructions = parse_input_b(lines);
    let (perimeter, polygon) = build_polygon(&instructions);

    let area = calculate_area(&polygon);
    return perimeter/2 + 1 + area;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_area_test() {
        let polygon = vec![
            (1,6),
            (3,1),
            (7,2),
            (4,4),
            (8,5),
            (1,6),
        ];
        let area = calculate_area(&polygon);
        assert_eq!(area, 16);

        let polygon = vec![
            (0,0),
            (0,3),
            (4,3),
            (4,0),
            (0,0),
        ];

        let area = calculate_area(&polygon);
        assert_eq!(area, 12);
    }

    #[test]
    fn test_solve_a_example() {
        let ans = solve_a_input("src/test_files/problem18/a_example.txt");
        assert_eq!(ans, 62);
    }

    #[test]
    fn test_solve_b_example() {
        let ans = solve_b_input("src/test_files/problem18/a_example.txt");
        assert_eq!(ans, 952408144115);
    }
}

