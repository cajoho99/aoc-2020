use crate::lib::Solver;

pub struct Day1Solver;

impl Solver for Day1Solver {
	fn solve(&self, lines: &[String], part_two: bool) -> String {
		let numbers: Vec<i32> = lines.iter().map(|line| line.parse().unwrap()).collect();
		let mut product = 0;
		if !part_two {
			for num_one in numbers.iter() {
				for num_two in numbers.iter() {
					if num_one + num_two == 2020 {
						product = num_one * num_two;
						break;
					}
				}
			}
		} else {
			for num_one in numbers.iter() {
				for num_two in numbers.iter() {
					for num_three in numbers.iter() {
						if num_one + num_two + num_three == 2020 {
							product = num_one * num_two * num_three;
							break;
						}
					}
				}
			}
		};
		product.to_string()
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
			"1721".to_string(),
			"979".to_string(),
			"366".to_string(),
			"299".to_string(),
			"675".to_string(),
			"1456".to_string(),
		];
		let solver: Day1Solver = Day1Solver {};

		assert_eq!(solver.solve(&input, false), "514579");
	}

	#[test]
	fn part_two_test_cases() {
		let input: Vec<String> = vec![
			"1721".to_string(),
			"979".to_string(),
			"366".to_string(),
			"299".to_string(),
			"675".to_string(),
			"1456".to_string(),
		];
		let solver: Day1Solver = Day1Solver {};

		assert_eq!(solver.solve(&input, true), "241861950");
	}

	#[bench]
	fn bench_part_one(bencher: &mut Bencher) {
		let input = read_lines("src/day01/input.txt");
		let solver = Day1Solver {};
		bencher.iter(|| solver.solve(&input, false));
	}

	#[bench]
	fn bench_part_two(bencher: &mut Bencher) {
		let input = read_lines("src/day01/input.txt");
		let solver = Day1Solver {};
		bencher.iter(|| solver.solve(&input, true));
	}
}
