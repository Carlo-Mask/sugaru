use ext_trait::extension;

#[extension(pub trait OptionExtension)]
impl<T> Option<T> {
	fn when(condition: bool, supplier: impl FnOnce() -> T) -> Self {
		condition.then(supplier)
	}

	fn unless(condition: bool, supplier: impl FnOnce() -> T) -> Self {
		Self::when(!condition, supplier)
	}
}
