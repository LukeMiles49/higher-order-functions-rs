use core::mem::{MaybeUninit, transmute_copy, forget};

pub fn as_maybe_uninit_array<T, const N: usize>(array: [T; N]) -> [MaybeUninit<T>; N] {
	// FIXME (#61956): Replace with transmute once it works with const generic array sizes
	let result = unsafe { transmute_copy(&array) };
	forget(array);
	result
}

// FIXME (#20041): Replace this workaround with real type equality constraints
pub trait TypeEquals<T>: From<T> { }
impl<T> TypeEquals<T> for T { }
