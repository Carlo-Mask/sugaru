
macro_rules! lambda {
	($param:ident $(: $param_type:ty)? => $body: expr) => {
		| $param $(: $param_type)? | $body
	};
    ($a:ident $(: $a_type:ty )? => $($x:ident $(: $x_type:ty)? => )+ $body:expr) => {
        | $a $(: $a_type )? | lambda!($( $x $(: $x_type )? => )+ $body)
    };
}

pub(crate) use lambda;

macro_rules! closure {
    ($param:ident $(: $param_type:ty)? => $body: expr) => {
		move | $param $(: $param_type)? | $body
	};
    ($a:ident $(: $a_type:ty )? => $($x:ident $(: $x_type:ty)? => )+ $body:expr) => {
        move | $a $(: $a_type )? | closure!($( $x $(: $x_type )? => )+ $body)
    };
}
pub(crate) use closure;

macro_rules! map {
	($($key:expr => $value:expr),*) => {{
		let mut map = std::collections::HashMap::new();
		$(
			map.insert($key, $value);
		)*
		map
	}}
}
pub(crate) use map;


macro_rules! freeze {
	($variable: ident) => { let $variable = $variable; }
}
pub(crate) use freeze;

macro_rules! make_mutable {
	($variable: ident) => { let mut $variable = $variable; }
}
pub(crate) use make_mutable;

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn lambda() {
		let add_5 = lambda!(x => x + 5);
		assert_eq!(add_5(3), 8);

		let add_6: fn(i32) -> i32 = lambda!(x: i32 => {
			let add = 6;
			x + add
		});
		assert_eq!(add_6(3), 9);

		// let add = lambda!(x => y => { x + y });
		//
		// assert_eq!(add(4)(5), 9);
		//
		// let add_20 = add(20);
		// assert_eq!(add_20(13), 33);
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
	fn freeze() {
		let mut _vec = vec![1, 2, 3];
		_vec.push(4);

		freeze!(_vec);

		// _vec.push(5);
	}

	#[test]
	fn make_mutable() {
		let vec = vec![1, 2, 3];
		make_mutable!(vec);
		vec.push(4);
	}

}
