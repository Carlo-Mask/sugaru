use ext_trait::extension;
use std::convert::identity;

#[extension(pub trait OptionExt)]
impl<T> Option<T> {
	fn when(condition: bool, supplier: impl FnOnce() -> T) -> Self {
		condition.then(supplier)
	}

	fn unless(condition: bool, supplier: impl FnOnce() -> T) -> Self {
		Self::when(!condition, supplier)
	}
}

#[extension(pub trait OptionBoolExt)]
impl Option<bool> {
	fn is_some_true(self) -> bool {
		self.is_some_and(identity)
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use std::ops::Not;

	#[test]
	fn is_some_true_test() {
		assert!(Some(true).is_some_true());
		assert!(Some(false).is_some_true().not());
		assert!(None.is_some_true().not());
	}
}
