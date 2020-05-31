use crate::syntax_tree;

pub struct Module {
	pub module_name: String,
	pub sub_modules: Vec<Module>,
	pub code: Vec<Box<dyn syntax_tree::SyntaxTreeElement>>
}

impl Module {
	pub fn new(module_name: &String, sub_modules: &Vec<Module>, code: &Vec<Box<dyn syntax_tree::SyntaxTreeElement>>) -> Module {
		let module_name = format!("{}", module_name);
		let sub_modules: Vec<Module> = sub_modules.to_vec();
		let code: Vec<Box<dyn syntax_tree::SyntaxTreeElement>> = code.to_vec();
		Module { module_name, sub_modules, code }
	}

	pub fn new_no_subs(module_name: &String, code: &Vec<Box<dyn syntax_tree::SyntaxTreeElement>>) -> Module {
		let module_name = format!("{}", module_name);
		let code: Vec<Box<dyn syntax_tree::SyntaxTreeElement>> = code.to_vec();
		let sub_modules = vec!();
		Module { module_name, code, sub_modules }
	}
}
