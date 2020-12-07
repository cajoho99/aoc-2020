use crate::lib::Solver;
use std::collections::HashSet;

pub struct Day6Solver;

impl Solver for Day6Solver {
	fn solve(&self, lines: &[String], part_two: bool) -> String {
		if !part_two {
			let mut score = 0;
			let mut current_answers = Vec::new();
			for line in lines.iter() {
				if line.is_empty() {
					score += current_answers.len();
					current_answers = Vec::new();
					continue;
				}
				for c in line.as_bytes().iter() {
					if !current_answers.contains(c) {
						current_answers.push(*c);
					}
				}
			}
			score += current_answers.len();
			score.to_string()
		} else {
			let mut score = 0;
			let mut groups: Vec<Vec<&str>> = Vec::new();
			let mut current: Vec<&str> = Vec::new();
			for line in lines {
				if line.is_empty() {
					groups.push(current.clone());
					current.clear();
				} else {
					current.push(line);
				}
			}
			groups.push(current.clone());
			for group in groups {
				let len = group.len();
				let mut set = HashSet::new();
				let mut all: Vec<&u8> = Vec::new();
				println!("Group {:?}", group);
				print!("Score {}", score);
				for p in group {
					for c in p.as_bytes().iter() {
						set.insert(c);
						all.push(c);
					}
				}
				for elem in set.iter() {
					if all
						.iter()
						.filter(|&&c| *c == **elem)
						.collect::<Vec<_>>()
						.len() == len
					{
						score += 1;
					}
				}
				print!("-> {}\n", score);
			}
			score.to_string()
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::lib::read_lines;
	use test::Bencher;

	#[test]
	fn part_one_test_cases() {
		let input: Vec<String> = vec![
			"abs".to_string(),
			"".to_string(),
			"a".to_string(),
			"b".to_string(),
			"c".to_string(),
			"".to_string(),
			"ab".to_string(),
			"ac".to_string(),
			"".to_string(),
			"a".to_string(),
			"a".to_string(),
			"a".to_string(),
			"a".to_string(),
			"".to_string(),
			"b".to_string(),
		];
		let solver: Day6Solver = Day6Solver {};

		assert_eq!(solver.solve(&input, false), "11");
	}

	#[test]
	fn part_two_test_cases() {
		let input: Vec<String> = vec![
			"abs".to_string(),
			"".to_string(),
			"a".to_string(),
			"b".to_string(),
			"c".to_string(),
			"".to_string(),
			"ab".to_string(),
			"ac".to_string(),
			"".to_string(),
			"a".to_string(),
			"a".to_string(),
			"a".to_string(),
			"a".to_string(),
			"".to_string(),
			"b".to_string(),
		];
		let solver: Day6Solver = Day6Solver {};

		assert_eq!(solver.solve(&input, true), "6");
	}

	#[bench]
	fn bench_part_one(bencher: &mut Bencher) {
		let input = read_lines("src/day06/input.txt");
		let solver = Day6Solver {};
		bencher.iter(|| solver.solve(&input, false));
	}

	#[bench]
	fn bench_part_two(bencher: &mut Bencher) {
		let input = read_lines("src/day06/input.txt");
		let solver = Day6Solver {};
		bencher.iter(|| solver.solve(&input, true));
	}
}
