use crate::Day;
pub fn new() -> Day8 { Day8 }
pub struct Day8;

impl Day for Day8 {
	fn part_1(&mut self, input: &str) -> String {
		input.lines()
			.map(|l| l.split(" | ")
				.nth(1).unwrap()
				.split(" ")
				.filter(|x| x.len() == 2 || x.len() == 4 || x.len() == 3 || x.len() == 7)
				.count())
			.sum::<usize>().to_string()
	}

	fn part_2(&mut self, input: &str) -> String {
		input.lines().map(|l| {
			let mut parts = l.split(" | ").map(|x| x.split(" "));
			let decoding = parts.next().unwrap().collect::<Vec<_>>();

			let one = decoding.iter().find(|x| x.len() == 2).unwrap();
			let four = decoding.iter().find(|x| x.len() == 4).unwrap();

			parts.next().unwrap()
				.map(|n| {
					let count_one = n.chars().filter(|&x| one.contains(x)).count();
					let count_four = n.chars().filter(|&x| four.contains(x)).count();
					match n.len() {
						2 => '1', 3 => '7', 4 => '4',
						5 => {
							if count_one == 1 && count_four == 2 { '2' }
							else if count_one == 2 && count_four == 3 { '3' }
							else { '5' }
						},
						6 => {
							if count_one == 2 && count_four == 3 { '0' }
							else if count_one == 1 && count_four == 3 { '6' }
							else { '9' }
						}
						7 => '8', _ => panic!("{}", n),
					}
				})
				.collect::<String>()
				.parse::<i32>().unwrap()
		}).sum::<i32>().to_string()
	}

	// fn part_2(&mut self, input: &str) -> String {
	// 	let mut result = 0;
	// 	for l in input.lines() {
	// 		let mut parts = l.split(" | ").map(|x| x.split(" "));
	// 		let decoding = parts.next().unwrap().collect::<Vec<_>>();
	// 		let data = parts.next().unwrap().collect::<Vec<_>>();

	// 		let number_map = [
	// 			vec!['a', 'b', 'c', 'e', 'f', 'g'],
	// 			vec!['c', 'f'],
	// 			vec!['a', 'c', 'd', 'e', 'g'],
	// 			vec!['a', 'c', 'd', 'f', 'g'],
	// 			vec!['b', 'c', 'd', 'f'],
	// 			vec!['a', 'b', 'd', 'f', 'g'],
	// 			vec!['a', 'b', 'd', 'e', 'f', 'g'],
	// 			vec!['a', 'c', 'f'],
	// 			vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'],
	// 			vec!['a', 'b', 'c', 'd', 'f', 'g'],
	// 		];

	// 		let count_map = [
	// 			/* 2 */ vec!['c', 'f'],
	// 			/* 3 */ vec!['a', 'c', 'f'],
	// 			/* 4 */ vec!['b', 'c', 'd', 'f'],
	// 		];

	// 		let mut segment_map = vec![];
	// 		for _ in 0..7 { segment_map.push(vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']); }

	// 		let mut combinations5 = vec![];
	// 		let mut combinations6 = vec![];
	// 		for d in decoding.iter() {
	// 			let len = d.len();

	// 			match len {
	// 				2 | 3 | 4 => {
	// 					let map = &count_map[len - 2];
	// 					for c in d.chars() {
	// 						for i in 0..7 {
	// 							if !map.contains(&((i + 'a' as u8) as char)) {
	// 								segment_map[i as usize].retain(|&x| x != c);
	// 							}
	// 						}
	// 					}		
	// 				}
	// 				7 => {},
	// 				len => {
	// 					// note for later
	// 					if len == 5 { combinations5.push(d.to_string()); }
	// 					if len == 6 { combinations6.push(d.to_string()); }
	// 				}
	// 			}
	// 		}

	// 		let mut num2 = vec![];
	// 		let mut num3 = String::new();
	// 		let mut num5 = vec![];

	// 		'a: for (i1, c1) in combinations5.iter().enumerate() {
	// 			for (i2, c2) in combinations5.iter().enumerate() {
	// 				if c1 == c2 { continue; }

	// 				// count similar letters in c1 and c2
	// 				let mut count = 0;
	// 				for ch in c1.chars() {
	// 					if c2.contains(ch) { count += 1; }
	// 				}

	// 				// if count is 3, then we know that c1 and c2 are the numbers 2 and 5, so the remaining thing is the number 3
	// 				if count == 3 {
	// 					let mut ix = vec![0, 1, 2];
	// 					ix.retain(|&x| x != i1 && x != i2);
	// 					num3 = combinations5.remove(ix[0]);
	// 					let map = &number_map[3];
	// 					for c in num3.chars() {
	// 						for i in 0..7 {
	// 							if !map.contains(&((i + 'a' as u8) as char)) {
	// 								segment_map[i as usize].retain(|&x| x != c);
	// 							}
	// 						}
	// 					}
	// 					break 'a;
	// 				}
	// 			}
	// 		}

	// 		for (_, c) in combinations5.iter().enumerate() {
	// 			for i in [2, 5] {
	// 				let map = &number_map[i];
	// 				let mut count = 0;
	// 				for m in map {
	// 					for ch in c.chars() {
	// 						if segment_map[(*m as usize) - 'a' as usize].contains(&ch) { count += 1; break; }
	// 					}
	// 				}

	// 				println!("{} as {}: {}", c, i, count);

	// 				if count == 5 {
	// 					if i == 2 {
	// 						num2.push(c.to_string());
	// 					}
	// 					else {
	// 						num5.push(c.to_string());
	// 					}
	// 				}
	// 			}
	// 		}

	// 		let num2new;
	// 		let num5new;

	// 		if num2.len() == 1 {
	// 			num2new = num2[0].clone();
	// 			num5new = num5.remove(num5.iter().position(|x| x != &num2[0]).unwrap());
	// 		}
	// 		else if num5.len() == 1 {
	// 			num5new = num5[0].clone();
	// 			num2new = num2.remove(num2.iter().position(|x| x != &num5[0]).unwrap());
	// 		}
	// 		else { panic!("{:?}", num2); }

	// 		let num2 = num2new;
	// 		let num5 = num5new;

	// 		for c in num2.chars() {
	// 			for i in 0..7 {
	// 				if !number_map[2].contains(&((i + 'a' as u8) as char)) {
	// 					segment_map[i as usize].retain(|&x| x != c);
	// 				}
	// 			}
	// 		}

	// 		for c in num5.chars() {
	// 			for i in 0..7 {
	// 				if !number_map[5].contains(&((i + 'a' as u8) as char)) {
	// 					segment_map[i as usize].retain(|&x| x != c);
	// 				}
	// 			}
	// 		}

	// 		let mut num0 = vec![];
	// 		let mut num6 = vec![];
	// 		let mut num9 = vec![];
			
	// 		for (_, c) in combinations6.iter().enumerate() {
	// 			for i in [0, 6, 9] {
	// 				let map = &number_map[i];
	// 				let mut count = 0;
	// 				for m in map {
	// 					for ch in c.chars() {
	// 						if segment_map[(*m as usize) - 'a' as usize].contains(&ch) { count += 1; break; }
	// 					}
	// 				}

	// 				println!("{} as {}: {}", c, i, count);

	// 				if count == 6 {
	// 					if i == 0 {
	// 						num0.push(c.to_string());
	// 					}
	// 					else if i == 6 {
	// 						num6.push(c.to_string());
	// 					}
	// 					else {
	// 						num9.push(c.to_string());
	// 					}
	// 				}
	// 			}
	// 		}

	// 		let mut num0new = String::new();
	// 		let mut found0 = false;
	// 		let mut num6new = String::new();
	// 		let mut found6 = false;
	// 		let mut num9new = String::new();
	// 		let mut found9 = false;

	// 		'l: loop {
	// 			if !found0 {
	// 				for n in num0.iter() {
	// 					if !num6.contains(n) && !num9.contains(n) {
	// 						num0new = n.clone();
	// 						found0 = true;
	// 						continue 'l;
	// 					}
	// 				}
	// 			}
	// 			if !found6 {
	// 				for n in num6.iter() {
	// 					if !num0.contains(n) && !num9.contains(n) {
	// 						num6new = n.clone();
	// 						found6 = true;
	// 						continue 'l;
	// 					}
	// 				}
	// 			}
	// 			if !found9 {
	// 				for n in num9.iter() {
	// 					if !num0.contains(n) && !num6.contains(n) {
	// 						num9new = n.clone();
	// 						found9 = true;
	// 						continue 'l;
	// 					}
	// 				}
	// 			}
	// 			break;
	// 		}

	// 		let num0 = num0new;
	// 		let num6 = num6new;
	// 		let num9 = num9new;

	// 		println!("{:?}", segment_map);
	// 		println!("{:?}", num2);
	// 		println!("{:?}", num3);
	// 		println!("{:?}", num5);
	// 		println!("{:?}", num0);
	// 		println!("{:?}", num6);
	// 		println!("{:?}", num9);

	// 		// check if one of segment_map with length 2 has a duplicate and dominates another
	// 		// in this case: ['c', 'd', 'g'], ['c', 'g']
	// 		// we can remove c and g from the first item, because they have to be in the second
	// 		// for i in 0..7 {
	// 		// 	let mut segment_map_i = segment_map[i as usize].clone();
	// 		// 	for j in 0..7 {
	// 		// 		if i == j { continue; }
	// 		// 		let mut segment_map_j = segment_map[j as usize].clone();
	// 		// 		for c in segment_map_i.iter() {
	// 		// 			if segment_map_j.contains(c) {
	// 		// 				segment_map_j.retain(|&x| x != *c);
	// 		// 			}
	// 		// 		}
	// 		// 		if segment_map_j.len() == 0 && segment_map_i.len() > segment_map[j as usize].len() {
	// 		// 			segment_map_i.retain(|&x| !segment_map[j as usize].contains(&x));
	// 		// 		}
	// 		// 	}
	// 		// 	segment_map[i as usize] = segment_map_i;
	// 		// }

	// 		let mut val = String::new();
	// 		for n in data.iter() {
	// 			let len = n.len();
	// 			match len {
	// 				2 => val.push('1'),
	// 				3 => val.push('7'),
	// 				4 => val.push('4'),
	// 				5 => {
	// 					// check if n has the same chars as num3

	// 					let mut count = 0;
	// 					for ch in n.chars() {
	// 						if num3.contains(ch) { count += 1; }
	// 					}
	// 					if count == 5 {
	// 						val.push('3');
	// 						continue;
	// 					}

	// 					count = 0;
	// 					for ch in n.chars() {
	// 						if num2.contains(ch) { count += 1; }
	// 					}
	// 					if count == 5 {
	// 						val.push('2');
	// 						continue;
	// 					}

	// 					count = 0;
	// 					for ch in n.chars() {
	// 						if num5.contains(ch) { count += 1; }
	// 					}
	// 					if count == 5 {
	// 						val.push('5');
	// 						continue;
	// 					}
	// 				},
	// 				7 => val.push('8'),
	// 				_ => {

	// 				}
	// 			}
	// 		}
	// 		result += val.parse::<i32>().unwrap();
	// 	}

	// 	result.to_string()
	// }
}
