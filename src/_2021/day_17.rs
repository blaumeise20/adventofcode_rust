type Box = (i32, i32, i32, i32);

use crate::Day;
pub fn new() -> Day17 { Day17 }
pub struct Day17;

impl Day for Day17 {
	fn part_1(&mut self, input: &str) -> String {
		let target = parse_input(input);

		(0..100)
			.flat_map(|vx| (0..100)
				.map(move |vy| run_with_velocity(target, vx, vy).1))
			.max().unwrap().to_string()
	}

	fn part_2(&mut self, input: &str) -> String {
		let target = parse_input(input);

		(0..1000)
			.map(|vx| (-500..1000)
				.map(move |vy| if run_with_velocity(target, vx, vy).0 { 1 } else { 0 })
				.sum::<usize>())
			.sum::<usize>().to_string()
	}
}

fn parse_input(input: &str) -> (i32, i32, i32, i32) {
    let mut parts = input.split(", y=");
    let mut x = parts.next().unwrap().split('=').last().unwrap().split("..");
    let mut y = parts.next().unwrap().split("..");
	macro_rules! parse { ($p:ident) => { $p.next().unwrap().parse().unwrap() } }
    (parse!(x), parse!(y), // from
	 parse!(x), parse!(y)) // to
}

fn run_with_velocity(target: Box, mut vx: i32, mut vy: i32) -> (bool, i32) {
	let (mut x, mut y) = (0, 0);
	let mut max_y = 0;
	loop {
		x += vx; y += vy;
		if vx > 0 { vx -= 1; }
		else if vx < 0 { vx += 1; }
		vy -= 1;

		if y > max_y { max_y = y; }
		if x >= target.0 && x <= target.2 && y >= target.1 && y <= target.3 { return (true, max_y); }
		else if x > target.2 || y < target.1 { return (false, 0); }
	}
}
