use crate::pipeline;
use core::{default::Default, ops::FnOnce};
use ext_trait::extension;

#[extension(pub trait ResultExtension)]
impl<T, E> Result<T, E> {
	fn filter_or(self, predicate: impl FnOnce(&T) -> bool, default_err: E) -> Self {
		match self {
			Ok(ref ok) if !predicate(ok) => Err(default_err),
			this => this,
		}
	}

	fn filter_or_else(
		self,
		predicate: impl FnOnce(&T) -> bool,
		default_err_supplier: impl FnOnce() -> E,
	) -> Self {
		match self {
			Ok(ref ok) if !predicate(ok) => pipeline!(default_err_supplier() => Err),
			this => this,
		}
	}

	fn filter_or_default(self, predicate: impl FnOnce(&T) -> bool) -> Self
	where
		E: Default,
	{
		self.filter_or_else(predicate, E::default)
	}
}
