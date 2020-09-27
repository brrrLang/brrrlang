pub mod error;
pub mod warning;

pub struct Warning {
	error_area: String,
	line_num: i32,
	message: String,
	line_text: String,
}
impl Warning {
	pub fn new(error_area: String, line_num: i32, message: String, line_text: String) -> Warning {
		return Warning {
			error_area: error_area,
			line_num: line_num,
			message: message,
			line_text: line_text,
		}
	}
}
pub struct Error {
	error_area: String,
	line_num: i32,
	message: String,
	line_text: String,
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