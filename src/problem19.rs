use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::input::read_lines;

pub fn solve_a() {
    println!("{}", solve_a_input("src/test_files/problem19/input.txt"));
}

#[derive(Clone, PartialEq, Hash, Eq, Debug)]
enum WorkflowResult {
    Accept,
    Reject,
    Move(String),
}

#[derive(Clone, PartialEq, Hash, Eq, Debug)]
struct Part {
    x: i64, m: i64, a: i64, s: i64,
}
impl Part{
    fn select_mut(&mut self, c: char) -> &mut i64 {
        match c {
            'x' => &mut self.x,
            'm' => &mut self.m,
            'a' => &mut self.a,
            's' => &mut self.s,
            _ => unreachable!(),
        }
    }
    fn select(&self, c: char) -> i64 {
        match c {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, PartialEq, Hash, Eq, Debug)]
struct Rule {
    rating: char,
    cmp: char,
    value: i64,
    result: WorkflowResult,
}

#[derive(Clone, PartialEq, Hash, Eq, Debug)]
struct Workflow {
    rules: Vec<Rule>,
    default_result: WorkflowResult,
}


fn parse_input(lines : Vec<String>) -> (HashMap<String, Workflow>, Vec<Part>) {
    let mut workflows: HashMap<String, Workflow> = HashMap::new();
    let mut parts: Vec<Part> = Vec::new();

    let (unparsed_workflows, unparsed_parts) = lines.split_at(
        lines.iter().position(|x| x.len() == 0).unwrap()
    );

    let workflow_result_from_string = |s:&str|{
            if s == "A" {
                return WorkflowResult::Accept
            } else if s == "R" {
                return WorkflowResult::Reject
            } else {
                return WorkflowResult::Move(s.parse().unwrap())
            };
        };

    for unparsed_workflow in unparsed_workflows {
        let result = sscanf::sscanf!(unparsed_workflow, "{str}{{{str}}}").unwrap();
        let workflow_name = result.0;

        let split = result.1.split(",").collect_vec();
        
        let rules = split.iter().take(split.len()-1).map(|unparsed_rule|{
            let (rating, cmp, value, result) = sscanf::sscanf!(unparsed_rule, "{char}{char}{i64}:{str}").unwrap();
            return Rule{
                rating,
                cmp, 
                value,
                result: workflow_result_from_string(result)
            }
        }).collect_vec();

        let default_result = workflow_result_from_string(split.last().unwrap());
        workflows.insert(workflow_name.to_owned(), Workflow{rules, default_result});
    }

    for unparsed_part in unparsed_parts {
        if let Ok((x,m,a,s)) = sscanf::sscanf!(unparsed_part, "{{x={i64},m={i64},a={i64},s={i64}}}") {
            parts.push(Part{x,m,a,s});
        }
    }

    return (workflows, parts);
}

fn apply_workflow<'a>(part: &'a Part, workflow: &'a Workflow) -> &'a WorkflowResult {
    for rule in &workflow.rules {
        let lhs = part.select(rule.rating);
        let cmp_result = match rule.cmp {
            '<' => lhs < rule.value,
            '>' => lhs > rule.value,
            _ => unreachable!(),
        };
        if cmp_result {
            return &rule.result;
        }
    }
    
    return &workflow.default_result;
}

fn solve_a_input(file : &str) -> i64 {
    let lines: Vec<String> = read_lines(file);
    let (workflows, parts) = parse_input(lines);

    let mut res = 0;
    for part in &parts {
        let mut recent_result = WorkflowResult::Move(String::from("in"));
        while let WorkflowResult::Move(destination) = &recent_result {
            recent_result = apply_workflow(&part, &workflows.get(destination).unwrap()).clone();
        }
        if recent_result == WorkflowResult::Accept {
            res += part.x + part.m + part.a + part.s;
        }
    }
    return res;
}

fn split_part(part : &(Part, Part), rating: char, value: i64, cmp: char) -> ((Part, Part), (Part, Part)) {
    let upper_bound;
    let lower_bound;

    match cmp {
        '<' => {
            upper_bound = value-1;
            lower_bound = value;
        },
        '>' => {
            upper_bound = value;
            lower_bound = value+1;
        },
        _ => unreachable!(),
    };

    let (mut lower_bound_constrain, mut upper_bound_constrain) = part.clone();
    *upper_bound_constrain.select_mut(rating) = upper_bound;
    *lower_bound_constrain.select_mut(rating) = lower_bound;

    return ((part.0.clone(), upper_bound_constrain), (lower_bound_constrain, part.1.clone()));
}

fn calculate_rec(workflows: &HashMap<String, Workflow>, workflow: &String, constrain: &(Part, Part)) -> HashSet<(Part, Part)> { 
    let workflow = workflows.get(workflow).unwrap();
    let mut res = HashSet::new();
    let mut next_part = constrain.clone();
    let mut current_part;
    for Rule{rating, cmp, value, result} in &workflow.rules {
        let (l, r) = split_part(&next_part, *rating, *value, *cmp);
        match cmp {
            '<' => {
                current_part = l.clone();
                next_part = r.clone();
            },
            '>' => {
                current_part = r.clone();
                next_part = l.clone();
            },
            _ => unreachable!()
        }
        match result {
            WorkflowResult::Accept => {
                res.insert(current_part);
            },
            WorkflowResult::Reject => {
                continue;
            }
            WorkflowResult::Move(new_workflow) => {
                res.extend(
                    calculate_rec(workflows, new_workflow, &current_part)
                );
            }
        }
    }

    match &workflow.default_result {
        WorkflowResult::Accept => {
            res.insert(next_part);
        },
        WorkflowResult::Reject => {
        }
        WorkflowResult::Move(new_workflow) => {
            res.extend(
                calculate_rec(workflows, &new_workflow, &next_part)
            );
        }
    }

    return res;
}

pub fn solve_b() {
    println!("{}", solve_b_input("src/test_files/problem19/input.txt"));
}

fn solve_b_input(file : &str) -> i64 {
    let lines: Vec<String> = read_lines(file);
    let (workflows, _) = parse_input(lines);
    let constrains : HashSet<(Part, Part)> = calculate_rec(&workflows, &String::from("in"),
         &(Part{x:1,m:1,a:1,s:1}, Part{x:4000, m:4000, a:4000, s:4000}));

    //dbg!(&constrains);
    let res = 
        constrains.iter()
            .map(|(l,r)|{
                (r.x-l.x+1)*(r.m-l.m+1)*(r.a-l.a+1)*(r.s-l.s+1)
            })
            .sum();

    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_a_example() {
        let ans = solve_a_input("src/test_files/problem19/a_example.txt");
        assert_eq!(ans, 19114);
    }

    #[test]
    fn test_solve_b_example() {
        let ans = solve_b_input("src/test_files/problem19/a_example.txt");
        assert_eq!(ans, 167409079868000);
    }
}

