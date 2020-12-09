use crate::lib::Solver;

pub struct Day8Solver;

impl Solver for Day8Solver  {
	fn solve(&self, lines: &[String], part_two: bool) -> String {
        let mut program = parse_program(lines).clone();
        
        if !part_two {
            let res = run_program(&program);
            let out;
            match res {
                ProgramResult::Success(v) => out = v,
                ProgramResult::Loop(v) => out =  v,
                ProgramResult::Err => panic!("something is very wrong"),
            }
            return out.to_string();
        }
        else {
            for i in 0..program.len() {
                println!("{} - {}", i, program.len());
                program[i] = match program.clone()[i] {
                    Instruction::Jmp(v) => Instruction::Nop(v),
                    Instruction::Nop(v) => Instruction::Jmp(v),
                    n => n,
                };
                //println!("{:#?}", program);
                let res = run_program(&program);
                program[i] = match program.clone()[i] {
                    Instruction::Jmp(v) => Instruction::Nop(v),
                    Instruction::Nop(v) => Instruction::Jmp(v),
                    n => n,
                };

                if let ProgramResult::Success(v) = res {
                    return v.to_string();
                }
            }
        }
        panic!("The program could not terminate!");
    }
}

#[derive(Copy, Clone, Debug)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

enum ProgramResult {
    Success(i32),
    Loop(i32),
    Err,
}

fn parse_program(lines: &[String]) -> Vec<Instruction> {
    lines.iter().map(|line| {
        let split_one: Vec<&str> = line.split(" ").collect();
        let instruction: &str = split_one[0];
        let value : i32 = split_one[1].parse().unwrap();
        match instruction {
            "acc" => Instruction::Acc(value),
            "jmp" => Instruction::Jmp(value),
            "nop" => Instruction::Nop(value),
            _ => panic!("boi watch out"),
        }
    }).collect()
   
}

fn run_program(program: &Vec<Instruction>) -> ProgramResult{
    let mut ip: isize = 0;
    let mut acc = 0;
    let mut processed: Vec<isize> = Vec::new();

    loop {
        if ip as usize == program.len() {
            return ProgramResult::Success(acc);
        }

        if processed.contains(&ip) {
            return ProgramResult::Loop(acc);
        }

        if ip < 0 && ip as usize >= program.len()  {
            return ProgramResult::Err;
        }
        processed.push(ip);

        match program[ip as usize] {
            Instruction::Acc(v) => acc += v,
            Instruction::Jmp(v) => {
                ip += v as isize;
                continue;
            }
            Instruction::Nop(_v) => (),
        }
        ip += 1
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
			"nop +0".to_string(),
			"acc +1".to_string(),
			"jmp +4".to_string(),
			"acc +3".to_string(),
			"jmp -3".to_string(),
			"acc -99".to_string(),
			"acc +1".to_string(),
			"jmp -4".to_string(),
			"acc +6".to_string(),
		];
		let solver: Day8Solver = Day8Solver {};

		assert_eq!(solver.solve(&input, false), "5");
	}

	#[test]
	fn part_two_test_cases() {
		let input: Vec<String> = vec![
			"nop +0".to_string(),
			"acc +1".to_string(),
			"jmp +4".to_string(),
			"acc +3".to_string(),
			"jmp -3".to_string(),
			"acc -99".to_string(),
			"acc +1".to_string(),
			"jmp -4".to_string(),
			"acc +6".to_string(),
		];
		let solver: Day8Solver = Day8Solver {};

		assert_eq!(solver.solve(&input, true), "8");
	}

	#[bench]
	fn bench_part_one(bencher: &mut Bencher) {
		let input = read_lines("src/day08/input.txt");
		let solver = Day8Solver {};
		bencher.iter(|| solver.solve(&input, false));
	}

	#[bench]
	fn bench_part_two(bencher: &mut Bencher) {
		let input = read_lines("src/day08/input.txt");
		let solver = Day8Solver {};
		bencher.iter(|| solver.solve(&input, true));
	}
}
