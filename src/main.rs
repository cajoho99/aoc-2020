use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

mod day01;
use day01::Day1Solver;

mod day02;
use day02::Day2Solver;

mod day03;
use day03::Day3Solver;

mod lib;
use lib::Solver;

fn read_lines<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("Failed to open file");
    io::BufReader::new(file)
        .lines()
        .filter_map(io::Result::ok)
        .collect()
}

fn get_solver(day: u32) -> (Box<dyn Solver>, String) {
    let file = format!("src/day{:02}/input.txt", day);
    let solver: Box<dyn Solver> = match day {
        1 => Box::new(Day1Solver {}),
        2 => Box::new(Day2Solver {}),
        3 => Box::new(Day3Solver {}),
        n => panic!("The solver for day {} has not been implemented", n),
    };
    (solver, file)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        panic!("Must input number as first argument");
    }
    let day: u32 = args[1].parse().expect("Not a number");
    let (solver, file) = get_solver(day);
    let part1_now = Instant::now();
    let part1_answer = solver.solve(read_lines(&file), false);
    let part1_time = part1_now.elapsed().as_secs_f32();
    println!("Day {} (part 1) answer: {}", day, part1_answer);
    println!("Day {} (part 1) time: {}", day, part1_time);
    let part2_now = Instant::now();
    let part2_answer = solver.solve(read_lines(&file), true);
    let part2_time = part2_now.elapsed().as_secs_f32();
    println!("Day {} (part 2) answer: {}", day, part2_answer);
    println!("Day {} (part 2) time: {}", day, part2_time);
    println!("Day {} total time: {}", day, part1_time + part2_time);
}
