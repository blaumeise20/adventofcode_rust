use crate::Day;
pub fn new() -> Day2 { Day2 }
pub struct Day2;

impl Day for Day2 {
	fn part_1(&mut self, input: &str) -> String {
		let mut horizontal = 0;
		let mut depth = 0;
		for l in input.lines() {
			let mut parts = l.split(" ");
			let action = parts.next().unwrap();
			let value = parts.next().unwrap().parse::<u32>().unwrap();
			match action {
				"forward" => horizontal += value,
				"down" => depth += value,
				"up" => depth -= value,
				_ => panic!("Unknown instruction: {}", action),
			}
		}

		(horizontal * depth).to_string()
	}

	fn part_2(&mut self, input: &str) -> String {
		let mut horizontal = 0;
		let mut depth = 0;
		let mut aim = 0;
		for l in input.lines() {
			let mut parts = l.split(" ");
			let action = parts.next().unwrap();
			let value = parts.next().unwrap().parse::<u32>().unwrap();
			match action {
				"forward" => { horizontal += value; depth += aim * value; },
				"down" => aim += value,
				"up" => aim -= value,
				_ => panic!("Unknown instruction: {}", action),
			}
		}

		(horizontal * depth).to_string()
	}
}
