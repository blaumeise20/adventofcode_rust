use crate::{Day, util::BitReader};
pub fn new() -> Day15 { Day15 }
pub struct Day15;

impl Day for Day15 {
	fn part_1(&mut self, input: &str) -> String { calculate_packet(&mut parse_input(input)).0.to_string() }
	fn part_2(&mut self, input: &str) -> String { calculate_packet(&mut parse_input(input)).1.to_string() }
}

fn parse_input(input: &str) -> BitReader {
	BitReader::from_bits(input.chars()
		.map(|c| c.to_digit(16).unwrap())
		.flat_map(|c| format!("{:04b}", c).chars().collect::<Vec<_>>())
		.map(|c| c == '1')
		.collect())
}

fn calculate_packet(buffer: &mut BitReader) -> (usize, usize) {
	let version = buffer.read_bits(3).unwrap() as u8;
	let packet_type = buffer.read_bits(3).unwrap() as u8;

	let mut result;
	let mut version_sum = version as usize;
	match packet_type {
		4 => {
			result = 0;
			loop {
				let more = buffer.next().unwrap();
				result = (result << 4) | buffer.read_bits(4).unwrap();
				if !more { break; }
			}
		}
		op_code => {
			let mut children = vec![];
			if !buffer.next().unwrap() {
				let total_count = buffer.read_bits(15).unwrap();
				let end_length = buffer.left() - total_count;
				while buffer.left() > end_length {
					let (child_version_sum, child_value) = calculate_packet(buffer);
					version_sum += child_version_sum;
					children.push(child_value);
				}
			}
			else {
				let packet_count = buffer.read_bits(11).unwrap();
				for _ in 0..packet_count {
					let (child_version_sum, child_value) = calculate_packet(buffer);
					version_sum += child_version_sum;
					children.push(child_value);
				}
			}

			result = match op_code {
				0 => children.iter().sum(),
				1 => children.iter().product(),
				2 => children.iter().min().unwrap().to_owned(),
				3 => children.iter().max().unwrap().to_owned(),
				5 => if children[0] >  children[1] { 1 } else { 0 },
				6 => if children[0] <  children[1] { 1 } else { 0 },
				7 => if children[0] == children[1] { 1 } else { 0 },
				_ => panic!("something bad happened"),
			};
		}
	}

	(version_sum, result)
}

// fn parse_input(input: &str) -> Vec<bool> {
// 	input.chars()
// 		.map(|c| c.to_digit(16).unwrap())
// 		.flat_map(|c| format!("{:04b}", c).chars().collect::<Vec<_>>())
// 		.map(|c| c == '1')
// 		.collect()
// }

// fn calculate_packet(buffer: &mut Vec<bool>) -> (usize, usize) {
// 	let version = u8::from_str_radix(&remove(buffer, 3), 2).unwrap();
// 	let packet_type = u8::from_str_radix(&remove(buffer, 3), 2).unwrap();

// 	let result;
// 	let mut version_sum = version as usize;
// 	match packet_type {
// 		4 => {
// 			let mut res = String::new();
// 			loop {
// 				let more = buffer.remove(0);
// 				res.push_str(&remove(buffer, 4));
// 				if !more { break; }
// 			}
// 			result = usize::from_str_radix(&res, 2).unwrap();
// 		}
// 		op_code => {
// 			let mut children = vec![];
// 			if !buffer.remove(0) {
// 				let total_count = u16::from_str_radix(&remove(buffer, 15), 2).unwrap();
// 				let end_length = buffer.len() as u16 - total_count;
// 				while buffer.len() as u16 > end_length {
// 					let (child_version_sum, child_value) = calculate_packet(buffer);
// 					version_sum += child_version_sum;
// 					children.push(child_value);
// 				}
// 			}
// 			else {
// 				let packet_count = u16::from_str_radix(&remove(buffer, 11), 2).unwrap();
// 				for _ in 0..packet_count {
// 					let (child_version_sum, child_value) = calculate_packet(buffer);
// 					version_sum += child_version_sum;
// 					children.push(child_value);
// 				}
// 			}

// 			result = match op_code {
// 				0 => children.iter().sum(),
// 				1 => children.iter().product(),
// 				2 => children.iter().min().unwrap().to_owned(),
// 				3 => children.iter().max().unwrap().to_owned(),
// 				5 => if children[0] >  children[1] { 1 } else { 0 },
// 				6 => if children[0] <  children[1] { 1 } else { 0 },
// 				7 => if children[0] == children[1] { 1 } else { 0 },
// 				_ => panic!("something bad happened"),
// 			};
// 		}
// 	}

// 	(version_sum, result)
// }

// fn remove(buffer: &mut Vec<bool>, count: usize) -> String {
// 	(0..count).map(|_| if buffer.remove(0) { '1' } else { '0' }).collect::<String>()
// }
