use std::collections::HashMap;

use itertools::Itertools;

use crate::input::read_lines;
use crate::itertools2d::{column_iterator, transpose};

pub fn solve_a() {
    println!("{}", solve_a_input("src/test_files/problem14/input.txt"));
}

fn parse_input(lines: Vec<String>) -> Vec<Vec<u8>>{
    lines.iter()
        .map(|line| line.clone().into_bytes())
        .collect_vec()
}

fn calculate_load(column : Vec<u8>) -> i64 {
    let mut res = 0;
    let mut cur_load = column.len();
    for i in 0..column.len() {
        match column[i] {
            b'O' => {
                res += cur_load;
                cur_load -= 1;
            },
            b'#' => {
                cur_load = column.len()-i-1;
            },
            _ => ()
        }
    }
    return res as i64;
}

fn tilt_west(arr: &mut Vec<Vec<u8>>) {
    for row in arr {
        let _ = row.as_mut_slice()
            .split_mut(|c| *c == b'#')
            .map(|group|{
                group.sort_by(|a, b| b.cmp(a))
            }).count();
    }
}

fn tilt_east(arr: &mut Vec<Vec<u8>>) {
    for row in &mut *arr {
        row.reverse();
    }
    tilt_west(&mut* arr);
    for row in  &mut *arr {
        row.reverse();
    }
}

fn tilt_north(arr: &mut Vec<Vec<u8>>) {
    let mut transposed = transpose(arr);
    tilt_west(&mut transposed);
    *arr = transpose(&transposed);
}

fn tilt_south(arr: &mut Vec<Vec<u8>>) {
    let mut transposed = transpose(arr);
    tilt_east(&mut transposed);
    *arr = transpose(&transposed);
}

fn solve_a_input(file : &str) -> i64 {
    let lines: Vec<String> = read_lines(file);
    let input = parse_input(lines);
    return column_iterator(&input).map(calculate_load).sum();
}

pub fn solve_b() {
    println!("{}", solve_b_input("src/test_files/problem14/input.txt"));
}

fn north_load(rows: &Vec<Vec<u8>>) -> i64 {
    let mut res = 0;
    for col in column_iterator(rows) {
        for (i, el) in col.iter().enumerate() {
            match el {
                b'O' => {res += col.len() - i},
                _ => (),
            }
        }
    }
    return res as i64;
}

fn solve_b_input(file : &str) -> i64 {
    let lines: Vec<String> = read_lines(file);
    let mut stones = parse_input(lines);
    let mut vis_so_far : HashMap<Vec<Vec<u8>>, usize> = HashMap::new();
    vis_so_far.insert(stones.clone(), 0);

    let mut cycle_len = 0;
    let mut cycle_start = 0;
    for i in 1..1000000000{
        tilt_north(&mut stones);
        tilt_west (&mut stones);
        tilt_south(&mut stones);
        tilt_east (&mut stones);
        if let Some(backtrack_location) = vis_so_far.get(&stones) {
            cycle_len = vis_so_far.len()-*backtrack_location;
            cycle_start = *backtrack_location;
            break;
        }

        vis_so_far.insert(stones.clone(), i);
    }
    let location = (1000000000-cycle_start)%cycle_len+cycle_start;


    let final_stones = vis_so_far.iter()
        .find(|(_, i)| **i == location).unwrap().0;

    return  north_load(final_stones);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_a_example() {
        let ans = solve_a_input("src/test_files/problem14/a_example.txt");
        assert_eq!(ans, 136);
    }

    #[test]
    fn tilt_test() {
        let mut input  =  vec![Vec::from("O.#..O.#.#..O..#.OO.O".as_bytes())];
        let output  = vec![Vec::from("O.#O...#.#O....#OOO..".as_bytes())];
        
        tilt_west(&mut input);
        assert_eq!(input, output);

        let mut input  =  vec![Vec::from("O.#..O.#.#..O..#.OO.O".as_bytes())];
        let output  = vec![Vec::from(".O#...O#.#....O#..OOO".as_bytes())];
        
        tilt_east(&mut input);
        assert_eq!(input, output);

        let mut input  =  vec![
                Vec::from("..#".as_bytes()),
                Vec::from("O..".as_bytes()),
                Vec::from("#O#".as_bytes()),
                Vec::from("O..".as_bytes()),
                Vec::from(".O.".as_bytes()),
                Vec::from("OOO".as_bytes()),
                ];

        let output  =  vec![
                Vec::from("OO#".as_bytes()),
                Vec::from(".O.".as_bytes()),
                Vec::from("#O#".as_bytes()),
                Vec::from("O.O".as_bytes()),
                Vec::from("O..".as_bytes()),
                Vec::from("...".as_bytes()),
                ];
        
        tilt_north(&mut input);
        assert_eq!(input, output);

        let mut input  =  vec![
                Vec::from("..#".as_bytes()),
                Vec::from("O..".as_bytes()),
                Vec::from("#O#".as_bytes()),
                Vec::from("O..".as_bytes()),
                Vec::from(".O.".as_bytes()),
                Vec::from("OOO".as_bytes()),
                ];

        let output  =  vec![
                Vec::from("..#".as_bytes()),
                Vec::from("O..".as_bytes()),
                Vec::from("#.#".as_bytes()),
                Vec::from(".O.".as_bytes()),
                Vec::from("OO.".as_bytes()),
                Vec::from("OOO".as_bytes()),
                ];
        
        tilt_south(&mut input);
        assert_eq!(input, output);
    }

    #[test]
    fn test_solve_b_example() {
        let ans = solve_b_input("src/test_files/problem14/a_example.txt");
        assert_eq!(ans, 64);
    }
}


// 0, 1, 2, 3, 4, 5, 6, 7, 8, 9 
// (10 - 3) % 7 = 
//
//
