use crate::lib::Solver;

pub struct Day2Solver;

impl Solver for Day2Solver {
    fn solve(&self, lines: Vec<String>, part_two: bool) -> String {
        let mut counter = 0;
        for line in lines {
            let first_split: Vec<&str> = line.split(|c| c == ':').collect();
            let password = first_split[1].trim();
            let s_split: Vec<&str> = first_split[0].split(|c| c == ' ').collect();
            let min_max: Vec<&str> = s_split[0].split(|c| c == '-').collect();
            let low: i32 = min_max[0].parse().unwrap();
            let high: i32 = min_max[1].parse().unwrap();
            let rule: u8 = s_split[1].as_bytes()[0];
            if !part_two {
                let matching_chars = password.chars().filter(|c| *c == rule as char).count() as i32;
                if matching_chars >= low && matching_chars <= high {
                    counter += 1;
                }
            } else {
                let first: bool = password.as_bytes()[low as usize - 1] == rule;
                let snd: bool = password.as_bytes()[high as usize - 1] == rule;

                if (first || snd) && !(first && snd) {
                    counter += 1;
                }
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
            "1-3 a: abcde".to_string(),
            "1-3 b: cdefg".to_string(),
            "2-9 c: ccccccccc".to_string(),
        ];
        let solver: Day2Solver = Day2Solver {};

        assert_eq!(solver.solve(input, false), "2");
    }

    #[test]
    fn part_two_test_cases() {
        let input: Vec<String> = vec![
            "1-3 a: abcde".to_string(),
            "1-3 b: cdefg".to_string(),
            "2-9 c: ccccccccc".to_string(),
        ];
        let solver: Day2Solver = Day2Solver {};

        assert_eq!(solver.solve(input, true), "1");
    }
}
