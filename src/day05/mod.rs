use crate::lib::Solver;

pub struct Day5Solver;

fn translate(c: char) -> char {
	match c {
		'F' => return '0',
		'B' => return '1',
		'L' => return '0',
		'R' => return '1',
		_ => panic!("Something went catastrophically wrong"),
	}
}

impl Solver for Day5Solver {
	fn solve(&self, lines: Vec<String>, part_two: bool) -> String {
		let mut max_value = 0;
		let mut min_value = 1000000;
		let mut seats: Vec<usize> = vec![];
		for line in lines {
			let bin: String = line.chars().map(|c| translate(c)).collect();
			//let row: String = bin.chars().take(7).collect();
			//let col: String = bin.chars().skip(7).take(3).collect();

			//let row_nr = isize::from_str_radix(&row, 2).unwrap();
			//let col_nr = isize::from_str_radix(&col, 2).unwrap();
			let val = usize::from_str_radix(&bin, 2).unwrap();
			seats.push(val);
			println!("{}", val);
			if val > max_value {
				max_value = val;
			} else if val < min_value {
				min_value = val;
			}
		}
		let mut out = "".to_string();
		if !part_two {
			out = max_value.to_string();
		} else {
			for n in min_value..=max_value {
				if !seats.contains(&n) {
					out = n.to_string();
				}
			}
		}
		out
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn part_one_test_cases() {
		let input: Vec<String> = vec![
			"BFFFBBFRRR".to_string(),
			"FFFBBBFRRR".to_string(),
			"BBFFBBFRLL".to_string(),
		];
		let solver: Day5Solver = Day5Solver {};

		assert_eq!(solver.solve(input, false), "820");
	}
}
