pub mod import;
pub mod require;

pub trait SyntaxTreeElement: dyn_clone::DynClone {
	fn has_children(&self) -> bool;
	fn children(&self) -> Vec<Box<dyn SyntaxTreeElement>>;
	fn to_code(&self) -> String;
}

clone_trait_object!(SyntaxTreeElement);
