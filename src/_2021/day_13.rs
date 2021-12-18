use std::collections::HashMap;

use crate::Day;
pub fn new() -> Day13 { Day13 }
pub struct Day13;

impl Day for Day13 {
	fn part_1(&mut self, input: &str) -> String {
		solve(input, true).iter()
			.filter(|&(_, &x)| x)
			.count().to_string()
	}

	fn part_2(&mut self, input: &str) -> String {
		let matrix = solve(input, false);
		for y in 0..6 {
			for x in 0..50 {
				if let Some(true) = matrix.get(&(x, y)) { print!("##"); }
				else { print!("  "); }
			}
			println!();
		}
		"".to_string()
	}
}

fn solve(input: &str, break_after_first: bool) -> HashMap<(usize, usize), bool> {
	let mut parts = input.split("\n\n");
	let mut matrix = HashMap::new();
	for mut line in parts.next().unwrap()
		.lines().map(|l| l.split(','))
		{ matrix.insert((line.next().unwrap().parse().unwrap(), line.next().unwrap().parse().unwrap()), true); }

	for l in parts.next().unwrap().lines() {
		let mut l = l.split('=');
		let axis_is_x = l.next().unwrap().ends_with('x');
		let value = l.next().unwrap().parse().unwrap();

		if axis_is_x {
			for x in value..2000 {
				for y in 0..2000 {
					let old = matrix.get_mut(&(x, y));
					let mat = match old { Some(d) => { let v = *d; *d = false; v }, None => false};
					let new_x = value as i64 - (x as i64 - value as i64);
					if new_x >= 0 {
						let new_x = new_x as usize;
						matrix.insert((new_x, y), mat || *matrix.get(&(new_x, y)).unwrap_or(&false));
					}
				}
			}
		}
		else {
			for y in value..2000 {
				for x in 0..2000 {
					let old = matrix.get_mut(&(x, y));
					let mat = match old { Some(d) => { let v = *d; *d = false; v }, None => false};
					let new_y = value as i64 - (y as i64 - value as i64);
					if new_y >= 0 {
						let new_y = new_y as usize;
						matrix.insert((x, new_y), mat || *matrix.get(&(x, new_y)).unwrap_or(&false));
					}
				}
			}
		}

		if break_after_first { break; }
	}

	matrix
}
