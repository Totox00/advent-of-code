mod d1;
mod d2;

pub fn problem(problem: Problem) {
    println!(
        "{}",
        match problem {
            Problem::Day1Problem1 => d1::problem1(),
            Problem::Day1Problem2 => d1::problem2(),
            Problem::Day2Problem1 => d2::problem1(),
            Problem::Day2Problem2 => d2::problem2(),
        }
    )
}

pub enum Problem {
    Day1Problem1,
    Day1Problem2,
    Day2Problem1,
    Day2Problem2,
}
