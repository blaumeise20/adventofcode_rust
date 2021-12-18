use crate::Day;
pub fn new() -> Day6 { Day6 }
pub struct Day6;

impl Day for Day6 {
	fn part_1(&mut self, input: &str) -> String { calc(input, 80) }
	fn part_2(&mut self, input: &str) -> String { calc(input, 256) }
}

fn calc(input: &str, days: u16) -> String {
	let input = input.split(',')
		.map(|x| x.parse::<usize>().unwrap());
	let mut fish_count = [0usize; 9];
	for n in input { fish_count[n] += 1; }

	for _ in 0..days {
		let mut new_fish_count = [0; 9];
		for i in 0..fish_count.len() {
			if i == 0 {
				new_fish_count[8] += fish_count[i];
				new_fish_count[6] += fish_count[i];
			}
			else { new_fish_count[i - 1] += fish_count[i]; }
		}
		fish_count = new_fish_count;
	}

	fish_count.iter().sum::<usize>().to_string()
}
