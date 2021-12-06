use crate::Day;

mod day_1;
mod day_2;
mod day_3;

pub fn new(day: i32) -> Option<Box<dyn Day>> {
	match day {
		1 => Some(box day_1::new()),
		2 => Some(box day_2::new()),
		3 => Some(box day_3::new()),
		_ => None
	}
}