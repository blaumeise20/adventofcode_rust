use crate::Day;

pub fn new() -> Day1 { Day1 }

pub struct Day1;
impl Day for Day1 {
	fn part_1(&mut self, input: &str) -> String {
		let mut lines = input.lines().map(|l| l.parse::<i32>().unwrap());
		let mut increase = 0;
		let mut prev_line = lines.next().unwrap();

		for line in lines {
			if line > prev_line {
				increase += 1;
			}
			prev_line = line;
		}

		increase.to_string()
	}

	fn part_2(&mut self, input: &str) -> String {
		let lines = input.lines().map(|l| l.parse().unwrap()).collect::<Vec<i32>>();
		let mut increase = 0;
		let mut prev_line = i32::MAX;

		for i in 0..lines.len() - 2 {
			let val = lines[i] + lines[i + 1] + lines[i + 2];

			if val > prev_line {
				increase += 1;
			}
			prev_line = val;
		}

		increase.to_string()
	}
}
