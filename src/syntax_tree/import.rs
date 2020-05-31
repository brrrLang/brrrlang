use crate::syntax_tree::SyntaxTreeElement;
use crate::tree::Tree;

pub struct Import {
	tree: Tree,
}

impl Import {
	pub fn new(tree: Tree) -> Import {
		Import { tree }
	}
}

impl SyntaxTreeElement for Import {
	fn has_children(&self) -> bool {
		false
	}

	fn children(&self) -> Vec<Box<dyn SyntaxTreeElement>> {
		vec!()
	}

	fn to_code(&self) -> String {
		format!("Import {}", tree)
	}
}