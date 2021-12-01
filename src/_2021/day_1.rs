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
		let lines = input.lines().map(|l| l.parse::<i32>().unwrap()).collect::<Vec<i32>>();
		let mut increase = 0;
		let mut prev_line = i32::MAX;
		let mut i = 0;
		while i < lines.len() - 2 {
			let val = lines[i..i + 3].iter().sum::<i32>();

			if val > prev_line {
				increase += 1;
			}
			prev_line = val;
			
			i += 1;
		}

		increase.to_string()
	}
}
