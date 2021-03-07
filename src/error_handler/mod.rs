pub mod error;
pub mod warning;

#[derive(Clone)]
pub struct Warning {
	pub error_area: String,
	pub line_num: i32,
	pub message: String,
	pub line_text: String,
}
#[allow(dead_code)]
impl Warning {
	pub fn new(error_area: String, line_num: i32, message: String, line_text: String) -> Warning {
		return Warning {
			error_area,
			line_num,
			message,
			line_text,
		}
	}
}

#[derive(Clone)]
pub struct Error {
	pub error_area: String,
	pub line_num: i32,
	pub message: String,
	pub line_text: String,
}
impl Error {
	pub fn new(error_area: String, line_num: i32, message: String, line_text: String) -> Error {
		return Error {
			error_area: error_area,
			line_num: line_num,
			message: message,
			line_text: line_text,
		}
	}
}