/// Syntaxe alternative pour écrire des `lambdas`
/// # Exemples
/// ```
/// # use sugaru::lambda;
/// let add_5 = lambda!(x => x + 5);
/// assert_eq!(add_5(3), 8);
/// ```
/// ```
/// # use sugaru::lambda;
/// let add_6: fn(i32) -> i32 = lambda!(x: i32 => {
///     let add = 6;
///     x + add
/// });
/// assert_eq!(add_6(3), 9);
/// ```
#[macro_export]
macro_rules! lambda {
	($param:ident $(: $param_type:ty)? => $body: expr) =>
		{ | $param $(: $param_type)? | $body };
}

/// Syntaxe alternative pour écrire des `closures`
/// # Exemples
/// ```
/// # use sugaru::closure;///
/// let add_5 = closure!(x => x + 5);
/// assert_eq!(add_5(3), 8);
/// ```
/// ```
/// # use sugaru::closure;
/// let sept = 7;
/// let add_7 = closure!(x: i32 => x + sept);
/// assert_eq!(add_7(10), 17);
/// ```
/// ```
/// # use sugaru::closure;
/// let add = closure!(x => y => { x + y });
/// assert_eq!(add(4)(5), 9);
///
/// let add_20 = add(20);
/// assert_eq!(add_20(13), 33);
/// ```
#[macro_export]
macro_rules! closure {
    ($param:ident $(: $param_type:ty)? => $body: expr) =>
		{ move | $param $(: $param_type)? | $body };
    ($a:ident $(: $a_type:ty )? => $($x:ident $(: $x_type:ty)? => )+ $body:expr) =>
		{ move | $a $(: $a_type )? | closure!($( $x $(: $x_type )? => )+ $body) };
}

/// One liner pour instancier un [std::collections::HashMap]
/// # Examples
///```
/// # use sugaru::map;
/// let map = map! {
///     1 => "un",
///     2 => "deux",
///     3 => "trois"
/// };
/// assert_eq!(map.len(), 3);
/// assert_eq!(map[&1], "un");
/// assert_eq!(map[&2], "deux");
/// assert_eq!(map[&3], "trois");
///```
#[macro_export]
macro_rules! map {
	($($key:expr => $value:expr),*) => {{
		let mut map = std::collections::HashMap::new();
		$(map.insert($key, $value);)*
		map
	}}
}

/// One liner pour instancier un [std::collections::HashSet]
/// # Examples
/// ```
/// # use sugaru::set;
/// let set = set! {
///     "un",
///     "deux",
///     "trois"
/// };
/// assert_eq!(set.len(), 3);
/// assert!(set.contains("un"));
/// assert!(set.contains("deux"));
/// assert!(set.contains("trois"));
/// ```
#[macro_export]
macro_rules! set {
	($($element:expr),*) => {{
		let mut set = std::collections::HashSet::new();
		$(set.insert($element);)*
		set
	}}
}

/// Rend une variable immuable
/// # Exemple
/// ```compile_fail,E0384
/// # use sugaru::freeze;
/// let mut a = 3;
/// a = 4;
/// freeze!(a);
/// a = 5; // cannot assign twice to immutable variable
/// ```
#[macro_export]
macro_rules! freeze {
	($variable: ident) => {
		let $variable = $variable;
	};
}

/// Rend une variable mutable
/// # Exemple
///```
/// # use sugaru::make_mutable;
/// let vec = vec![1, 2, 3];
/// make_mutable!(vec);
/// vec.push(4);
/// ```
#[macro_export]
macro_rules! make_mutable {
	($variable: ident) => {
		let mut $variable = $variable;
	};
}

#[macro_export]
macro_rules! flat_mod {
	($mod_name: ident) => {
		mod $mod_name;
		pub use $mod_name::*;
	};
}

/// Inspiré du pipeline operator `|>` que l'on trouve dans certains [langages fonctionnel](https://cs3110.github.io/textbook/chapters/hop/pipelining.html)
///
/// Chaine une valeur avec un appel de fonction
/// # Exemples
/// ```
/// # use sugaru::pipeline;
/// fn increment(n: i32) -> i32 { n + 1 }
/// fn double(n: i32) -> i32 { n * 2 }
/// let i = pipeline!(5 |> increment |> double |> increment);
/// assert_eq!(i, 13);
///
/// let boxed_twelve = pipeline!(2 + 3 => increment => double => Box::new);
/// assert_eq!(boxed_twelve, Box::new(12));
///
/// pipeline!(
///     (1 + 2 + 3) // Il faut délimiter le token tree
///     |> increment
///     |> double
///     |> Box::new
/// );
///
/// pipeline!(
///     1 + 2 + 3 + 4 // On peut utiliser une expression dans cette version avec les =>
///     => increment
///     => double
///     => increment
/// );
/// ```
/// ```compile_fail
/// # use sugaru::pipeline;
/// # fn increment(n: i32) -> i32 { n + 1 }
/// # fn double(n: i32) -> i32 { n * 2 }
/// pipeline!(
///     1 + 2 + 3 // Pas un token tree
///     |> increment
///     |> double
///     |> Box::new
/// );
/// ```
#[macro_export]
macro_rules! pipeline  {
    ($value:tt |> $function:path ) => { $function($value) };
	($value:tt |> $first_operation:path $(|> $function:path)+) => { pipeline!(($first_operation($value)) $(|> $function)+) };
    ($value:expr => $function:expr ) => { $function($value) };
	($value:expr => $first_operation:expr $(=> $function:expr)+) => { pipeline!($first_operation($value) $(=> $function)+) };
}

/// Ressemble au `throw` de certains langages, mais en plus rusty
///
/// Inspiré de <https://doc.rust-lang.org/beta/unstable-book/language-features/yeet-expr.html>
/// # Exemples
/// ```
/// # use sugaru::yeet;
/// fn divide_by_two(n: i32) -> Result<i32, String> {
///     if n % 2 != 0 {
///         yeet!(format!("Cannot divide {n} by 2"));
///     }
///     Ok(n / 2)
/// }
/// assert_eq!(divide_by_two(7), Err("Cannot divide 7 by 2".to_string()))
/// ```
#[macro_export]
macro_rules! yeet {
	($error:expr) => {
		return Err($error)
	};
}

///
/// # Exemples
/// ```
/// # use sugaru::skip_none;
/// let mut vec = vec![];
/// for option in [Some(1), None, Some(3), None] {
///     let int = skip_none!(option);
///     vec.push(int);
/// }
/// assert_eq!(vec.len(), 2);
/// assert_eq!(vec[0], 1);
/// assert_eq!(vec[1], 3);
/// ```
#[macro_export]
macro_rules! skip_none {
	($option:expr) => {
		match $option {
			None => continue,
			Some(value) => value,
		}
	};
}

///
/// Initialise un vec en recalculant l'expression autant de fois que nécessaire
///
/// Contrairement à `vec![toto(); 3]`, on ne clone pas le résultat de `toto()` mais on rappelle `toto()`
/// # Exemples
/// ```
/// # use sugaru::init_vec;
/// struct NoCopy(i32);
/// let mut nb_of_calls: u16 = 0;
/// fn new_no_copy(nb_of_calls: &mut u16, i: i32) -> NoCopy {
///     *nb_of_calls += 1;
///     NoCopy(i)
/// }
/// let vec = init_vec![new_no_copy(&mut nb_of_calls, 42); 3];
/// assert_eq!(nb_of_calls, 3);
/// assert_eq!(vec.len(), 3);
/// assert_eq!(vec[0].0, 42);
/// assert_eq!(vec[1].0, 42);
/// assert_eq!(vec[2].0, 42);
///
/// ```
#[macro_export]
macro_rules! init_vec {
	($init_expression:expr; $length: expr) => {{
		let length = $length;
		let mut vec = Vec::with_capacity(length);
		for _ in 0..length {
			vec.push($init_expression);
		}
		vec
	}};
}

/// # Exemples
/// ```
/// # use sugaru::until;
/// let mut i = 0;
/// let mut n = 0;
/// until!((i > 2) {
///     i += 1;
///     n = (n + 1) * 2
/// });
/// assert_eq!(i, 3);
/// assert_eq!(n, 14);
/// ```
#[macro_export]
macro_rules! until {
	($condition:tt $body:tt) => {
		while !$condition {
			$body
		}
	};
	($condition:tt do $body:tt) => {
		until!($condition $body)
	};
}

/// # Exemples
/// ```
/// # use sugaru::does;
/// let mut i = 0;
/// let mut n = 0;
/// does!({
///     i += 1;
///     n = (n + 1) * 2;
/// } while i < 3);
/// assert_eq!(i, 3);
/// assert_eq!(n, 14);
/// ```
/// ```
/// # use sugaru::does;
/// let mut i = 0;
/// let mut n = 0;
/// does!({
///     i += 1;
///     n = (n + 1) * 2;
/// } until i == 3);
/// assert_eq!(i, 3);
/// assert_eq!(n, 14);
/// ```
/// ```
/// # use sugaru::does;
/// let mut n = 0;
/// does!((n = 1) unless true);
/// assert_eq!(n, 0);
/// does!((n = 1) unless false);
/// assert_eq!(n, 1);
/// ```
#[macro_export]
macro_rules! does {
	($body:tt while $condition:expr) => {
		does!($body until !$condition)
	};
	($body:tt until $condition:expr) => {
		loop {
			$body
			if $condition {
				break;
			}
		}
	};
	($todo:tt unless $condition:expr) => {
		if !$condition {
			$todo
		}
	};
}

/// # Exemples
/// ```
/// # use sugaru::unless;
/// let mut n = 0;
/// unless!(true {
///     n = 1;
/// });
/// assert_eq!(n, 0);
/// unless!((1 == 2) {
///     n = 1;
/// });
/// assert_eq!(n, 1);
/// ```
/// ```
/// # use sugaru::unless;
/// let mut n = 0;
/// unless!(true {
///     n = 1;
/// } else {
///     n = 2;
/// });
/// assert_eq!(n, 2);
/// ```
#[macro_export]
macro_rules! unless {
	($condition:tt $body:tt) => {
		if !$condition {
			$body
		}
	};
	($condition:tt $body:tt else $else_body:tt) => {
		if $condition {
			$else_body
		} else {
			$body
		}
	};
}

/// Fonctionne que avec [Result] mais pourra fonctionner avec toutes les implémentations de [core::ops::Try]
/// quand `try_trait_v2` sera stabilisé
/// # Exemples
/// ```
/// # use sugaru::tries;
/// fn attempt() -> Result<i32, String> {
///     Ok(7)
/// }
///
/// fn transform(n: i32) -> Result<f32, String> {
///     Ok(n as f32 / 2.0)
/// }
///
/// let result: Result<f32, String> = tries!({
///     let int: i32 = attempt()?;
///     let float: f32 = transform(int)?;
///     float
/// });
/// assert_eq!(result, Ok(3.5));
/// ```
#[macro_export]
macro_rules! tries {
	// Dès que try_trait_v2 est stabilisé
	// (|| core::ops::Try::from_output($to_try))()
	($to_try:expr) => {
		(|| Ok($to_try))()
	};
}
