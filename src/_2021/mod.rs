use crate::Day;

mod day_1;

pub fn new(day: i32) -> Option<Box<dyn Day>> {
	match day {
		1 => Some(box day_1::new()),
		_ => None
	}
}