use std::collections::HashMap;
use itertools::Itertools;

use crate::input::read_lines;

pub fn solve_a() {
    println!("{}", solve_a_input("src/test_files/problem12/input.txt"));
}

struct Row {
    lava : Vec<u8>,
    springs: Vec<i64>,
}

fn parse_input(lines: Vec<String>) -> Vec<Row> {
    return lines.iter()
        .map(|line|{
            let s = line.split(" ").collect_vec();
            Row {
                lava : s[0].as_bytes().iter().map(|x| *x).collect_vec(),
                springs: s[1].split(",").map(|x| x.parse::<i64>().unwrap()).collect_vec(),
            }
        }).collect_vec();
}



fn calculate_row_rec(lava: &[u8], springs: &[i64], cache : &mut HashMap<(Vec<u8>, Vec<i64>), i64>) -> i64 {
    let cache_key = (Vec::from(lava), Vec::from(springs));
    if let Some(cached) = cache.get(&cache_key) {
        return *cached;
    }

    if springs.len() == 0 {
        if lava.iter().find(|x| **x == b'#').is_some() {
            return 0;
        } 
        return 1;
    }

    let current = springs[0] as usize;
    let mut res = 0i64;

    for (i, w) in lava.windows(current+1).enumerate() {
        if lava[..i].iter().any(|x| *x == b'#') {
            break;
        }
        let w_pref = &w[0..w.len()-1];
        let w_last = w.last().unwrap();

        if w_pref.iter().all(|x| *x != b'.') && *w_last != b'#' {
           res += calculate_row_rec(&lava[i+w.len()..], &springs[1..], cache);
        }
    }
    cache.insert( cache_key, res);
    return res;
}

fn calculate_row(row: &Row) -> i64 {
    let mut lava = row.lava.clone();
    lava.push(b'.');
    let mut cache : HashMap<(Vec<u8>, Vec<i64>), i64> = HashMap::new();

    return calculate_row_rec(&lava, &row.springs, &mut cache);
}

fn solve_a_input(file : &str) -> i64 {
    let lines: Vec<String> = read_lines(file);
    let rows = parse_input(lines);

    return rows.iter().map(calculate_row).sum();
}

pub fn solve_b() {
    println!("{}", solve_b_input("src/test_files/problem12/input.txt"));
}

fn solve_b_input(file : &str) -> i64 {
    let lines: Vec<String> = read_lines(file);
    let rows = parse_input(lines)
        .iter().map(|row|{
            Row {
                lava: (0..5)
                    .map(|_| String::from_utf8_lossy(row.lava.as_slice()))
                    .join("?")
                    .into_bytes(),
                springs: row.springs.repeat(5),
            }
    }).collect_vec();

    return rows.iter().map(calculate_row).sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_a_example() {
        let ans = solve_a_input("src/test_files/problem12/a_example.txt");
        assert_eq!(ans, 21);
    }

    #[test]
    fn test_solve_b_example() {
        let ans = solve_b_input("src/test_files/problem12/a_example.txt");
        assert_eq!(ans, 525152);
    }
}

