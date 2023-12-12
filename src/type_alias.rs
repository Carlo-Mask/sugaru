use std::sync::Arc;

/// Alias pour [u8], type privilégié pour représenter un octet
/// ```
/// # use sugaru::type_alias::byte;
/// let bytes: &[byte] = "Hello world".as_bytes();
/// ```
#[allow(non_camel_case_types)]
pub type byte = u8;

/// A UTF-8–encoded, heap allocated, immutable string
/// # Examples
/// ```
/// # use sugaru::type_alias::ImmutString;
/// let string = ImmutString::from("Hello");
/// ```
/// ```
/// # use sugaru::type_alias::ImmutString;
/// let string: ImmutString = String::from("World").into_boxed_str();
/// ```
/// motivé par <https://www.youtube.com/watch?v=A4cKi7PTJSs>
pub type ImmutString = Box<str>;

/// A UTF-8–encoded, heap allocated, immutable string optimized for cloning
/// # Example
/// ```
/// # use sugaru::type_alias::CloneStr;
/// let string = CloneStr::from("Hello world");
/// ```
/// motivé par <https://www.youtube.com/watch?v=A4cKi7PTJSs>
pub type CloneStr = Arc<str>;

/// 🇫🇷 Tableau alloué sur le tas, comme [Vec], mais en plus compact et non conçu pour être modifié
///
/// 🇬🇧 Heap allocated array, just like [Vec], but more compact and not designed to be updated
///
/// # Examples
/// ```
/// # use sugaru::type_alias::BoxedSlice;
/// let slice: BoxedSlice<i32> = vec![1, 2, 3].into_boxed_slice();
/// ```
/// motivé par <https://www.youtube.com/watch?v=A4cKi7PTJSs>
pub type BoxedSlice<T> = Box<[T]>;

/// Tableau alloué sur le tas, comme [Vec], mais
/// * plus compact
/// * non conçu pour être modifié
/// * optimisé pour être cloné
///
/// # Exemple
/// ```
/// # use sugaru::type_alias::CloneSlice;
/// let vec: CloneSlice<i32> = CloneSlice::from(&[1, 2, 3][..]);
/// ```
/// motivé par <https://www.youtube.com/watch?v=A4cKi7PTJSs>
pub type CloneSlice<T> = Arc<[T]>;
