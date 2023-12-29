use itertools::Itertools;

use crate::input::read_lines;

pub fn solve_a() {
    println!("{}", solve_a_input("src/test_files/problem11/input.txt", 1));
}


fn parse_input(lines: Vec<String>) -> Vec<Vec<u8>> {
    lines.iter()
        .map(|l| l.as_bytes().to_vec())
        .collect_vec()
        
}
fn solve_a_input(file : &str, mul: i64) -> i64 {
    let lines: Vec<String> = read_lines(file);
    let input = parse_input(lines);

    let empty_rows = input.iter()
        .enumerate()
        .filter_map(|(i, row)|{
            if row.iter()
                .all(|c| *c == b'.') {
                    Some(i)
                } else {
                    None
                }
        }).collect_vec();
    
    let empty_columns = (0..input[0].len())
        .map(|col_i|{
            (
                col_i, 
                input.iter()
                    .map(|row| row[col_i])
                    .collect_vec()
            )
        })
        .filter_map(|(i, col)|{
            if col.iter()
                .all(|c| *c == b'.') {
                    Some(i)
                } else {
                    None
                }
        }).collect_vec();

    let stars :Vec<(usize, usize)> = input.iter()
        .enumerate()
        .map(|(i, row)|{
            row.iter()
                .enumerate()
                .filter_map(move |(j, el)|{
                    if *el == b'#' {
                        Some((i, j))
                    } else {
                        None
                    }
                })
        })
        .flatten()
        .collect();
    
    let res = stars.iter()
        .enumerate()
        .map(|(i, s1)|{
            stars.iter()
                .skip(i+1)
                .enumerate()
                .map(|(_j, s2)|{
                    let cnt_between_rows = empty_rows.iter()
                        .filter(|row_i|
                             std::cmp::min(s1.0, s2.0) < **row_i && **row_i < std::cmp::max(s1.0, s2.0))
                        .count() as i64;
                    let cnt_between_cols = empty_columns.iter()
                        .filter(|col_i|
                             std::cmp::min(s1.1, s2.1) < **col_i && **col_i < std::cmp::max(s1.1, s2.1))
                        .count() as i64;
                    let cnt = num::abs(s1.0 as i64 - s2.0 as i64) 
                        + num::abs(s1.1 as i64 - s2.1 as i64)
                        + cnt_between_rows * mul
                        + cnt_between_cols * mul;
                    return cnt;
                })
                .sum::<i64>()
        }).sum::<i64>();
    
    return res;
}

pub fn solve_b() {
    println!("{}", solve_b_input("src/test_files/problem11/input.txt"));
}

fn solve_b_input(file : &str) -> i64 {
    return solve_a_input(file, 1000000-1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_a_example() {
        let ans = solve_a_input("src/test_files/problem11/a_example.txt", 1);
        assert_eq!(ans, 374);
    }

    #[test]
    fn test_solve_b_example() {
        let ans = solve_a_input("src/test_files/problem11/a_example.txt", 9);
        assert_eq!(ans, 1030);
        let ans = solve_a_input("src/test_files/problem11/a_example.txt", 99);
        assert_eq!(ans, 8410);
    }
}
