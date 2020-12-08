use crate::lib::Solver;

use std::collections::{HashMap, HashSet};

pub struct Day7Solver;

impl Solver for Day7Solver {
	fn solve(&self, lines: &[String], part_two: bool) -> String {
		let mut count = 0;
		if !part_two {
			let mut ctp_map = HashMap::new();
			
			for line in lines {
				let split:Vec<&str> = line.split("bags contain").collect();
				let from = split[0].trim();
				//println!("from {}", from);
				let to:Vec<&str> = split[1].split(",").collect();
								
				for t in to {
					let t_split: Vec<&str> = t.trim().split(" ").collect();
					let out;
					if t_split[0].trim() != "no" {
						let bag_count = t_split.iter().cloned().next().unwrap();
						let bag_name: String = t_split.iter().cloned().skip(1).take(t_split.len() - 2).collect::<Vec<&str>>().join(" ");
						//println!("name: {}, amount: {}", bag_name, bag_count);
						out = ((*bag_count).parse::<usize>().unwrap(), bag_name.replace(".", ""));

						let entry = ctp_map.entry(out.1.clone()).or_insert_with(Vec::new);
						entry.push(from);
					}
					
				}
				//println!{"{:#?}", ctp_map};

			}
			count = find_bags(&ctp_map, "shiny gold").len();
		}
		else {
			let mut ptc_map = HashMap::new();
			for line in lines {
				let split:Vec<&str> = line.split("bags contain").collect();
				let from = split[0].trim();
				//println!("from {}", from);
				let to:Vec<&str> = split[1].split(",").collect();
								
				 let childs: Vec<(usize, String)> = to.iter().map(|t| {
					let t_split: Vec<&str> = t.trim().split(" ").collect();
					let out;
					if t_split[0].trim() != "no" {
						//println!("{:#?}", t_split);
						let bag_count = t_split.iter().cloned().next().unwrap();
						let bag_name: String = t_split.iter().cloned().skip(1).take(t_split.len() - 2).collect::<Vec<&str>>().join(" ");
						//println!("name: {}, amount: {}", bag_name, bag_count);
						out = Some(((*bag_count).parse::<usize>().unwrap(), bag_name.replace(".", "")));
					}
					else {
						out = None;
					}
					match out {
						Some(v) => return v,
						None => return (0, "".to_string()),
					}
				}).filter(|(_, name)| !name.is_empty()).collect();
				
				ptc_map.insert(from, childs);
				//println!{"{:#?}", ptc_map};
			}
			count = count_bags(&ptc_map, "shiny gold");
		}
		count.to_string()
	}
}

fn find_bags<'a>(ctp_map:&'a HashMap<String, Vec<&str>>, bag:&'a str) -> HashSet<&'a str>{
	let mut res = HashSet::new();
	find_bags_r(ctp_map, bag, &mut res);
	res
}

fn find_bags_r<'a>(ctp_map:&'a HashMap<String, Vec<&'a str>>, bag:&str, res: &mut HashSet<&'a str>) {
	if let Some(wrapping_bags) = ctp_map.get(bag) {
		for wrapping_bag in wrapping_bags.iter() {
			res.insert(wrapping_bag);
			find_bags_r(ctp_map, wrapping_bag, res);
		}
	}
}

fn count_bags(ptc_map: &HashMap<&str, Vec<(usize, String)>>, bag:&str) -> usize {
	if let Some(bags) = ptc_map.get(bag) {
		bags.iter().map(|(count, name)| {count * (1 + count_bags(ptc_map, name))}).sum()
	}
	else {
		0
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
			"light red bags contain 1 bright white bag, 2 muted yellow bags.".to_string(),
			"dark orange bags contain 3 bright white bags, 4 muted yellow bags.".to_string(),
			"bright white bags contain 1 shiny gold bag.".to_string(),
			"muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.".to_string(),
			"shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.".to_string(),
			"dark olive bags contain 3 faded blue bags, 4 dotted black bags.".to_string(),
			"vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.".to_string(),
			"faded blue bags contain no other bags.".to_string(),
			"dotted black bags contain no other bags.".to_string(),
		];
		let solver: Day7Solver = Day7Solver {};

		assert_eq!(solver.solve(&input, false), "4");
	}

	#[test]
	fn part_two_test_cases() {
		let input: Vec<String> = vec![
			"shiny gold bags contain 2 dark red bags.".to_string(),
			"dark red bags contain 2 dark orange bags.".to_string(),
			"dark orange bags contain 2 dark yellow bags.".to_string(),
			"dark yellow bags contain 2 dark green bags.".to_string(),
			"dark green bags contain 2 dark blue bags.".to_string(),
			"dark blue bags contain 2 dark violet bags.".to_string(),
			"dark violet bags contain no other bags.".to_string(),
		];
		let solver: Day7Solver = Day7Solver {};

		assert_eq!(solver.solve(&input, true), "126");
	}

	//#[bench]
	fn bench_part_one(bencher: &mut Bencher) {
		let input = read_lines("src/day01/input.txt");
		let solver = Day7Solver {};
		bencher.iter(|| solver.solve(&input, false));
	}

	#[bench]
	fn bench_part_two(bencher: &mut Bencher) {
		let input = read_lines("src/day01/input.txt");
		let solver = Day7Solver {};
		bencher.iter(|| solver.solve(&input, true));
	}
}
