#![feature(test)]
extern crate test;

use std::env;
use std::time::Instant;

mod day01;
use day01::Day1Solver;

mod day02;
use day02::Day2Solver;

mod day03;
use day03::Day3Solver;

mod day04;
use day04::Day4Solver;

mod day05;
use day05::Day5Solver;

mod day06;
use day06::Day6Solver;

mod day07;
use day07::Day7Solver;

mod day08;
use day08::Day8Solver;

mod day10;
use day10::Day10Solver;

mod lib;
use lib::read_lines;
use lib::Solver;

fn get_solver(day: u32) -> (Box<dyn Solver>, String) {
	let file = format!("src/day{:02}/input.txt", day);
	let solver: Box<dyn Solver> = match day {
		1 => Box::new(Day1Solver {}),
		2 => Box::new(Day2Solver {}),
		3 => Box::new(Day3Solver {}),
		4 => Box::new(Day4Solver {}),
		5 => Box::new(Day5Solver {}),
		6 => Box::new(Day6Solver {}),
		7 => Box::new(Day7Solver {}),
		8 => Box::new(Day8Solver {}),
		10 => Box::new(Day10Solver {}),
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
	let part1_answer = solver.solve(&read_lines(&file), false);
	let part1_time = part1_now.elapsed().as_secs_f32();
	println!("Day {} (part 1) answer: {}", day, part1_answer);
	println!("Day {} (part 1) time: {}", day, part1_time);
	let part2_now = Instant::now();
	let part2_answer = solver.solve(&read_lines(&file), true);
	let part2_time = part2_now.elapsed().as_secs_f32();
	println!("Day {} (part 2) answer: {}", day, part2_answer);
	println!("Day {} (part 2) time: {}", day, part2_time);
	println!("Day {} total time: {}", day, part1_time + part2_time);
}
