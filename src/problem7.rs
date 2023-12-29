use std::{cmp::Ordering, collections::HashMap, iter::zip};

use crate::input::read_lines;
use itertools::Itertools;



pub fn solve_a() {
    println!("{}", solve_a_input("src/test_files/problem7/input.txt"));

}


#[derive(Debug)]
struct CardsSet {
    cards: String,
    value: i32,
}

fn parse_input(lines: Vec<String>) -> Vec<CardsSet> {
    lines.iter().map(|line|{
        let res = line.split(" ").collect_vec();
        CardsSet {
            cards: res[0].to_string(),
            value: res[1].parse().unwrap()
        }
    }).collect_vec()
}

const CARD_ORDERING: [char; 13] = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];
const JOKER_CARD_ORDERING: [char; 13] = ['J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A'];

fn card_cmp(a: &char, b: &char) -> Ordering  {
    let a_pos = CARD_ORDERING.iter().position(|x| x == a).unwrap();
    let b_pos = CARD_ORDERING.iter().position(|x| x == b).unwrap();
    return a_pos.cmp(&b_pos);
}

fn joker_card_cmp(a: &char, b: &char) -> Ordering  {
    let a_pos = JOKER_CARD_ORDERING.iter().position(|x| x == a).unwrap();
    let b_pos = JOKER_CARD_ORDERING.iter().position(|x| x == b).unwrap();
    return a_pos.cmp(&b_pos);
}

#[derive(PartialEq, PartialOrd, Debug)]
enum Kind {
    HighCard,
    OnePair,
    TwoPair,
    Three,
    FullHouse,
    Four,
    Five,
}

fn get_kind(cards: &str) -> Kind {
    let mut cnt: HashMap<char, i32> = HashMap::new();
    for card in cards.chars() {
        match cnt.get(&card) {
            Some(old_cnt) => { cnt.insert(card, old_cnt+1); }
            None =>  { cnt.insert(card, 1); }
        }
    }

    let mx = cnt.values().max().unwrap();
    if *mx == 5 {
        return Kind::Five;
    }
    if *mx == 4 {
        return Kind::Four;
    }
    if *mx == 3 {
        let two = cnt.values().find(|x| **x == 2);
        if two.is_some() {
            return Kind::FullHouse;
        }
        else {
            return Kind::Three;
        }
    }
    if *mx == 2 {
        let mx_cnt = cnt.values().filter(|x| **x == 2).count();
        if mx_cnt == 2 {
            return Kind::TwoPair;
        }
        return Kind::OnePair;
    }

    return Kind::HighCard
}

fn get_strongest_kind(cards: &str) -> Kind {
    let cards_with_no_joker = &cards.chars().filter(|x| *x != 'J').collect::<String>();
    let mut cnt: HashMap<char, i32> = HashMap::new();
    for card in cards_with_no_joker.chars() {
        match cnt.get(&card) {
            Some(old_cnt) => { cnt.insert(card, old_cnt+1); }
            None =>  { cnt.insert(card, 1); }
        }
    }
    let maybe_mx = cnt.iter()
        .max_by(|(_,v1),(_,v2)| v1.cmp(v2));

    
    if maybe_mx.is_none() {
        return Kind::Five;
    }

    let mx = maybe_mx.unwrap().0;

    let new_cards = cards.chars().map(|c|
        if c == 'J' {
            *mx
        } else {
            c
        }
    ).collect::<String>();

    return get_kind(&new_cards);
}

fn cmp_card_sets(a: &CardsSet, b: &CardsSet) -> Ordering {
    if a.cards == b.cards {
        return Ordering::Equal;
    }

    let kind_a = get_kind(&a.cards);
    let kind_b = get_kind(&b.cards);

    dbg!(&kind_a);
    dbg!(&kind_b);
    if kind_a == kind_b  {
        return zip(a.cards.chars(), b.cards.chars())
            .find_map(|(a,b)| match card_cmp(&a,&b) {
                std::cmp::Ordering::Equal => None,
                b => Some(b)
            }
        ).unwrap();
    }

    return kind_a.partial_cmp(&kind_b).unwrap();
}

fn cmp_card_sets_joker(a: &CardsSet, b: &CardsSet) -> Ordering {
    if a.cards == b.cards {
        return Ordering::Equal;
    }

    let kind_a = get_strongest_kind(&a.cards);
    let kind_b = get_strongest_kind(&b.cards);

    if kind_a == kind_b  {
        return zip(a.cards.chars(), b.cards.chars())
            .find_map(|(a,b)| match joker_card_cmp(&a,&b) {
                std::cmp::Ordering::Equal => None,
                b => Some(b)
            }
        ).unwrap();
    }

    return kind_a.partial_cmp(&kind_b).unwrap();
}

fn solve_a_input(file : &str) -> i32 {
    let lines: Vec<String> = read_lines(file);
    let mut card_sets = parse_input(lines);
    card_sets.sort_by(cmp_card_sets);

    dbg!(&card_sets);
    dbg!(cmp_card_sets(&card_sets[1], &card_sets[2]));
    return card_sets.iter().enumerate().map(|(rank, set)|{
        (rank+1) as i32 * set.value
    }).sum();
}

pub fn solve_b() {
    println!("{}", solve_b_input("src/test_files/problem7/input.txt"));

}

fn solve_b_input(file : &str) -> i32 {
    let lines: Vec<String> = read_lines(file);
    let mut card_sets = parse_input(lines);
    card_sets.sort_by(cmp_card_sets_joker);

    dbg!(cmp_card_sets_joker(&card_sets[1], &card_sets[2]));
    return card_sets.iter().enumerate().map(|(rank, set)|{
        (rank+1) as i32 * set.value
    }).sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_a_example() {
        let ans = solve_a_input("src/test_files/problem7/a_example.txt");
        assert_eq!(ans, 6440);
    }

    #[test]
    fn test_solve_b_example() {
        let ans = solve_b_input("src/test_files/problem7/a_example.txt");
        assert_eq!(ans, 5905);
    }
}
