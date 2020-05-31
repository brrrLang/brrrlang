use crate::tree::module::Module;

pub mod module;

#[derive(Clone)]
pub struct Tree {
	pub name: String,
	pub root_module: Module,
	pub children: Vec<Module>
}

impl Tree {
	pub fn new_no_children(name: &String, root_module: &Module) -> Tree {
		let name = format!("{}", name);
		let root_module = root_module.to_owned();
		Tree { name, root_module, children: vec!() }
	}

	pub fn new(name: String, root_module: module::Module, children: Vec<Module>) -> Tree {
		Tree { name, root_module, children }
	}

	pub fn has_children(&self) -> bool {
		return self.children.is_empty();
	}
}