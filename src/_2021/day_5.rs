use crate::Day;
pub fn new() -> Day5 { Day5 }
pub struct Day5;

impl Day for Day5 {
	fn part_1(&mut self, input: &str) -> String { calc(input, false) }
	fn part_2(&mut self, input: &str) -> String { calc(input, true) }
}

fn calc(input: &str, allow_diagonals: bool) -> String {
	let lines = input.lines().map(|l | {
		let mut points = l.split(" -> ")
			.map(|p| p.split(',').map(|c| c.parse::<i32>().unwrap()))
			.map(|mut p| (p.next().unwrap(), p.next().unwrap()));
		(points.next().unwrap(), points.next().unwrap())
	});

	let mut grid = vec![vec![0u8; 10000]; 10000];
	let mut count = 0;
	for ((x1, y1), (x2, y2)) in lines {
		if x1 != x2 && y1 != y2 && !allow_diagonals { continue; }

		let dx = if x1 > x2 { -1 } else if x1 < x2 { 1 } else { 0 };
		let dy = if y1 > y2 { -1 } else if y1 < y2 { 1 } else { 0 };
		let (mut x, mut y) = (x1, y1);
		while x != x2 + dx || y != y2 + dy {
			let field = &mut grid[y as usize][x as usize];
			if *field == 0 { *field = 1; }
			else if *field == 1 { count += 1; *field = 2; }
			x += dx; y += dy;
		}
	}

	count.to_string()
}
