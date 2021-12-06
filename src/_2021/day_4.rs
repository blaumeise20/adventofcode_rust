use crate::Day;
pub fn new() -> Day4 { Day4 }
pub struct Day4;

impl Day for Day4 {
	fn part_1(&mut self, input: &str) -> String { calc(input, true) }
	fn part_2(&mut self, input: &str) -> String { calc(input, false) }
}

fn calc(input: &str, break_on_win: bool) -> String {
	let mut lines = input.split("\n\n");
	let random_nums = lines.next().unwrap().split(",").map(|x| x.parse::<i32>().unwrap());
	let mut boards = lines.map(|line| 
		line.trim().split("\n")
			.map(|row| {
				row.split_whitespace().map(|n| (n.parse().unwrap(), false)).collect()
			}).collect()).collect::<Vec<Vec<Vec<(i32, bool)>>>>();

	let mut boards_won = vec![false; boards.len()];
	let mut winning = (0, 0);
	'l: for num in random_nums {
		for (i, board) in boards.iter_mut().enumerate() {
			if boards_won[i] { continue; }

			for x in 0..5 { for y in 0..5 {
				if board[y][x].0 == num { board[y][x].1 = true; }
			} }

			for n in 0..5 {
				if board[n].iter().all(|x| x.1) ||
				   board.iter().all(|x| x[n].1) {
					boards_won[i] = true;
					winning = (i, num);
					if break_on_win { break 'l; } else { break; }
				}
			}
		}
	}

	(boards[winning.0].iter()
		.map(|row| row.iter().filter(|(_, m)| !m).map(|(n, _)| n).sum::<i32>())
		.sum::<i32>() * winning.1).to_string()
}
