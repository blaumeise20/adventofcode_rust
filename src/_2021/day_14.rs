use std::collections::HashMap;

use indexmap::IndexMap;

use crate::Day;
pub fn new() -> Day14 { Day14 }
pub struct Day14;

impl Day for Day14 {
	fn part_1(&mut self, input: &str) -> String { calc(input, 10) }
	fn part_2(&mut self, input: &str) -> String { calc(input, 40) }
}

fn calc(input: &str, steps: usize) -> String {
	let mut parts = input.split("\n\n");
	let start = parts.next().unwrap().chars().collect::<Vec<_>>();
	let instructions = parts.next().unwrap()
		.lines()
		.map(|l| l.split(" -> "))
		.map(|mut l| {
			let mut from = l.next().unwrap().chars();
			((from.next().unwrap(), from.next().unwrap()), l.next().unwrap().chars().next().unwrap())
		}).collect::<HashMap<_, _>>();

	let mut current_pairs = IndexMap::new();
	for i in 0..start.len() - 1 {
		current_pairs.insert((start[i], start[i + 1]), *current_pairs.get(&(start[i], start[i + 1])).unwrap_or(&0) + 1);
	}
	let mut current_amounts = HashMap::new();

	for _ in 0..steps {
		let mut new_pairs = IndexMap::new();
		let mut new_amounts = HashMap::new();
		let mut is_first = true;
		for (k, v) in current_pairs {
			let mapping = *instructions.get(&k).unwrap();
			new_pairs.insert(
				(k.0, mapping),
				v + *new_pairs.get(&(k.0, mapping)).unwrap_or(&0)
			);
			new_pairs.insert(
				(mapping, k.1),
				v + *new_pairs.get(&(mapping, k.1)).unwrap_or(&0)
			);
			if is_first { new_amounts.insert(k.0, *new_amounts.get(&k.0).unwrap_or(&0) + v); }
			new_amounts.insert(mapping, *new_amounts.get(&mapping).unwrap_or(&0) + v);
			new_amounts.insert(k.1, *new_amounts.get(&k.1).unwrap_or(&0) + v);
			is_first = false;
		}
		current_pairs = new_pairs;
		current_amounts = new_amounts;
	}

	let max = current_amounts.iter()
		.fold(0usize, |a, (_, &b)| if b > a { b } else { a });
	let min = current_amounts.iter()
		.fold(usize::MAX, |a, (_, &b)| if b < a { b } else { a });
	(max - min).to_string()
}
