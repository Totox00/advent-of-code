use std::cmp::Ordering;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    High,
    Pair,
    TwoPair,
    Three,
    House,
    Four,
    Five,
}

pub fn problem1() -> i32 {
    let mut bids: Vec<(Vec<u8>, i32)> = INPUT
        .lines()
        .map(|line| {
            let chrs = line.as_bytes();
            (
                chrs[0..5]
                    .iter()
                    .map(|chr| match chr {
                        b'A' => 83,
                        b'K' => 82,
                        b'T' => 58,
                        _ => *chr,
                    })
                    .collect(),
                chrs[6..]
                    .iter()
                    .map(|chr| (chr & 0b1111) as i32)
                    .reduce(|acc, x| acc * 10 + x)
                    .unwrap(),
            )
        })
        .collect();

    bids.sort_unstable_by(|(a, _), (b, _)| {
        let ord = get_type(a).cmp(&get_type(b));
        if ord == Ordering::Equal {
            a.cmp(b)
        } else {
            ord
        }
    });

    bids.iter()
        .zip(1..)
        .map(|((_, bid), rank)| bid * rank)
        .sum()
}

pub fn problem2() -> i32 {
    let mut bids: Vec<(Vec<u8>, i32)> = INPUT
        .lines()
        .map(|line| {
            let chrs = line.as_bytes();
            (
                chrs[0..5]
                    .iter()
                    .map(|chr| match chr {
                        b'A' => 83,
                        b'K' => 82,
                        b'T' => 58,
                        b'J' => 1,
                        _ => *chr,
                    })
                    .collect(),
                chrs[6..]
                    .iter()
                    .map(|chr| (chr & 0b1111) as i32)
                    .reduce(|acc, x| acc * 10 + x)
                    .unwrap(),
            )
        })
        .collect();

    bids.sort_unstable_by(|(a, _), (b, _)| {
        let ord = get_type_jokers(a).cmp(&get_type_jokers(b));
        if ord == Ordering::Equal {
            a.cmp(b)
        } else {
            ord
        }
    });

    bids.iter()
        .zip(1..)
        .map(|((_, bid), rank)| bid * rank)
        .sum()
}

fn get_type(hand: &[u8]) -> HandType {
    let mut distinct: Vec<(u8, u8)> = vec![];

    for card in hand {
        if let Some(i) = distinct.iter().position(|(c, _)| c == card) {
            distinct[i].1 += 1;
        } else {
            distinct.push((*card, 1));
        }
    }

    match distinct.len() {
        1 => HandType::Five,
        2 if distinct[0].1 == 1 || distinct[0].1 == 4 => HandType::Four,
        2 => HandType::House,
        3 if distinct.iter().any(|(_, count)| *count == 3) => HandType::Three,
        3 => HandType::TwoPair,
        4 => HandType::Pair,
        _ => HandType::High,
    }
}

fn get_type_jokers(hand: &[u8]) -> HandType {
    let mut distinct: Vec<(u8, u8)> = vec![];
    let mut jokers = 0;

    for card in hand {
        if *card == 1 {
            jokers += 1;
        } else if let Some(i) = distinct.iter().position(|(c, _)| c == card) {
            distinct[i].1 += 1;
        } else {
            distinct.push((*card, 1));
        }
    }

    match distinct.len() {
        0 | 1 => HandType::Five,
        2 if distinct[0].1 + jokers == 4 || distinct[1].1 + jokers == 4 => HandType::Four,
        2 => HandType::House,
        3 if distinct.iter().any(|(_, count)| *count + jokers == 3) => HandType::Three,
        3 => HandType::TwoPair,
        4 => HandType::Pair,
        _ => HandType::High,
    }
}
