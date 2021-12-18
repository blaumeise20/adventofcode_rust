use crate::Day;
pub fn new() -> Day10 { Day10 }
pub struct Day10;

impl Day for Day10 {
	fn part_1(&mut self, input: &str) -> String {
		collect_scores(input, false)
			.iter().sum::<usize>().to_string()
	}

	fn part_2(&mut self, input: &str) -> String {
		let mut scores = collect_scores(input, true);
		scores.sort();
		scores[scores.len() / 2].to_string()
	}
}

fn collect_scores(input: &str, incomplete_lines: bool) -> Vec<usize> {
	input.lines()
		.filter_map(|l| match parse_chunk(l.chars().collect(), incomplete_lines) {
			0 => None,
			c => Some(c)
		}).collect()
}

fn parse_chunk(data: Vec<char>, incomplete: bool) -> usize {
	let mut stack = vec![];
	let mut err_count = 0;
	for ch in data {
		match ch {
			'(' => stack.push(')'),
			'[' => stack.push(']'),
			'{' => stack.push('}'),
			'<' => stack.push('>'),
			_ => {
				if let Some(st) = stack.pop() {
					if st != ch {
						err_count += match ch {
							')' => 3,
							']' => 57,
							'}' => 1197,
							'>' => 25137,
							_ => unreachable!()
						};
					}
				}
			}
		}
	}

	if incomplete {
		if err_count != 0 { 0 }
		else { 
			stack.iter().rev()
				.fold(0, |a, &b| 
					a * 5 + [')', ']', '}', '>'].iter()
						.position(|&x| x == b).unwrap()
				)
		}
	}
	else {
		err_count
	}
}
