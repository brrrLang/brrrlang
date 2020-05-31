use crate::tree::module::Module;

pub struct Require {
	module: Module
}

impl Require {
	pub fn new(module: Module) -> Require {
		Require { module }
	}
}