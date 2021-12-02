use crate::Day;

mod day_1;
mod day_2;

pub fn new(day: i32) -> Option<Box<dyn Day>> {
	match day {
		1 => Some(box day_1::new()),
		2 => Some(box day_2::new()),
		_ => None
	}
}