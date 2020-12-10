use crate::lib::Solver;

pub struct Day_Solver;

impl Solver for Day_Solver {
	fn solve(&self, lines: &[String], part_two: bool) -> String {
	
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::lib::read_lines;
	use test::Bencher;

	#[test]
	fn part_one_test_cases() {
		let input = read_lines("src/day_/input.test1.txt");
		let solver: Day_Solver = Day_Solver {};
		assert_eq!(solver.solve(&input, false), "");
	}

	#[test]
	fn part_two_test_cases() {
        let input = read_lines("src/day_/input.test1.txt");
		let solver: Day_Solver = Day_Solver {};
		assert_eq!(solver.solve(&input, true), "");
	}

	#[bench]
	fn bench_part_one(bencher: &mut Bencher) {
		let input = read_lines("src/day_/input.txt");
		let solver = Day_Solver {};
		bencher.iter(|| solver.solve(&input, false));
	}

	#[bench]
	fn bench_part_two(bencher: &mut Bencher) {
		let input = read_lines("src/day_/input.txt");
		let solver = Day_Solver {};
		bencher.iter(|| solver.solve(&input, true));
	}
}
