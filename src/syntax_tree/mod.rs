pub mod import;
pub mod require;

pub trait SyntaxTreeElement {
	fn has_children(&self) -> bool;
	fn children(&self) -> Vec<Box<dyn SyntaxTreeElement>>;
	fn to_code(&self) -> String;
}

