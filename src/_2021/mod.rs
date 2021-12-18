use crate::Day;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_9;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;

pub fn new(day: i32) -> Option<Box<dyn Day>> {
	match day {
		1 => Some(box day_1::new()),
		2 => Some(box day_2::new()),
		3 => Some(box day_3::new()),
		4 => Some(box day_4::new()),
		5 => Some(box day_5::new()),
		6 => Some(box day_6::new()),
		7 => Some(box day_7::new()),
		9 => Some(box day_9::new()),
		10 => Some(box day_10::new()),
		11 => Some(box day_11::new()),
		12 => Some(box day_12::new()),
		13 => Some(box day_13::new()),
		14 => Some(box day_14::new()),
		15 => Some(box day_15::new()),
		16 => Some(box day_16::new()),
		17 => Some(box day_17::new()),

		_ => None
	}
}