#![no_std]

use higher_order_functions::Map;

#[cfg(feature = "std")]
#[macro_use]
extern crate std;

#[cfg(all(not(feature = "std"), feature = "alloc"))]
#[macro_use]
extern crate alloc;
#[cfg(all(not(feature = "std"), feature = "alloc"))]
use alloc::collections::LinkedList;

#[test]
fn map_empty_array() {
	let arr = [].map(|_: ()| panic!("Shouldn't call map function"));
	assert_eq!(arr, []);
}

#[test]
fn map_singleton_array() {
	let mut called = false;
	let arr = [321].map(|x| {
		assert_eq!(x, 321);
		if called { panic!("Should only call map function once"); }
		else { called = true; }
		123
	});
	assert!(called);
	assert_eq!(arr, [123]);
}

#[test]
fn double_array() {
	let arr = [1, 4, 6, -3, 6].map(|x| x * 2);
	
	assert_eq!(arr, [2, 8, 12, -6, 12]);
}

#[test]
fn cast_array() {
	let arr = [1, 4, 6, -3, 6].map(f64::from);
	
	assert_eq!(arr, [1.0, 4.0, 6.0, -3.0, 6.0]);
}

#[test]
fn map_2d_array() {
	let arr = [[1, 2, 3], [4, 5, 6]].map(|a| a.map(|x| x * 2));
	
	assert_eq!(arr, [[2, 4, 6], [8, 10, 12]]);
}

#[cfg(any(feature = "std", feature = "alloc"))]
#[test]
fn map_empty_vec() {
	let vec = vec![].map(|_: ()| panic!("Shouldn't call map function"));
	assert_eq!(vec, vec![]);
}

#[cfg(any(feature = "std", feature = "alloc"))]
#[test]
fn map_singleton_vec() {
	let mut called = false;
	let vec = vec![321].map(|x| {
		assert_eq!(x, 321);
		if called { panic!("Should only call map function once"); }
		else { called = true; }
		123
	});
	assert!(called);
	assert_eq!(vec, vec![123]);
}

#[cfg(any(feature = "std", feature = "alloc"))]
#[test]
fn double_vec() {
	let vec = vec![1, 4, 6, -3, 6].map(|x| x * 2);
	
	assert_eq!(vec, vec![2, 8, 12, -6, 12]);
}

#[cfg(any(feature = "std", feature = "alloc"))]
#[test]
fn cast_vec() {
	let vec = vec![1, 4, 6, -3, 6].map(f64::from);
	
	assert_eq!(vec, vec![1.0, 4.0, 6.0, -3.0, 6.0]);
}
