
use std::sync::Arc;

/// Alias pour [u8], type privilégié pour représenter un octet
/// ```
/// # use util::type_alias::byte;
/// let x: &[byte] = "Hello world".as_bytes();
/// ```
#[allow(non_camel_case_types)]
pub type byte = u8;

/// A UTF-8–encoded, heap allocated, immutable string
/// # Examples
/// ```
/// # use util::type_alias::ImmutString;
/// let string = ImmutString::from("Hello");
/// ```
/// ```
/// # use util::type_alias::ImmutString;
/// let string: ImmutString = String::from("World").into_boxed_str();
/// ```
/// motivé par <https://www.youtube.com/watch?v=A4cKi7PTJSs>
pub type ImmutString = Box<str>;

/// A UTF-8–encoded, heap allocated, immutable string optimized for cloning
/// # Example
/// ```
/// # use util::type_alias::CloneStr;
/// let string = CloneStr::from("Hello world");
/// ```
/// motivé par <https://www.youtube.com/watch?v=A4cKi7PTJSs>
pub type CloneStr = Arc<str>;

type BoxedSlice<T> = Box<[T]>;

/// 🇫🇷 Tableau alloué sur le tas, comme [Vec], mais en plus compact et non conçu pour être modifié
///
/// 🇬🇧 Heap allocated array, just like [Vec], but more compact and not designed to be updated
///
/// # Examples
///
/// ```
/// # use util::type_alias::ImmutVec;
/// let vec: ImmutVec<i32> = ImmutVec::from([1, 2, 3]);
/// ```
/// ```
/// # use util::type_alias::ImmutVec;
/// let vec: ImmutVec<i32> = vec![1, 2, 3].into_boxed_slice();
/// ```
/// motivé par <https://www.youtube.com/watch?v=A4cKi7PTJSs>
pub type ImmutVec<T> = BoxedSlice<T>;

/// Tableau alloué sur le tas, comme [Vec], mais
/// * plus compact
/// * non conçu pour être modifié
/// * optimisé pour être cloné
///
/// # Exemple
/// ```
/// # use util::type_alias::CloneSlice;
/// let vec: CloneSlice<i32> = CloneSlice::from(&[1, 2, 3][..]);
/// ```
/// motivé par <https://www.youtube.com/watch?v=A4cKi7PTJSs>
pub type CloneSlice<T> = Arc<[T]>;
