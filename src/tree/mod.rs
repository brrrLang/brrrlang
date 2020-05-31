use crate::tree::module::Module;

pub mod module;

pub struct Tree {
	name: String,
	root_module: module::Module,
	children: Vec<module::Module>
}

impl Tree {
	pub fn new_no_children(name: String, root_module: module::Module) -> Tree {
		Tree { name, root_module, children: vec!() }
	}

	pub fn new(name: String, root_module: module::Module, children: Vec<module::Module>) -> Tree {
		Tree { name, root_module, children }
	}

	pub fn has_children(&self) -> bool {
		return self.children.is_empty();
	}
}