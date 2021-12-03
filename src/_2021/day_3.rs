use crate::Day;

pub fn new() -> Day3 { Day3 }

pub struct Day3;
impl Day for Day3 {
	fn part_1(&mut self, input: &str) -> String {
		let lines = input.lines().collect::<Vec<_>>();
		let mut gamma = String::new();
		let mut epsilon = String::new();

		for index in 0..lines[0].len() {
			let more_zeros = has_more_zeros(&lines, index);
			gamma  .push(if more_zeros { '0' } else { '1' });
			epsilon.push(if more_zeros { '1' } else { '0' });
		}

		let gamma = usize::from_str_radix(&gamma, 2).unwrap();
		let epsilon = usize::from_str_radix(&epsilon, 2).unwrap();
		(gamma * epsilon).to_string()
	}

	fn part_2(&mut self, input: &str) -> String {
		let val1 = usize::from_str_radix(&get_value(input, false), 2).unwrap();
		let val2 = usize::from_str_radix(&get_value(input, true), 2).unwrap();
		(val1 * val2).to_string()
	}
}

fn get_value(input: &str, least_common: bool) -> &str {
    let mut lines = input.lines().collect::<Vec<_>>();

    for index in 0..lines[0].len() {
		let more_zeros = has_more_zeros(&lines, index);
		lines = lines.iter().map(|&line| line).filter(|&line| {
			let char = line.chars().nth(index).unwrap();
			more_zeros != least_common && char == '0' ||
			more_zeros == least_common && char == '1'
		}).collect();
		if lines.len() == 1 { break; }
	}

    lines[0]
}

fn has_more_zeros(lines: &Vec<&str>, index: usize) -> bool {
	let mut zeros = 0;
	let mut ones = 0;
	for line in lines {
		if line.chars().nth(index).unwrap() == '1' { ones += 1; }
		else { zeros += 1; }
	}
	zeros > ones
}