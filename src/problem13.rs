use itertools::Itertools;

use crate::input::read_lines;

pub fn solve_a() {
    println!("{}", solve_a_input("src/test_files/problem13/input.txt"));
}

fn parse_input(lines: Vec<String>) -> Vec<Vec<Vec<u8>>> {
    lines
        .split(|f| f.is_empty())
        .map(|mirror|
             mirror.iter()
            .map(|s| s.clone().into_bytes())
            .collect_vec()
        ).collect_vec()
}

fn calculate_mirror(mirror: &Vec<Vec<u8>>) -> i64 {
    if let Some(horizontal) = (1..mirror.len())
        .map(|pos| ({
            let diff = std::cmp::min(pos, mirror.len()-pos);
            (&mirror[pos-diff..pos], &mirror[pos..pos+diff])
        }))
        .position(|(up, down)|{
            return up.iter().zip(down.iter().rev()).all(|(l,r)| l == r);
        }) {
            return 100*(horizontal+1) as i64;
        }

    if let Some(vertical) = (1..mirror[0].len())
        .map(|pos| {
            mirror.iter().map(|row| {
                let diff = std::cmp::min(pos, mirror[0].len()-pos);
                (&row[pos-diff..pos], &row[pos..pos+diff])
        }).collect_vec()
        }).position(|columns| {
            columns.iter()
                .all(|(left, right)|{
                    left.iter().zip(right.iter().rev()).all(|(l,r)| l == r)
                })
        }) {
            return (vertical+1) as i64;
        }
    return 0;
}

fn calculate_mirror_all(mirror: &Vec<Vec<u8>>) -> Vec<i64> {
    let mut res = Vec::new();
    let horizontals = (1..mirror.len())
        .map(|pos| ({
            let diff = std::cmp::min(pos, mirror.len()-pos);
            (&mirror[pos-diff..pos], &mirror[pos..pos+diff])
        }))
        .enumerate()
        .filter(|(_, (up, down))|{
            return up.iter().zip(down.iter().rev()).all(|(l,r)| l == r);
        })
        .map(|(pos, _)| 100*(pos+1) as i64)
        .collect_vec();
    res.extend(horizontals);

    let verticals = (1..mirror[0].len())
        .map(|pos| {
            mirror.iter().map(|row| {
                let diff = std::cmp::min(pos, mirror[0].len()-pos);
                (&row[pos-diff..pos], &row[pos..pos+diff])
        }).collect_vec()
        })
        .enumerate()
        .filter(|(_, columns)| {
            columns.iter()
                .all(|(left, right)|{
                    left.iter().zip(right.iter().rev()).all(|(l,r)| l == r)
                })
        })
        .map(|(pos, _)| (pos+1) as i64)
        .collect_vec();

    res.extend(verticals);
    return res;
}

fn solve_a_input(file : &str) -> i64 {
    let lines: Vec<String> = read_lines(file);
    let input = parse_input(lines);

    return input.iter().map(calculate_mirror).sum();
}

pub fn solve_b() {
    println!("{}", solve_b_input("src/test_files/problem13/input.txt"));
}

fn solve_b_input(file : &str) -> i64 {
    let lines: Vec<String> = read_lines(file);
    let input = parse_input(lines);
    return input.iter()
        .map(|mirror|{
            (0..mirror.len())
                .map(|i| {
                    (0..mirror[i].len())
                        .map(|j| {
                            let mut cloned_mirror = mirror.clone();
                            cloned_mirror[i][j] = match cloned_mirror[i][j] {
                                    b'#' => b'.',
                                    b'.' => b'#',
                                    _ => b'?',
                                };
                            return cloned_mirror;
                    }).collect_vec()
                }).flatten().collect_vec()
        })
        .zip(input.iter())
        .map(|(generated_mirrors, mirror)|{
            let mirror_value = calculate_mirror(mirror);
            generated_mirrors.iter()
                .map(calculate_mirror_all).flatten()
                    .find(|x| *x != mirror_value && *x != 0)
        })
        .map(|x| x.unwrap()).sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_a_example() {
        let ans = solve_a_input("src/test_files/problem13/a_example.txt");
        assert_eq!(ans, 405);
    }

    #[test]
    fn test_solve_b_example() {
        let ans = solve_b_input("src/test_files/problem13/a_example.txt");
        assert_eq!(ans, 400);
    }
}

