use crate::Day;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_9;

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

		_ => None
	}
}