use std::collections::HashMap;

use crate::Day;
pub fn new() -> Day12 { Day12 }
pub struct Day12;

impl Day for Day12 {
	fn part_1(&mut self, input: &str) -> String {
		walk_path(&parse_input(input), "start", vec!["start"], true)
			.unwrap().len().to_string()
	}

	fn part_2(&mut self, input: &str) -> String {
		walk_path(&parse_input(input), "start", vec!["start"], false)
			.unwrap().len().to_string()
	}
}

fn parse_input<'a>(input: &'a str) -> HashMap<&'a str, Vec<&'a str>> {
	let mut connections: HashMap<&str, Vec<&str>> = HashMap::new();
	for mut l in input.lines().map(|l| l.split('-')) {
		let (from, to) = (l.next().unwrap(), l.next().unwrap());
		if connections.contains_key(from) { connections.get_mut(from).unwrap().push(to); }
		else { connections.insert(from, vec![to]); }
		if connections.contains_key(to) { connections.get_mut(to).unwrap().push(from); }
		else { connections.insert(to, vec![from]); }
	}
	connections
}

fn walk_path<'a>(connections: &HashMap<&str, Vec<&'a str>>, current_cave: &'a str, visited_small_caves: Vec<&'a str>, twice_visited: bool) -> Option<Vec<Vec<&'a str>>> {
	if current_cave == "end" { return Some(vec![vec!["end"]]); }

	let mut paths = vec![];
	let mut found_end = false;
	for &c in connections.get(current_cave).unwrap() {
		macro_rules! try_walk {
			($add_to_visited:literal, $twice_visited:expr) => {{
				let mut new_visited = visited_small_caves.clone();
				if $add_to_visited && c.chars().all(char::is_lowercase) {
					new_visited.push(c);
				}

				if let Some(n) = walk_path(&connections, c, new_visited, $twice_visited) {
					for mut p in n {
						p.insert(0, current_cave);
						paths.push(p);
					}
					found_end = true;
				}
			}}
		}
	
		if visited_small_caves.contains(&c) {
			if !twice_visited && c != "start" { try_walk!(false, true); }
		}
		else { try_walk!(true, twice_visited); }
	}

	if found_end { Some(paths) } else { None }
}
