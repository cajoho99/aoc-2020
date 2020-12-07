use crate::lib::Solver;

pub struct Day3Solver;

impl Solver for Day3Solver {
	fn solve(&self, lines: &[String], part_two: bool) -> String {
		let length = lines.len();
		let width = lines[0].len();
		let mut pairs: Vec<(usize, usize)> = vec![(1, 3)];
		let mut counter: Vec<usize> = vec![0];

		if part_two {
			pairs = vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];
			counter = vec![0, 0, 0, 0, 0];
		}
		for (i, pair) in pairs.iter().enumerate() {
			let mut rightleft: usize = 0;
			let mut vert: usize = 0;
			while vert < length {
				if lines[vert].as_bytes()[rightleft] == '#' as u8 {
					counter[i] += 1;
				}

				vert += pair.0;
				rightleft += pair.1;
				rightleft = rightleft % width;
			}
		}
		let mut prod = 1;
		counter.iter().for_each(|x| prod *= x);
		prod.to_string()
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
			"..##.......".to_string(),
			"#...#...#..".to_string(),
			".#....#..#.".to_string(),
			"..#.#...#.#".to_string(),
			".#...##..#.".to_string(),
			"..#.##.....".to_string(),
			".#.#.#....#".to_string(),
			".#........#".to_string(),
			"#.##...#...".to_string(),
			"#...##....#".to_string(),
			".#..#...#.#".to_string(),
		];
		let solver: Day3Solver = Day3Solver {};

		assert_eq!(solver.solve(&input, false), "7");
	}

	#[test]
	fn part_two_test_cases() {
		let input: Vec<String> = vec![
			"..##.......".to_string(),
			"#...#...#..".to_string(),
			".#....#..#.".to_string(),
			"..#.#...#.#".to_string(),
			".#...##..#.".to_string(),
			"..#.##.....".to_string(),
			".#.#.#....#".to_string(),
			".#........#".to_string(),
			"#.##...#...".to_string(),
			"#...##....#".to_string(),
			".#..#...#.#".to_string(),
		];
		let solver: Day3Solver = Day3Solver {};

		assert_eq!(solver.solve(&input, true), "336");
	}

	#[bench]
	fn bench_part_one(bencher: &mut Bencher) {
		let input = read_lines("src/day03/input.txt");
		let solver = Day3Solver {};
		bencher.iter(|| solver.solve(&input, false));
	}

	#[bench]
	fn bench_part_two(bencher: &mut Bencher) {
		let input = read_lines("src/day03/input.txt");
		let solver = Day3Solver {};
		bencher.iter(|| solver.solve(&input, true));
	}
}
