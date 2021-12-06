use crate::Day;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;

pub fn new(day: i32) -> Option<Box<dyn Day>> {
	match day {
		1 => Some(box day_1::new()),
		2 => Some(box day_2::new()),
		3 => Some(box day_3::new()),
		4 => Some(box day_4::new()),
		5 => Some(box day_5::new()),
		_ => None
	}
}