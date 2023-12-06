mod d1;
mod d2;
mod d3;
mod d4;
mod d5;
mod d6;

pub fn problem(problem: Problem) {
    match problem {
        Problem::Day1Problem1 => println!("{}", d1::problem1()),
        Problem::Day1Problem2 => println!("{}", d1::problem2()),
        Problem::Day2Problem1 => println!("{}", d2::problem1()),
        Problem::Day2Problem2 => println!("{}", d2::problem2()),
        Problem::Day3Problem1 => println!("{}", d3::problem1()),
        Problem::Day3Problem2 => println!("{}", d3::problem2()),
        Problem::Day4Problem1 => println!("{}", d4::problem1()),
        Problem::Day4Problem2 => println!("{}", d4::problem2()),
        Problem::Day5Problem1 => println!("{}", d5::problem1()),
        Problem::Day5Problem2 => println!("{}", d5::problem2()),
        Problem::Day6Problem1 => println!("{}", d6::problem1()),
        Problem::Day6Problem2 => println!("{}", d6::problem2()),
    }
}

pub enum Problem {
    Day1Problem1,
    Day1Problem2,
    Day2Problem1,
    Day2Problem2,
    Day3Problem1,
    Day3Problem2,
    Day4Problem1,
    Day4Problem2,
    Day5Problem1,
    Day5Problem2,
    Day6Problem1,
    Day6Problem2,
}
