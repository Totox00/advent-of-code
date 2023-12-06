use std::env::args;

use days::{problem, Problem};

mod days;

fn main() {
    if args().len() > 1 {
        problem(Problem::Day1Problem1);
    }
    if args().len() > 1 {
        problem(Problem::Day1Problem2);
    }
    if args().len() > 1 {
        problem(Problem::Day2Problem1);
    }
    if args().len() > 1 {
        problem(Problem::Day2Problem2);
    }
    if args().len() > 1 {
        problem(Problem::Day3Problem1);
    }
    if args().len() > 1 {
        problem(Problem::Day3Problem2);
    }
    if args().len() > 1 {
        problem(Problem::Day4Problem1);
    }
    if args().len() > 1 {
        problem(Problem::Day4Problem2);
    }
    if args().len() > 1 {
        problem(Problem::Day5Problem1);
    }
    if args().len() > 1 {
        problem(Problem::Day5Problem2);
    }
    problem(Problem::Day6Problem1);
    problem(Problem::Day6Problem2);
}
