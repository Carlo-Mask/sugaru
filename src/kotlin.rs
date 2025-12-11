use std::borrow::{Borrow, BorrowMut};

/// Utility functions inspired by [Kotlin scope functions](https://kotlinlang.org/docs/scope-functions.html)
pub trait KotlinExt: Sized {
	/// Inspired by https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/let.html
	/// # Examples
	/// ```
	/// # use crate::sugaru::kotlin::KotlinExt;
	/// assert_eq!(2.lets( |x| x + 3 ), 5);
	/// ```
	/// Also works when `self` is a reference
	/// ```
	/// # use crate::sugaru::kotlin::KotlinExt;
	/// assert_eq!("Hello".lets(str::to_uppercase), "HELLO");
	/// ```
	#[inline]
	fn lets<R>(self, function: impl FnOnce(Self) -> R) -> R {
		function(self)
	}

	/// Inspired by https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/also.html
	/// # Examples
	/// ```
	/// # use crate::sugaru::kotlin::KotlinExt;
	/// let remove_last_char: fn(&mut String) = | string | { string.pop(); };
	/// let string = String::from("Hello World!");
	/// let mut string = string.also(remove_last_char);
	/// assert_eq!(string, "Hello World");
	/// // Also works when `self` is a reference
	/// let _: &mut String = (&mut string).also(remove_last_char);
	/// assert_eq!(string, "Hello Worl");
	/// ```
	#[inline]
	fn also<SB, F>(mut self, function: F) -> Self
	where
		SB: ?Sized,
		Self: BorrowMut<SB>,
		F: FnOnce(&mut SB),
	{
		function(self.borrow_mut());
		self
	}

	/// Inspired by https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/take-if.html
	/// # Examples
	/// ```
	/// # use crate::sugaru::kotlin::KotlinExt;
	/// let is_even: fn (&i32) -> bool = |&n| n % 2 == 0;
	/// assert_eq!(2.take_if(is_even), Some(2));
	/// assert_eq!(3.take_if(is_even), None);
	/// ```
	/// With borrow:
	/// ```
	/// # use crate::sugaru::kotlin::KotlinExt;
	/// assert_eq!("Rustacean".to_string().take_if(str::is_ascii), Some(String::from("Rustacean")));
	/// assert_eq!("Rustacé".to_string().take_if(str::is_ascii), None);
	/// ```
	#[inline]
	fn take_if<SB, F>(self, predicate: F) -> Option<Self>
	where
		SB: ?Sized,
		Self: Borrow<SB>,
		F: FnOnce(&SB) -> bool,
	{
		self.borrow().lets(predicate).then_some(self)
	}

	/// Inspired by https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/take-unless.html
	/// # Examples
	/// ```
	/// # use crate::sugaru::kotlin::KotlinExt;
	/// assert_eq!("Rustacean".take_unless(str::is_ascii), None);
	/// assert_eq!("Rustacé".take_unless(str::is_ascii), Some("Rustacé"));
	/// ```
	/// With borrow:
	/// ```
	/// # use crate::sugaru::kotlin::KotlinExt;
	/// assert_eq!("Rustacean".to_string().take_unless(str::is_ascii), None);
	/// assert_eq!("Rustacé".to_string().take_unless(str::is_ascii), Some(String::from("Rustacé")));
	/// ```
	#[inline]
	fn take_unless<SB, F>(self, predicate: F) -> Option<Self>
	where
		SB: ?Sized,
		Self: Borrow<SB>,
		F: FnOnce(&SB) -> bool,
	{
		if !predicate(self.borrow()) {
			Some(self)
		} else {
			None
		}
	}
}

impl<T> KotlinExt for T {}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn also_test() {
		let vec = Vec::new().also(|it: &mut Vec<i32>| {
			it.push(1);
			it.push(2);
			it.push(3);
		});
		assert_eq!(vec, vec![1, 2, 3]);

		let tuple = (1, 2).also(|it| it.1 = 3);
		assert_eq!(tuple, (1, 3));
	}
}
