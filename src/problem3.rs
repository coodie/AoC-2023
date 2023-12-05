use std::collections::HashSet;

use crate::input::read_lines;

pub fn solve_a() {
    println!("{}", solve_a_input("src/test_files/problem3/input.txt"));
}

fn surrounding_indices(i_usize:usize,j_usize:usize,n_usize:usize,m_usize:usize) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    let i = i_usize as i32;
    let j = j_usize as i32;
    let n = n_usize as i32;
    let m = m_usize as i32;

    for x in i-1..i+2 {
        for y in j-1..j+2 {
            if x >= 0 && y >= 0 && x < n && y < m {
                res.push((x as usize,y as usize))
            }
        }
    }

    return res;
}

fn solve_a_input(file : &str) -> u32 {
    let lines: Vec<Vec<char>> = read_lines(file).iter().map(|x|  x.as_bytes().iter().map(|y| *y as char).collect()).collect();
    let n = lines.len();
    let m = lines[0].len();
    let mut cur_num: u32 = 0;
    let mut symbol: bool = false;
    let mut res: u32 = 0;
    for i in 0..n {
        for j in 0..m {
            if lines[i][j].is_ascii_digit() {
                cur_num = cur_num * 10 + lines[i][j].to_digit(10).unwrap();

                let indices = surrounding_indices(i, j, n, m);
                for (x, y) in indices {
                    if !lines[x][y].is_ascii_digit() && lines[x][y] != '.' {
                       symbol = true;
                    }
                }
            } else {
                if symbol {
                    res += cur_num;
                }
                cur_num = 0;
                symbol = false;
            } 
        }
        if symbol {
            res += cur_num;
        }
        cur_num = 0;
        symbol = false;
    }
    return res;
}

pub fn solve_b() {
    println!("{}", solve_b_input("src/test_files/problem3/input.txt"));
}

fn solve_b_input(file : &str) -> u32 {
    let lines: Vec<Vec<char>> = read_lines(file).iter().map(|x|  x.as_bytes().iter().map(|y| *y as char).collect()).collect();
    let n = lines.len();
    let m = lines[0].len();

    let mut gears = vec![vec![Vec::<u32>::new(); m]; n];

    let mut cur_num: u32 = 0;
    let mut touched_gears: HashSet<(usize, usize)> = HashSet::new();
    let mut res: u32 = 0;
    for i in 0..n {
        for j in 0..m {
            if lines[i][j].is_ascii_digit() {
                cur_num = cur_num * 10 + lines[i][j].to_digit(10).unwrap();

                let indices = surrounding_indices(i, j, n, m);
                for (x, y) in indices {
                    if lines[x][y] == '*' {
                       touched_gears.insert((x,y));
                    }
                }
            } else {
                if cur_num != 0 {
                    for (x,y) in &touched_gears {
                        gears[*x][*y].push(cur_num);
                    }
                }
                cur_num = 0;
                touched_gears.clear();
            } 
        }
        if cur_num != 0 {
            for (x,y) in &touched_gears {
                gears[*x][*y].push(cur_num);
            }
        }
        cur_num = 0;
        touched_gears.clear();
    }

    for row in gears {
        for gear in row {
            if gear.len() == 2 {
                res += gear[0]*gear[1];
            }
        }
    }
    return res;
}

mod tests {
    use super::solve_a_input;
    use super::solve_b_input;
    use super::surrounding_indices;

    #[test]
    fn test_solve_a_example() {
        let ans = solve_a_input("src/test_files/problem3/a_example.txt");
        assert_eq!(ans, 4361);
    }

    #[test]
    fn test_solve_a_test() {
        let ans = solve_a_input("src/test_files/problem3/test1.txt");
        assert_eq!(ans, 102);
    }
    #[test]
    fn test_solve_b_example() {
        let ans = solve_b_input("src/test_files/problem3/a_example.txt");
        assert_eq!(ans, 467835);
    }
}
