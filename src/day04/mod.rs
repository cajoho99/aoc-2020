extern crate regex;

use crate::lib::Solver;
use regex::Regex;

pub struct Day4Solver;

pub struct Passport {
	byr: String,
	iyr: String,
	eyr: String,
	hgt: String,
	hcl: String,
	ecl: String,
	pid: String,
	cid: String,
}

pub trait PassportActions {
	fn valid(&self, part_two: bool) -> bool;
}

impl PassportActions for Passport {
	fn valid(&self, part_two: bool) -> bool {
		let mut valid_vec: Vec<bool> = vec![false, false, false, false, false, false, false];
		if !part_two {
			if !String::is_empty(&self.byr) {
				valid_vec[0] = true;
			}
			if !String::is_empty(&self.iyr) {
				valid_vec[1] = true;
			}
			if !String::is_empty(&self.eyr) {
				valid_vec[2] = true;
			}
			if !String::is_empty(&self.hgt) {
				valid_vec[3] = true;
			}
			if !String::is_empty(&self.hcl) {
				valid_vec[4] = true;
			}
			if !String::is_empty(&self.ecl) {
				valid_vec[5] = true;
			}
			if !String::is_empty(&self.pid) {
				valid_vec[6] = true;
			}
		} else {
			if !String::is_empty(&self.byr) {
				let byr_value: u16 = self.byr.parse().expect("BYR failed parsing");
				if byr_value >= 1920 && byr_value <= 2002 {
					valid_vec[0] = true;
				}
			}
			if !String::is_empty(&self.iyr) {
				let iyr_value: u16 = self.iyr.parse().expect("IYR failed parsing");
				if iyr_value >= 2010 && iyr_value <= 2020 {
					valid_vec[1] = true;
				}
			}
			if !String::is_empty(&self.eyr) {
				let eyr_value: u16 = self.eyr.parse().expect("EYR failed parsing");
				if eyr_value >= 2020 && eyr_value <= 2030 {
					valid_vec[2] = true;
				}
			}
			if !String::is_empty(&self.hgt) {
				if let Some(stripped) = self.hgt.strip_suffix("in") {
					let height: u16 = stripped.parse().expect("HGT failed parsning");
					if height >= 59 && height <= 76 {
						valid_vec[3] = true;
					}
				} else if let Some(stripped) = self.hgt.strip_suffix("cm") {
					let height: u16 = stripped.parse().expect("HGT failed parsning");
					if height >= 150 && height <= 193 {
						valid_vec[3] = true;
					}
				}
			}
			if !String::is_empty(&self.hcl) {
				let re = Regex::new(r"#[a-f,0-9]{6}$").unwrap();
				if re.is_match(&self.hcl) {
					valid_vec[4] = true;
				}
			}
			if !String::is_empty(&self.ecl) {
				let valid_strings = vec![
					"amb".to_string(),
					"blu".to_string(),
					"brn".to_string(),
					"gry".to_string(),
					"grn".to_string(),
					"hzl".to_string(),
					"oth".to_string(),
				];
				if valid_strings.contains(&self.ecl) {
					valid_vec[5] = true;
				}
			}
			if !String::is_empty(&self.pid) {
				let re = Regex::new(r"[0-9]{9}$").unwrap();
				if re.is_match(&self.pid) {
					valid_vec[6] = true;
				}
			}
		}
		println!("Cool guy {:#?}", valid_vec);
		!valid_vec.iter().any(|&v| v == false)
	}
}

fn parse_pass(input: &String) -> Passport {
	let details = input.split(" ");
	let mut byr: String = "".to_string();
	let mut iyr: String = "".to_string();
	let mut eyr: String = "".to_string();
	let mut hgt: String = "".to_string();
	let mut hcl: String = "".to_string();
	let mut ecl: String = "".to_string();
	let mut pid: String = "".to_string();
	let mut cid: String = "".to_string();
	for detail in details {
		let vector: Vec<&str> = detail.split(":").collect();
		match vector[0] {
			"byr" => byr = vector[1].to_string(),
			"iyr" => iyr = vector[1].to_string(),
			"eyr" => eyr = vector[1].to_string(),
			"hgt" => hgt = vector[1].to_string(),
			"hcl" => hcl = vector[1].to_string(),
			"ecl" => ecl = vector[1].to_string(),
			"pid" => pid = vector[1].to_string(),
			"cid" => cid = vector[1].to_string(),
			_ => panic!("Hellowuw"),
		}
	}

	Passport {
		byr,
		iyr,
		eyr,
		hgt,
		hcl,
		ecl,
		pid,
		cid,
	}
}

impl Solver for Day4Solver {
	fn solve(&self, lines: Vec<String>, part_two: bool) -> String {
		let mut passports: Vec<String> = vec![];
		let mut current: Vec<String> = vec![];
		let mut counter: usize = 0;
		for line in lines {
			if line == "" {
				passports.append(&mut vec![current.join(" ")]);
				current = vec![];
				continue;
			} else {
				current.append(&mut vec![line]);
			}
		}
		if current.len() != 0 {
			passports.append(&mut vec![current.join(" ")]);
		}

		print!("{:#?}", passports);
		for passport in passports {
			let pass: Passport = parse_pass(&passport);
			println!("{:#?}", passport);
			if pass.valid(part_two) {
				counter += 1;
			}
		}
		counter.to_string()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn part_one_test_cases() {
		let input: Vec<String> = vec![
			"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd".to_string(),
			"byr:1937 iyr:2017 cid:147 hgt:183cm".to_string(),
			"".to_string(),
			"iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884".to_string(),
			"hcl:#cfa07d byr:1929".to_string(),
			"".to_string(),
			"hcl:#ae17e1 iyr:2013".to_string(),
			"eyr:2024".to_string(),
			"ecl:brn pid:760753108 byr:1931".to_string(),
			"hgt:179cm".to_string(),
			"".to_string(),
			"hcl:#cfa07d eyr:2025 pid:166559648".to_string(),
			"iyr:2011 ecl:brn hgt:59in".to_string(),
		];
		let solver: Day4Solver = Day4Solver {};

		assert_eq!(solver.solve(input, false), "2");
	}

	#[test]
	fn part_two_test_cases() {
		let invalid_input: Vec<String> = vec![
			"eyr:1972 cid:100".to_string(),
			"hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926".to_string(),
			"".to_string(),
			"iyr:2019".to_string(),
			"hcl:#602927 eyr:1967 hgt:170cm".to_string(),
			"ecl:grn pid:012533040 byr:1946".to_string(),
			"".to_string(),
			"hcl:dab227 iyr:2012".to_string(),
			"ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277".to_string(),
			"".to_string(),
			"hgt:59cm ecl:zzz".to_string(),
			"eyr:2038 hcl:74454a iyr:2023".to_string(),
			"pid:3556412378 byr:2007".to_string(),
		];
		let valid_input: Vec<String> = vec![
			"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980".to_string(),
			"hcl:#623a2f".to_string(),
			"".to_string(),
			"eyr:2029 ecl:blu cid:129 byr:1989".to_string(),
			"iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm".to_string(),
			"".to_string(),
			"hcl:#888785".to_string(),
			"hgt:164cm byr:2001 iyr:2015 cid:88".to_string(),
			"pid:545766238 ecl:hzl".to_string(),
			"eyr:2022".to_string(),
			"".to_string(),
			"iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719".to_string(),
		];
		let solver: Day4Solver = Day4Solver {};

		assert_eq!(solver.solve(invalid_input, true), "0");
		assert_eq!(solver.solve(valid_input, true), "4");
	}
}
