use crate::Day;

pub fn new() -> Day3 { Day3 }

pub struct Day3;
impl Day for Day3 {
	fn part_1(&mut self, input: &str) -> String {
		let lines = input.lines().collect::<Vec<_>>();
		let mut gamma = String::new();
		let mut epsilon = String::new();

		for index in 0..lines[0].len() {
			let (zeroes, ones) = count_in_index(&lines, index);
			gamma  .push(if zeroes > ones { '0' } else { '1' });
			epsilon.push(if zeroes > ones { '1' } else { '0' });
		}

		let gamma = usize::from_str_radix(&gamma, 2).unwrap();
		let epsilon = usize::from_str_radix(&epsilon, 2).unwrap();
		(gamma * epsilon).to_string()
	}

	fn part_2(&mut self, input: &str) -> String {
		let val1 = get_value(input, false);
		let val1 = usize::from_str_radix(&val1, 2).unwrap();

		let val2 = get_value(input, true);
		let val2 = usize::from_str_radix(&val2, 2).unwrap();

		(val1 * val2).to_string()
	}
}

fn get_value(input: &str, least_common: bool) -> &str {
    let mut lines = input.lines().collect::<Vec<_>>();

    for index in 0..lines[0].len() {
		let (zeroes, ones) = count_in_index(&lines, index);

		let mut new_lines = Vec::new();
		for line in &lines {
			let char = line.chars().nth(index).unwrap();
			// a != b same as a xor b
			if ((zeroes > ones) != least_common)  && char == '0' ||
			   ((zeroes <= ones) != least_common) && char == '1' {
				new_lines.push(*line);
			}
		}
		lines = new_lines;

		if lines.len() == 1 { break; }
	}

    lines[0]
}

fn count_in_index(lines: &Vec<&str>, index: usize) -> (i32, i32) {
	let mut zeros = 0;
	let mut ones = 0;
	for line in lines {
		if line.chars().nth(index).unwrap() == '1' { ones += 1; }
		else { zeros += 1; }
	}
	(zeros, ones)
}