#![no_std]

use higher_order_functions::Init;

#[cfg(feature = "std")]
#[macro_use]
extern crate std;
#[cfg(feature = "std")]
use std::{vec::Vec, collections::LinkedList};

#[cfg(all(not(feature = "std"), feature = "alloc"))]
#[macro_use]
extern crate alloc;
#[cfg(all(not(feature = "std"), feature = "alloc"))]
use alloc::{vec::Vec, collections::LinkedList};

#[test]
fn init_empty_array() {
	let arr = <[usize; 0]>::init(|_| panic!("Shouldn't call init function"));
	assert_eq!(arr, []);
}

#[test]
fn init_singleton_array() {
	let mut called = false;
	let arr = <[usize; 1]>::init(|i| {
		assert_eq!(i, 0);
		if called { panic!("Should only call init function once"); }
		else { called = true; }
		123
	});
	assert!(called);
	assert_eq!(arr, [123]);
}

#[test]
fn init_array() {
	let arr = <[usize; 123]>::init(|i| i);
	for i in 0..123 {
		assert_eq!(arr[i], i);
	}
}

#[test]
fn init_2d_array() {
	let arr = <[_; 12]>::init(|x| <[_; 34]>::init(|y| (x, y)));
	for x in 0..12 {
		for y in 0..34 {
			assert_eq!(arr[x][y], (x, y));
		}
	}
}

#[cfg(any(feature = "std", feature = "alloc"))]
#[test]
fn init_empty_vec() {
	let vec = Vec::init_with(0, |_| panic!("Shouldn't call init function"));
	assert_eq!(vec, vec![]);
}

#[cfg(any(feature = "std", feature = "alloc"))]
#[test]
fn init_singleton_vec() {
	let mut called = false;
	let vec = Vec::init_with(1, |i| {
		assert_eq!(i, 0);
		if called { panic!("Should only call init function once"); }
		else { called = true; }
		123
	});
	assert!(called);
	assert_eq!(vec, vec![123]);
}

#[cfg(any(feature = "std", feature = "alloc"))]
#[test]
fn init_vec() {
	let vec = Vec::init_with(123, |i| i);
	assert_eq!(vec.len(), 123);
	for i in 0..123 {
		assert_eq!(vec[i], i);
	}
}

#[cfg(any(feature = "std", feature = "alloc"))]
#[test]
fn init_empty_linked_list() {
	let list = LinkedList::init_with(0, |_| panic!("Shouldn't call init function"));
	assert_eq!(list, LinkedList::new());
}

#[cfg(any(feature = "std", feature = "alloc"))]
#[test]
fn init_singleton_linked_list() {
	let mut called = false;
	let list = LinkedList::init_with(1, |i| {
		assert_eq!(i, 0);
		if called { panic!("Should only call init function once"); }
		else { called = true; }
		123
	});
	assert!(called);
	assert_eq!(list, {let mut l = LinkedList::new(); l.push_back(123); l});
}

#[cfg(any(feature = "std", feature = "alloc"))]
#[test]
fn init_linked_list() {
	let list = LinkedList::init_with(123, |i| i);
	assert_eq!(list.len(), 123);
	for i in 0..123 {
		assert_eq!(list.iter().nth(i), Some(&i));
	}
}

#[cfg(any(feature = "std", feature = "alloc"))]
#[test]
fn init_empty_linked_list_unfold() {
	let mut called = false;
	let list = LinkedList::<()>::init(|i| {
		assert_eq!(i, 0);
		if called { panic!("Should only call init function once"); }
		else { called = true; }
		None
	});
	assert_eq!(list, LinkedList::new());
}

#[cfg(any(feature = "std", feature = "alloc"))]
#[test]
fn init_singleton_linked_list_unfold() {
	let mut called = 0;
	let list = LinkedList::<usize>::init(|i| {
		assert_eq!(i, called);
		called += 1;
		match called {
			1 => Some(123),
			2 => None,
			_ => panic!("Should only call init function twice"),
		}
	});
	assert_eq!(list, {let mut l = LinkedList::new(); l.push_back(123); l});
}

#[cfg(any(feature = "std", feature = "alloc"))]
#[test]
fn init_linked_list_unfold() {
	let list = LinkedList::init(|i| if i < 123 { Some(i) } else { None });
	assert_eq!(list.len(), 123);
	for i in 0..123 {
		assert_eq!(list.iter().nth(i), Some(&i));
	}
}
