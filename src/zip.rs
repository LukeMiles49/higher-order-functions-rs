use crate::helpers::as_maybe_uninit_array;

use core::mem::{MaybeUninit, transmute_copy};

/// Zip two collections using a function to combine pairs of elements.
///
/// # Examples
///
/// Zipping two arrays of different types into a single array of tuples:
///
/// ```rust
/// use higher_order_functions::Zip;
///
/// let a = [1, 2, 3];
/// let b = ["a", "b", "c"];
///
/// let arr = a.zip(b, |ax, bx| (ax, bx));
///
/// assert_eq!(arr, [(1, "a"), (2, "b"), (3, "c")]);
/// ```
///
/// Zipping two arrays by multiplying each pair of elements:
///
/// ```rust
/// use higher_order_functions::Zip;
///
/// let a = [1, 2, 3];
/// let b = [4, 5, 6];
///
/// let arr = a.zip(b, |ax, bx| ax * bx);
///
/// assert_eq!(arr, [4, 10, 18]);
/// ```
pub trait Zip {
	/// The type parameter of `Self`.
	///
	/// `Self: TSelf<TLhs>`
	type TLhs;
	
	/// The generic type of `Self`.
	///
	/// `Self: TSelf<TLhs>`
	type TSelf<T>;
	
	/// Apply `f` to each pair of elements in `self` and `rhs` and return the results.
	fn zip<TRhs, TTo, F: FnMut(Self::TLhs, TRhs) -> TTo>(self, rhs: Self::TSelf<TRhs>, f: F) -> Self::TSelf<TTo>;
}

/// Zip two arrays using a function to combine pairs of elements.
impl<TLhs, const N: usize> Zip for [TLhs; N] {
	type TLhs = TLhs;
	type TSelf<T> = [T; N];
	
	fn zip<TRhs, TTo, F: FnMut(Self::TLhs, TRhs) -> TTo>(self, rhs: Self::TSelf<TRhs>, mut f: F) -> Self::TSelf<TTo> {
		let consumed_lhs: [MaybeUninit<TLhs>; N] = as_maybe_uninit_array(self);
		let consumed_rhs: [MaybeUninit<TRhs>; N] = as_maybe_uninit_array(rhs);
		let mut contents: [MaybeUninit<TTo>; N] = MaybeUninit::uninit_array();
		
		for i in 0..N {
			contents[i].write(f(unsafe { consumed_lhs[i].read() }, unsafe { consumed_rhs[i].read() }));
		}
		
		// FIXME (#61956): Replace with transmute once it works with const generic array sizes
		unsafe { transmute_copy(&contents) }
	}
}
