const INPUT: &str = include_str!("input.txt");

pub fn problem1() -> i32 {
    let mut sum = 0;

    for line in INPUT.lines() {
        let (_, nums) = line.split_once(':').unwrap();
        let (winning, have) = nums.split_once('|').unwrap();

        sum += (1
            << winning
                .as_bytes()
                .chunks_exact(3)
                .filter(|num| have.as_bytes().chunks_exact(3).any(|other| other == *num))
                .count())
            >> 1;
    }

    sum
}

pub fn problem2() -> i32 {
    let mut lines: Vec<(&str, usize)> = INPUT.lines().map(|a| (a, 1)).collect();
    let mut sum = 0;

    for i in 0..lines.len() {
        let (_, nums) = lines[i].0.split_once(':').unwrap();
        let (winning, have) = nums.split_once('|').unwrap();

        let matches = winning
            .as_bytes()
            .chunks_exact(3)
            .filter(|num| have.as_bytes().chunks_exact(3).any(|other| other == *num))
            .count();

        sum += lines[i].1;

        if matches > 0 {
            for j in 1..=matches {
                lines[i + j].1 += lines[i].1;
            }
        }
    }

    sum as i32
}
