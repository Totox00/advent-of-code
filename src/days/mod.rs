mod d1;

pub fn problem(problem: Problem) {
    println!(
        "{}",
        match problem {
            Problem::Day1Problem1 => d1::problem1(),
            Problem::Day1Problem2 => d1::problem2(),
        }
    )
}

pub enum Problem {
    Day1Problem1,
    Day1Problem2,
}
