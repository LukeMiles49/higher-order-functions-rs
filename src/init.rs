use crate::type_equals::TypeEquals;

use core::marker::Sized;
use core::mem::{MaybeUninit, transmute_copy, forget};

/// Types which can be initialised by applying a function to each 'index' of the type.
///
/// # Examples
///
/// ```rust
/// use higher_order_functions::Init;
///
/// struct House { number: usize }
///
/// // [T; N]: Init<T, usize>
/// let road = <[House; 3]>::init(|i| House { number: i + 1 });
///
/// assert_eq!(road[0].number, 1);
/// assert_eq!(road[1].number, 2);
/// assert_eq!(road[2].number, 3);
/// ```
pub trait Init<T, I, V = ()>: Sized {
	/// Initialise an instance of this type using `value` by applying `elem` to each 'index' of the type.
	///
	/// # Examples
	///
	#[cfg_attr(feature = "std", doc = r##"
	Constructing a Vec containing the values 0 to 4:
	
	```rust
	use higher_order_functions::Init;
	
	let vec = Vec::<usize>::init_with(5, |i| i);
	
	assert_eq!(vec, vec![0, 1, 2, 3, 4]);
	```
	"##)]
	fn init_with<F: FnMut(I) -> T>(value: V, elem: F) -> Self;
	
	/// Initialise an instance of this type by applying `elem` to each 'index' of the type.
	///
	/// This is syntax sugar for `init_with((), elem)`.
	///
	/// # Examples
	///
	/// Constructing an array containing the values 0 to 4:
	///
	/// ```rust
	/// use higher_order_functions::Init;
	///
	/// let arr = <[usize; 5]>::init(|i| i);
	///
	/// assert_eq!(arr, [0, 1, 2, 3, 4]);
	/// ```
	fn init<F: FnMut(I) -> T>(elem: F) -> Self where V: TypeEquals<()> {
		Self::init_with(().into(), elem)
	}
}

/// Initialise an array from a function applied to each index.
impl<T, const N: usize> Init<T, usize> for [T; N] {
	fn init_with<F: FnMut(usize) -> T>(_: (), mut elem: F) -> Self {
		let mut contents: [MaybeUninit<T>; N] = MaybeUninit::uninit_array();
		
		for i in 0..N {
			contents[i] = MaybeUninit::new(elem(i));
		}
		
		// FIXME: Replace with transmute once it works with const generic array sizes
		let res = unsafe { transmute_copy(&contents) };
		forget(contents);
		res
	}
}

/// Initialise a Vec from a length and a function applied to each index.
#[cfg(any(feature = "std", feature = "alloc"))]
impl<T> Init<T, usize, usize> for lib::vec::Vec<T> {
	fn init_with<F: FnMut(usize) -> T>(length: usize, mut elem: F) -> Self {
		let mut value = lib::vec::Vec::with_capacity(length);
		
		for i in 0..length {
			value.push(elem(i));
		}
		
		value
	}
}

/// Initialise a LinkedList from a length and a function applied to each index.
#[cfg(any(feature = "std", feature = "alloc"))]
impl<T> Init<T, usize, usize> for lib::collections::LinkedList<T> {
	fn init_with<F: FnMut(usize) -> T>(length: usize, mut elem: F) -> Self {
		let mut value = lib::collections::LinkedList::new();
		
		for i in 0..length {
			value.push_back(elem(i));
		}
		
		value
	}
}

/// Initialise a LinkedList from a function called repeatedly until it returns None.
///
/// This is equivalent to an unfold but with a mutable closure instead of a mutable state parameter.
#[cfg(any(feature = "std", feature = "alloc"))]
impl<T> Init<Option<T>, usize> for lib::collections::LinkedList<T> {
	fn init_with<F: FnMut(usize) -> Option<T>>(_: (), mut elem: F) -> Self {
		let mut value = lib::collections::LinkedList::new();
		
		while let Some(x) = elem(value.len()) {
			value.push_back(x);
		}
		
		value
	}
}
