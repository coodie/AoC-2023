use std::collections::HashMap;

use crate::input::read_lines;

pub fn solve_a() {
    println!("{}", solve_a_input("src/test_files/problem2/input.txt"));
}

fn parse_game(line: &String) -> Vec<HashMap<String, i32>> {
    let mut game: Vec<HashMap<String, i32>> = Vec::new();

    let game_str: &str = &line.split(":").map(|x| x.trim()).collect::<Vec<&str>>()[1];
    let bags: Vec<&str> = game_str.split(";").map(|x| x.trim()).collect();
    for balls in bags {
        let mut bag: HashMap<String, i32> = HashMap::new();
        bag.insert("red".to_string(), 0);
        bag.insert("green".to_string(), 0);
        bag.insert("blue".to_string(), 0);
        for ball_str in balls.split(", ") {
            let ball: Vec<&str> = ball_str.split(" ").collect();
            let num: i32 = ball[0].parse().unwrap();
            let color: String = ball[1].to_string();
            bag.insert(color, num);
        }

        game.push(bag);
    }
    return game;
}

fn solve_a_input(file : &str) -> usize {
    let lines: Vec<String> = read_lines(file);
    let mut res = 0;
    for (i, line) in lines.iter().enumerate() {
        let game = parse_game(line);
        if game.iter().all(|mp| 
            mp["red"] <= 12 && mp["green"] <= 13 && mp["blue"] <= 14
        ) {
            res += i+1;
        }
    }
    return res;
}

pub fn solve_b() {
    println!("{}", solve_b_input("src/test_files/problem2/input.txt"));
}

fn solve_b_input(file : &str) -> i64 {
    let lines: Vec<String> = read_lines(file);
    let mut res: i64 = 0;
    for line in lines {
        let game = parse_game(&line);
        
        let red_cnt = game.iter().map(|mp| mp["red"]).max().unwrap();
        let green_cnt = game.iter().map(|mp| mp["green"]).max().unwrap();
        let blue_cnt = game.iter().map(|mp| mp["blue"]).max().unwrap();

        res += i64::from(red_cnt*green_cnt*blue_cnt);
    }

    return res;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_solve_a_example() {
        let ans = solve_a_input("src/test_files/problem2/a_example.txt");
        assert_eq!(ans, 8);
    }

    #[test]
    fn test_solve_b_example() {
        let ans = solve_b_input("src/test_files/problem2/a_example.txt");
        assert_eq!(ans, 2286);
    }
}