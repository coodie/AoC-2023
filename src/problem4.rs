use std::{collections::{HashSet}, cmp::min};

use crate::input::read_lines;

pub fn solve_a() {
    println!("{}", solve_a_input("src/test_files/problem4/input.txt"));
}

fn parse_input(lines: Vec<String>) -> Vec<(Vec<i32>, Vec<i32>)> {
    return lines.iter().map(|l| {
        let nums = l.split(":").last().unwrap().split("|").map(|x| x.trim().to_string()).collect::<Vec<String>>();
       (
           nums[0].split(" ").filter(|x| !x.is_empty()).map(|x| x.trim().parse::<i32>().unwrap()).collect::<Vec<i32>>(),
           nums[1].split(" ").filter(|x| !x.is_empty()).map(|x| x.trim().parse::<i32>().unwrap()).collect::<Vec<i32>>()
       )
    }).collect();
}

fn special_pow2(num: i32) -> i32 {
    if num == 0 {
        return 0;
    }
    return 2_i32.pow(num as u32-1);

}

fn solve_a_input(file : &str) -> i32 {
    let lines: Vec<String> = read_lines(file);
    let input = parse_input(lines);
    
    return input.iter().map(|nums|{
       let winning_numbers : HashSet<i32> = HashSet::from_iter(nums.0.clone());
       let my_numbers: HashSet<i32>  = HashSet::from_iter(nums.1.clone());
       let intersection: Vec<i32> =  winning_numbers.intersection(&my_numbers).map(|x| *x).collect::<Vec<i32>>();
       return special_pow2(intersection.len() as i32);
    }).sum();
}

pub fn solve_b() {
    println!("{}", solve_b_input("src/test_files/problem4/input.txt"));
}

fn count_matches(nums: &(Vec<i32>, Vec<i32>)) -> i32 {
    let winning_numbers : HashSet<i32> = HashSet::from_iter(nums.0.clone());
    let my_numbers: HashSet<i32>  = HashSet::from_iter(nums.1.clone());
    let intersection: Vec<i32> =  winning_numbers.intersection(&my_numbers).map(|x| *x).collect::<Vec<i32>>();
    return intersection.len() as i32;
}

fn solve_b_input(file : &str) -> u32 {
    let lines: Vec<String> = read_lines(file);
    let input = parse_input(lines);
    let matches: Vec<i32> = input.iter().map(count_matches).collect();
    let mut copy_count = vec![1; input.len()];

    for (i, val) in matches.iter().enumerate() {
        let r = min(i+1+*val as usize, copy_count.len());
        for j in i+1..r {
            copy_count[j] += copy_count[i];
        }
    }
    return copy_count.iter().sum();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_solve_a_example() {
        let ans = solve_a_input("src/test_files/problem4/a_example.txt");
        assert_eq!(ans, 13);
    }

    #[test]
    fn test_solve_b_example() {
        let ans = solve_b_input("src/test_files/problem4/a_example.txt");
        assert_eq!(ans, 30);
    }
}

