use crate::Day;
pub fn new() -> Day11 { Day11 }
pub struct Day11;

impl Day for Day11 {
	fn part_1(&mut self, input: &str) -> String {
		let mut grid = parse_input(input);
		let mut flashes = 0;
		for _ in 0..100 {
			flashes += count_flashes(&mut grid).0;
		}
		flashes.to_string()
	}

	fn part_2(&mut self, input: &str) -> String {
		let mut grid = parse_input(input);
		let mut steps = 0usize;
		loop {
			steps += 1;
			if count_flashes(&mut grid).1 {
				return steps.to_string();
			}
		}
	}
}

fn parse_input(input: &str) -> Vec<Vec<(u8, bool)>> {
	input.lines()
		.map(|line| line.chars()
			.map(|c| c.to_digit(10).unwrap() as u8)
			.map(|c| (c, false))
			.collect())
		.collect()
}

fn count_flashes(grid: &mut Vec<Vec<(u8, bool)>>) -> (usize, bool) {
    let mut flashes = 0;
    // try to flash every octopus
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            flash(grid, x, y, &mut flashes);
        }
    }

    // clear all flash markers
    let mut result = true;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if !grid[y][x].1 {
                result = false;
            }
            grid[y][x].1 = false;
        }
    }

    // how many octopuses flashed and if all of them flashed
    (flashes, result)
}

fn flash(grid: &mut Vec<Vec<(u8, bool)>>, x: usize, y: usize, flashes: &mut usize) {
    if grid[y][x].1 { return; }
    grid[y][x].0 += 1;
    if grid[y][x].0 <= 9 { return; }

    *flashes += 1;
    grid[y][x].0 = 0;
    grid[y][x].1 = true;

    // flash neighbors
    for dy in -1..2 {
        for dx in -1..2 {
            if dx == 0 && dy == 0 { continue; }
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx < 0 || ny < 0 { continue; }
            if nx >= grid[y].len() as i32 || ny >= grid[y].len() as i32 { continue; }
			
            flash(grid, nx as usize, ny as usize, flashes);
        }
    }
}