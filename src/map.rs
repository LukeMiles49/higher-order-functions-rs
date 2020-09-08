use crate::helpers::as_maybe_uninit_array;

use core::mem::{MaybeUninit, transmute_copy};
use core::hash::Hash;

/// Types containing 'inner' values which can be mapped over.
///
/// # Examples
///
/// Multiplying an array by 2:
///
/// ```rust
/// use higher_order_functions::Map;
///
/// let arr = [1, 4, 6, -3, 6].map(|x| x * 2);
///
/// assert_eq!(arr, [2, 8, 12, -6, 12]);
/// ```
///
/// Converting an i32 array to an f64 array:
///
/// ```rust
/// use higher_order_functions::Map;
///
/// let arr = [1, 4, 6, -3, 6].map(f64::from);
///
/// assert_eq!(arr, [1.0, 4.0, 6.0, -3.0, 6.0]);
/// ```
pub trait Map {
	/// The type of values being mapped over.
	type TFrom;
	
	/// The generic type of the result after mapping.
	type TOut<TTo>;
	
	/// Apply a function to the inner values of this type.
	fn map<TTo, F: FnMut(Self::TFrom) -> TTo>(self, f: F) -> Self::TOut<TTo>;
}

/// Apply a function to all elements of the array.
impl<TFrom, const N: usize> Map for [TFrom; N] {
	type TFrom = TFrom;
	type TOut<TTo> = [TTo; N];
	
	fn map<TTo, F: FnMut(TFrom) -> TTo>(self, mut f: F) -> Self::TOut<TTo> {
		let consumed: [MaybeUninit<TFrom>; N] = as_maybe_uninit_array(self);
		let mut contents: [MaybeUninit<TTo>; N] = MaybeUninit::uninit_array();
		
		for i in 0..N {
			contents[i].write(f(unsafe { consumed[i].read() }));
		}
		
		// FIXME (#61956): Replace with transmute once it works with const generic array sizes
		unsafe { transmute_copy(&contents) }
	}
}

/// Apply a function to all elements of the Vec.
#[cfg(any(feature = "std", feature = "alloc"))]
impl<TFrom> Map for lib::vec::Vec<TFrom> {
	type TFrom = TFrom;
	type TOut<TTo> = lib::vec::Vec<TTo>;
	
	fn map<TTo, F: FnMut(TFrom) -> TTo>(self, f: F) -> Self::TOut<TTo> {
		self.into_iter().map(f).collect()
	}
}

/// Apply a function to all values of the BTreeMap.
#[cfg(any(feature = "std", feature = "alloc"))]
impl<TKey, TFrom> Map for lib::collections::BTreeMap<TKey, TFrom> where TKey: Ord {
	type TFrom = TFrom;
	type TOut<TTo> = lib::collections::BTreeMap<TKey, TTo>;
	
	fn map<TTo, F: FnMut(TFrom) -> TTo>(self, mut f: F) -> Self::TOut<TTo> {
		self.into_iter().map(|(k, v)| (k, f(v))).collect()
	}
}

/// Apply a function to all values of the HashMap.
#[cfg(any(feature = "std"))]
impl<TKey, TFrom> Map for lib::collections::HashMap<TKey, TFrom> where TKey: Hash + Eq {
	type TFrom = TFrom;
	type TOut<TTo> = lib::collections::HashMap<TKey, TTo>;
	
	fn map<TTo, F: FnMut(TFrom) -> TTo>(self, mut f: F) -> Self::TOut<TTo> {
		self.into_iter().map(|(k, v)| (k, f(v))).collect()
	}
}

/// Apply a function to all values of the LinkedList.
#[cfg(any(feature = "std", feature = "alloc"))]
impl<TFrom> Map for lib::collections::LinkedList<TFrom> {
	type TFrom = TFrom;
	type TOut<TTo> = lib::collections::LinkedList<TTo>;
	
	fn map<TTo, F: FnMut(TFrom) -> TTo>(self, f: F) -> Self::TOut<TTo> {
		self.into_iter().map(f).collect()
	}
}

/// Apply a function to all values of the VecDeque.
#[cfg(any(feature = "std", feature = "alloc"))]
impl<TFrom> Map for lib::collections::VecDeque<TFrom> {
	type TFrom = TFrom;
	type TOut<TTo> = lib::collections::VecDeque<TTo>;
	
	fn map<TTo, F: FnMut(TFrom) -> TTo>(self, f: F) -> Self::TOut<TTo> {
		self.into_iter().map(f).collect()
	}
}
