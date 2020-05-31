use crate::tree::module::Module;

#[derive(Clone)]
pub struct Require {
	module: Module
}

impl Require {
	pub fn new(module: Module) -> Require {
		Require { module }
	}
}