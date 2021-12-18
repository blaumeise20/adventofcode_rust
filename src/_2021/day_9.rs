use crate::Day;
pub fn new() -> Day9 { Day9 }
pub struct Day9;

impl Day for Day9 {
	fn part_1(&mut self, input: &str) -> String {
		let grid = parse_input(input);
		grid.iter().enumerate()
			.map(|(y, d)| 
				d.iter().enumerate()
					.filter(|&(x, &n)| is_lowest(&grid, x, y, n))
					.map(|(_, &n)| n as usize + 1)
					.sum::<usize>())
			.sum::<usize>().to_string()
	}

	fn part_2(&mut self, input: &str) -> String {
		let grid = parse_input(input);
		let mut bassins = vec![];

		for (y, d) in grid.iter().enumerate() {
			for (x, _) in d.iter().enumerate().filter(|&(x, &n)| is_lowest(&grid, x, y, n)) {
				let mut new_data = grid.iter()
					.map(|d| d.iter().map(|&n| (n, false)).collect())
					.collect::<Vec<Vec<(u8, bool)>>>();
				bassins.push(calculate_bassin(&mut new_data, x, y));
			}
		}

		bassins.sort();
		bassins.iter()
			.skip(bassins.len() - 3)
			.take(3)
			.product::<usize>().to_string()
	}
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
	input.lines()
		.map(|l| l.chars()
			.map(|c| c.to_digit(10).unwrap() as u8)
			.collect())
		.collect::<Vec<Vec<u8>>>()
}
fn is_lowest(data: &Vec<Vec<u8>>, x: usize, y: usize, n: u8) -> bool {
	(y == 0 || data[y - 1][x] > n) && // top
	(x == 0 || data[y][x - 1] > n) && // left
	(x >= data.len() - 1 || data[y][x + 1] > n) && // right
	(y >= data.len() - 1 || data[y + 1][x] > n) // bottom
}
fn calculate_bassin(data: &mut Vec<Vec<(u8, bool)>>, x: usize, y: usize) -> usize {
	let mut bassin_size = 1;
	data[y][x].1 = true;
	for (ox, oy) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
		if x == 0 && ox == -1 ||                // overflow left
		   x == data[0].len() - 1 && ox == 1 || // overflow right
		   y == 0 && oy == -1 ||                // overflow top
		   y == data.len() - 1 && oy == 1       // overflow bottom
			{ continue; }

		let dx = (x as isize + ox) as usize;
		let dy = (y as isize + oy) as usize;
		if data[dy][dx].0 != 9 && !data[dy][dx].1 {
			bassin_size += calculate_bassin(data, dx, dy);
		}
	}
	bassin_size
}
