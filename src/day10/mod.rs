use crate::lib::Solver;

pub struct Day10Solver;

impl Solver for Day10Solver {
	fn solve(&self, lines: &[String], part_two: bool) -> String {
        if !part_two {
            let ones: i64 = 0;
            let threes: i64 = 0;
            let values: Vec<i64> = lines.iter().map(|val| val.parse().unwrap());
            let sorted = values.sort_unstable();
            let mut prev = values[0];
            for adapter in values {
                match adapter - prev {
                    1 => ones += 1,
                    3 => threes += 1,
                    _ => (),
                }
                prev = adapter;
            }
            ones * threes
        }
        else {
            unimplemented!()
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
		let input = read_lines("src/day10/input.test1.txt");
		let solver: Day10Solver = Day10Solver {};
		assert_eq!(solver.solve(&input, false), "");
    }
    
    #[test]
	fn part_one_test_long() {
		let input = read_lines("src/day10/input.test_long.txt");
		let solver: Day10Solver = Day10Solver {};
		assert_eq!(solver.solve(&input, false), "");
	}

	#[test]
	fn part_two_test_cases() {
        let input = read_lines("src/day10/input.test1.txt");
		let solver: Day10Solver = Day10Solver {};
		assert_eq!(solver.solve(&input, true), "");
	}

	#[bench]
	fn bench_part_one(bencher: &mut Bencher) {
		let input = read_lines("src/day10/input.txt");
		let solver = Day10Solver {};
		bencher.iter(|| solver.solve(&input, false));
	}

	#[bench]
	fn bench_part_two(bencher: &mut Bencher) {
		let input = read_lines("src/day10/input.txt");
		let solver = Day10Solver {};
		bencher.iter(|| solver.solve(&input, true));
	}
}