const INPUT: &str = include_str!("input.txt");

pub fn problem1() -> i32 {
    let mut sum = 0;

    for line in INPUT.lines() {
        let first = line.chars().find(|chr| chr.is_numeric()).unwrap_or('0') as u8 & 0b1111;
        let last = line
            .chars()
            .rev()
            .find(|chr| chr.is_numeric())
            .unwrap_or('0') as u8
            & 0b1111;

        sum += first as i32 * 10 + last as i32;
    }

    sum
}

pub fn problem2() -> i32 {
    let mut sum = 0;

    for line in INPUT.lines() {
        let chrs = line.as_bytes();

        let first = find_first(&chrs);
        let last = find_last(&chrs);

        sum += first as i32 * 10 + last as i32;
    }

    sum
}

const NUMS: [&[u8]; 9] = [
    "one".as_bytes(),
    "two".as_bytes(),
    "three".as_bytes(),
    "four".as_bytes(),
    "five".as_bytes(),
    "six".as_bytes(),
    "seven".as_bytes(),
    "eight".as_bytes(),
    "nine".as_bytes(),
];

fn find_first(chrs: &[u8]) -> u8 {
    for i in 0..chrs.len() {
        if chrs[i] & 0b11110000 == 0b110000 {
            return chrs[i] as u8 & 0b1111;
        } else if let Some((_, num)) = NUMS
            .iter()
            .zip(1..)
            .find(|(num, _)| i + num.len() <= chrs.len() && chrs[i..(i + num.len())] == ***num)
        {
            return num;
        }
    }

    return 0;
}

fn find_last(chrs: &[u8]) -> u8 {
    for i in (0..chrs.len()).rev() {
        if chrs[i] & 0b11110000 == 0b110000 {
            return chrs[i] as u8 & 0b1111;
        } else if let Some((_, num)) = NUMS
            .iter()
            .zip(1..)
            .find(|(num, _)| i + num.len() <= chrs.len() && chrs[i..(i + num.len())] == ***num)
        {
            return num;
        }
    }

    return 0;
}
