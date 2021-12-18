use crate::Day;
pub fn new() -> Day7 { Day7 }
pub struct Day7;

impl Day for Day7 {
	fn part_1(&mut self, input: &str) -> String { calc(input, false) }
	fn part_2(&mut self, input: &str) -> String { calc(input, true) }
}

fn calc(input: &str, increase_fuel: bool) -> String {
	let crabs = input.split(',').map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
	let mut min_fuel = std::i64::MAX;

	for f in 0..*crabs.iter().max().unwrap() {
		let mut fuel = 0;
		for c in crabs.iter() {
			let crab_fuel = (f - c).abs();
			if increase_fuel { fuel += crab_fuel * (crab_fuel + 1) / 2 }
			else { fuel += crab_fuel; }
		}
		if fuel < min_fuel { min_fuel = fuel; }
	}

	min_fuel.to_string()
}
