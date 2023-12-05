use std::{cmp::min, iter::once};
use itertools::{Itertools, chain};

use crate::input::read_lines;

pub fn solve_a() {
    println!("{}", solve_a_input("src/test_files/problem5/input.txt"));
}

#[derive(Debug)]
struct FromToMap {
    name : String,
    ranges : Vec<(i64, i64, i64)>,
}

fn find_value(map: &FromToMap, key: i64) -> i64 {
    for (dest, src, len) in &map.ranges {
        if *src <= key && key < *src + len {
            return *dest + (key - *src);
        }
    }
    return key;
}

fn interval_intersection((s1, len1) : (i64, i64), (s2, len2) : (i64, i64)) -> Option<(i64,i64)> {
    let s3 = std::cmp::max(s1, s2);
    let len3 = min(s1+len1, s2+len2)-s3;
    if len3 > 0 {
        return Some((s3, len3));
    }
    return None
}

fn between_interval((s1, len1):(i64,i64), (s2, len2):(i64,i64)) -> (i64,i64){
    assert!(s1+len1 <= s2);
    return (s1+len1, s2-(s1+len1))
}

// we assume that all intervals are a subset of s1 and sorted
fn interval_complement((s1, len1) : (i64, i64), intervals: &Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    chain!(
        once((s1, 0)),
        intervals.iter().map(|x| *x),
        once((s1+len1, 0)),
    ).tuple_windows().map(
            |(prev, next)| between_interval(prev, next)
        ).
    filter(|(_, len)| *len > 0).collect_vec()  
}

fn map_interval(from_to_map: &FromToMap, (s, s_len) : (i64, i64)) -> Vec<(i64, i64)> {
    let interval_pairs = from_to_map.ranges.iter()
        .filter_map(|(dest, src, len)|{
            interval_intersection((s, s_len), (*src, *len)).map(
                |intr| ((intr.0 + *dest-*src, intr.1), intr)
            ) 
        }).collect_vec();

    let mut res = interval_pairs.clone().iter()
        .map(|(a,_)| *a)
        .collect_vec();
    res.extend(
            interval_complement(
                    (s, s_len),
          &interval_pairs.clone().iter()
                        .map(|(_,b)| *b).collect_vec(),
                 )
        );
    return res;
}

#[derive(Debug)]
struct ProblemInput {
    seeds: Vec<i64>,
    maps: Vec<FromToMap>,
}

fn parse_input(lines: Vec<String>) -> ProblemInput {
    let mut maps = Vec::new();

    let seeds = lines[0]
        .split(":")
        .collect::<Vec<&str>>()[1]
        .trim()
        .split(" ")
        .map(|x| x.parse().unwrap())
        .collect();
    
    let mut cur_map : FromToMap = FromToMap { name: String::new(), ranges: Vec::new() };
    for i in 2..lines.len() {
        if lines[i].ends_with(" map:") {
            cur_map.name = lines[i].strip_suffix(" map:").unwrap().to_string();
        } else if lines[i].is_empty() {
            maps.push(cur_map);
            cur_map = FromToMap { name: String::new(), ranges: Vec::new() };
        } else {
            let ranges: Vec<i64> = 
                lines[i]
                .split(" ")
                .map(|x| x.parse().unwrap())
                .collect();
            cur_map.ranges.push((ranges[0], ranges[1], ranges[2]));
        }
    }
    maps.push(cur_map);

    //cur_map.ranges.sort_by(|x,y| x.1.cmp(&y.2));
    maps.iter_mut().for_each(|from_to_map| 
        from_to_map.ranges.sort_by(
            |x, y| x.1.cmp(&y.1)
        )
    );
    return ProblemInput { seeds, maps }
}


fn solve_a_input(file : &str) -> i64 {
    let lines: Vec<String> = read_lines(file);
    let input: ProblemInput = parse_input(lines);
    let mut min_val = i64::MAX;
    for seed in &input.seeds {
        let mut cur_val = *seed;
        for map in &input.maps {
            cur_val = find_value(map, cur_val);
        }
        min_val = min(cur_val, min_val);
    }
    return min_val;
}

pub fn solve_b() {
    println!("{}", solve_b_input("src/test_files/problem5/input.txt"));
}

fn solve_b_input(file : &str) -> i64 {
    let lines: Vec<String> = read_lines(file);
    let input: ProblemInput = parse_input(lines);

    let res = input.seeds.iter()
        .tuples()
        .map(|(s, s_len):(&i64, &i64)|{
            input.maps.iter().fold(Vec::from([(*s, *s_len)]), |intervals, from_to_map|{
                intervals.iter()
                    .map(|intr| map_interval(from_to_map, *intr)).concat()
            })
        }).
        concat();
            
    return res.iter()
            .min().unwrap().0;
}

mod tests {
    use crate::input::read_lines;
    use crate::problem5::interval_intersection;

    use super::ProblemInput;
    use super::interval_complement;
    use super::parse_input;
    use super::solve_a_input;
    use super::solve_b_input;

    #[test]
    fn test_solve_a_example() {
        let ans = solve_a_input("src/test_files/problem5/a_example.txt");
        assert_eq!(ans, 35);
    }

    #[test]
    fn test_interval_intersection() {
        let i1 = (5,3); // 5,6,7
        let i2 = (6,4); // 6,7,8,9
        let i3 = (4,2); // 4,5
        let i4 = (7,2); // 7,8
        assert_eq!(interval_intersection(i1, i2), Some((6, 2)));
        assert_eq!(interval_intersection(i2, i3), None);
        assert_eq!(interval_intersection(i2, i4), Some((7,2)));
    }

    #[test]
    fn test_interval_complement() {
        let intervals = vec![(5,1), (7,2), (10,2), (14, 3), (18,2)];
        assert_eq!(interval_complement((5, 15), &intervals), vec![(6,1), (9,1), (12,2), (17,1)]);
        assert_eq!(interval_complement((0, 21), &intervals), vec![(0,5), (6,1), (9,1), (12,2), (17,1), (20,1)]);
    }

    #[test]
    fn test_solve_b_example() {
        let ans = solve_b_input("src/test_files/problem5/a_example.txt");
        assert_eq!(ans, 46);
    }
}

