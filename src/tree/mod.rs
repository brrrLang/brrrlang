pub mod module;

pub struct Tree {
	name: String,
	root_module: module::Module,
	children: Vec<module::Module>
}