use crate::Day;
pub fn new() -> Day15 { Day15 }
pub struct Day15;

impl Day for Day15 {
	fn part_1(&mut self, input: &str) -> String {
		let grid = input.lines()
			.map(|l| l.chars()
				.map(|c| c.to_digit(10).unwrap() as u8)
				.collect())
			.collect::<Vec<Vec<_>>>();
		let height = grid.len();
		let width = grid[0].len();

		let mut current_positions = vec![(0, 0, 0)];
		let directions = [(0, 1), (1, 0)];
		loop {
			let mut new_positions = vec![];
			for (x, y, path_size) in current_positions {
				for (dx, dy) in directions.iter() {
					let nx = x as i32 + dx;
					let ny = y as i32 + dy;
					if nx < 0 || nx >= width as i32 || ny < 0 || ny >= height as i32 { continue; }
					let nx = nx as usize;
					let ny = ny as usize;
					let new_path_size = path_size + grid[ny][nx] as usize;
					if let Some(a) = new_positions.iter_mut().find(|(nx2, ny2, _)| *nx2 == nx && *ny2 == ny) {
						if a.2 > new_path_size {
							*a = (nx, ny, new_path_size);
						}
					}
					else {
						new_positions.push((nx, ny, new_path_size));
					}
				}
			}
			if new_positions.len() == 1 {
				return new_positions[0].2.to_string();
			}
			current_positions = new_positions;
		}
	}

	fn part_2(&mut self, input: &str) -> String {
		let old_grid = input.lines()
			.map(|l| l.chars()
				.map(|c| c.to_digit(10).unwrap() as u8)
				.map(|c| (c, 0))
				.collect())
			.collect::<Vec<Vec<_>>>();
		
		let height = old_grid.len();
		let width = old_grid[0].len();
		let mut grid = vec![vec![(0, 0, vec![]); width * 5]; height * 5];
		for row in 0..height {
			for col in 0..width {
				for i in 0..5 {
					for j in 0..5 {
						let mut new_risk = old_grid[row][col].0 + i as u8 + j as u8;
						if new_risk > 9 { new_risk -= 9; }
						grid[row + i * height][col + j * width] = (new_risk, 0, vec![]);
					}
				}
			}
		}

		let height = grid.len();
		let width = grid[0].len();
		grid[0][0].1 = grid[0][0].0 as usize;

		for x in 0..width {
			for y in 0..height {
				calculate_risk(&mut grid, x, y, width, height);
			}
		}

		print_grid(&grid, width, height);

		(grid[height - 1][width - 1].1 - grid[0][0].0 as usize).to_string()
	}
}

fn print_grid(grid: &Vec<Vec<(u8, usize, Vec<(usize, usize)>)>>, width: usize, height: usize) {
	let path = grid[height - 1][width - 1].2.clone();
	let (mut x, mut y) = (0, 0);
	for row in 0..height {
		for col in 0..width {
			if path.contains(&(col, row)) {
				if col < x || row < y {
					for i in -10..10 {
						for j in -10..10 {
							let nx = col as i32 + i;
							let ny = row as i32 + j;
							let nx = nx as usize;
							let ny = ny as usize;
							if path.contains(&(nx, ny)) {
								print!("#");
							}
							else {
								print!(".");
							}
						}
						println!();
					}
					return;
				}
				(x, y) = (col, row);
				// print!("\x1b[1;32m");
			}
			// else {
			// 	print!("\x1b[0m");
			// }
			// print!("{}", grid[row][col].0);
		}
		// println!();
	}
}

fn calculate_risk(grid: &mut Vec<Vec<(u8, usize, Vec<(usize, usize)>)>>, x: usize, y: usize, width: usize, height: usize) {
	let mut min_risk = 0;
	let mut min_path = vec![];
	for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
		let nx = x as i32 + dx;
		let ny = y as i32 + dy;
		if nx < 0 || ny < 0 || nx >= width as i32 || ny >= height as i32 { continue; }
		let nx = nx as usize;
		let ny = ny as usize;

		let (_, risk, path) = &grid[ny][nx];
		if min_risk == 0 || *risk < min_risk {
			min_risk = *risk;
			min_path = path.clone();
		}
	}
	let new_risk = min_risk + grid[y][x].0 as usize;
	if min_risk != 0 && (new_risk < grid[y][x].1 || grid[y][x].1 == 0) {
		grid[y][x].1 = new_risk;
		min_path.push((x, y));
		grid[y][x].2 = min_path;
		for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
			let nx = x as i32 + dx;
			let ny = y as i32 + dy;
			if nx < 0 || ny < 0 || nx >= width as i32 || ny >= height as i32 { continue; }
			let nx = nx as usize;
			let ny = ny as usize;

			if grid[ny][nx].1 != 0 {
				calculate_risk(grid, nx, ny, width, height);
			}
		}
	}
}
