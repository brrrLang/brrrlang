use crate::syntax_tree;

pub struct Module {
	pub module_name: String,
	pub sub_modules: Vec<Module>,
	pub code: Vec<Box<dyn syntax_tree::SyntaxTreeElement>>
}