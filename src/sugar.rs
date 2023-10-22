
#[macro_export]
macro_rules! lambda {
	($param:ident $(: $param_type:ty)? => $body: expr) => {
		| $param $(: $param_type)? | $body
	};
}

#[macro_export]
macro_rules! closure {
    ($param:ident $(: $param_type:ty)? => $body: expr) => {
		move | $param $(: $param_type)? | $body
	};
    ($a:ident $(: $a_type:ty )? => $($x:ident $(: $x_type:ty)? => )+ $body:expr) => {
        move | $a $(: $a_type )? | closure!($( $x $(: $x_type )? => )+ $body)
    };
}

/// # Examples
///```
/// # use util::map;
/// let map = map! {
/// 	1 => "un",
/// 	2 => "deux",
/// 	3 => "trois"
/// };
///```
#[macro_export]
macro_rules! map {
	($($key:expr => $value:expr),*) => {{
		let mut map = std::collections::HashMap::new();
		$(map.insert($key, $value);)*
		map
	}}
}

/// # Examples
/// ```
/// # use util::set;
/// let set = set! {
/// 	"un",
/// 	"deux",
/// 	"trois"
/// };
/// ```
#[macro_export]
macro_rules! set {
	($($element:expr),*) => {{
		let mut set = std::collections::HashSet::new();
		$(set.insert($element);)*
		set
	}}
}

/// ```compile_fail,E0384
/// # use util::freeze;
/// let mut a = 3;
/// a = 4;
/// freeze!(a);
/// a = 5;
/// ```
#[macro_export]
macro_rules! freeze {
	($variable: ident) => { let $variable = $variable; }
}

/// # Examples
///```
/// # use util::make_mutable;
/// let vec = vec![1, 2, 3];
/// make_mutable!(vec);
/// vec.push(4);
/// ```
#[macro_export]
macro_rules! make_mutable {
	($variable: ident) => { let mut $variable = $variable; }
}

#[macro_export]
macro_rules! flat_mod {
    ($mod_name: ident) => {
		mod $mod_name;
		pub use $mod_name::*;
	};
}

/// Inspiré du pipeline operator `|>` que l'on trouve dans certains [langages fonctionnel](https://cs3110.github.io/textbook/chapters/hop/pipelining.html)
/// # Exemples
/// ```
/// # use util::pipeline;
/// fn increment(n: i32) -> i32 { n + 1 }
/// fn double(n: i32) -> i32 { n * 2 }
/// let i = pipeline!(5 |> increment |> double |> increment);
/// let boxed_twelve = pipeline!(2 + 3 => increment => double => Box::new);
/// ```
#[macro_export]
macro_rules! pipeline  {
    ($value:tt |> $function:path ) => { $function($value) };
	($value:tt |> $first_operation:path $(|> $function:path)+) => { pipeline!(($first_operation($value)) $(|> $function)+) };
    ($value:expr => $function:path ) => { $function($value) };
	($value:expr => $first_operation:path $(=> $function:path)+) => { pipeline!($first_operation($value) $(=> $function)+) };
}

/// Inspiré de <https://doc.rust-lang.org/beta/unstable-book/language-features/yeet-expr.html>
/// Ressemble au `throw` de certains langages, mais en différent
/// # Exemples
/// ```
/// # use util::yeet;
/// fn divide_by_two(n: i32) -> Result<i32, &'static str> {
/// 	if n % 2 != 0 {
///         yeet!("Cannot divide an odd number by 2");
/// 	}
/// 	Ok(n / 2)
/// }
/// assert_eq!(divide_by_two(7), Err("Cannot divide an odd number by 2"))
/// ```
#[macro_export]
macro_rules! yeet {
    ($error:expr) => { return Err($error) }
}



#[cfg(test)]
mod tests {

	#[test]
	fn lambda() {
		let add_5 = lambda!(x => x + 5);
		assert_eq!(add_5(3), 8);

		let add_6: fn(i32) -> i32 = lambda!(x: i32 => {
			let add = 6;
			x + add
		});
		assert_eq!(add_6(3), 9);
	}

	#[test]
	fn closure() {
		let add_5 = closure!(x => x + 5);
		assert_eq!(add_5(3), 8);

		let add_6: fn(i32) -> i32 = closure!(x: i32 => {
			let add = 6;
			x + add
		});
		assert_eq!(add_6(3), 9);

		let add = closure!(x => y => { x + y });

		assert_eq!(add(4)(5), 9);

		let add_20 = add(20);
		assert_eq!(add_20(13), 33);

		let sept = 7;
		let add_7 = closure!(x: i32 => x + sept);
		assert_eq!(add_7(10), 17);
	}

	#[test]
	fn map() {
		let map = map! {
			1 => "un",
			2 => "deux",
			3 => "trois"
		};
		assert_eq!(map.len(), 3);
		assert_eq!(map[&1], "un");
		assert_eq!(map[&2], "deux");
	}

	#[test]
	fn set() {
		let set = set! {
			"un",
			"deux",
			"trois",
			"deux"
		};
		assert_eq!(set.len(), 3);
		assert!(set.contains("un"));
		assert!(set.contains("deux"));
	}

	#[test]
	fn make_mutable() {
		let vec = vec![1, 2, 3];
		make_mutable!(vec);
		vec.push(4);
	}

	#[test]
	fn pipeline() {
		fn increment(n: i32) -> i32 { n + 1 }
		fn double(n: i32) -> i32 { n * 2 }

		let result = pipeline!(5 |> increment |> double |> increment);
		assert_eq!(result, 13);
		let result = pipeline!(5 => increment => double => increment);
		assert_eq!(result, 13);
		let result = pipeline!(5
			|> increment
			|> double
			|> increment
		);
		assert_eq!(result, 13);
		let result = pipeline!(5
			=> increment
			=> double
			=> Box::new
		);
		assert_eq!(result, Box::new(12))
	}

}
