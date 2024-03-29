//! A small collection of traits for implementing higher order functions.
//!
//! # Init:
//!
//! ```rust
//! use higher_order_functions::Init;
//!
//! struct House { number: usize }
//!
//! // [T; N]: Init<T, usize>
//! let road = <[House; 3]>::init(|i| House { number: i + 1 });
//!
//! assert_eq!(road[0].number, 1);
//! assert_eq!(road[1].number, 2);
//! assert_eq!(road[2].number, 3);
//! ```
//!
//! # Map:
//!
//! ```rust
//! use higher_order_functions::Map;
//!
//! let arr = [1, 4, 6, -3, 6].map(|x| x * 2);
//!
//! assert_eq!(arr, [2, 8, 12, -6, 12]);
//! ```
//!
//! ```rust
//! use higher_order_functions::Map;
//!
//! let arr = [1, 4, 6, -3, 6].map(f64::from);
//!
//! assert_eq!(arr, [1.0, 4.0, 6.0, -3.0, 6.0]);
//! ```
//!
//! # Zip:
//!
//! ```rust
//! use higher_order_functions::Zip;
//!
//! let a = [1, 2, 3];
//! let b = ["a", "b", "c"];
//!
//! let arr = a.zip_with(b, |ax, bx| (ax, bx));
//!
//! assert_eq!(arr, [(1, "a"), (2, "b"), (3, "c")]);
//! ```
//!
//! ```rust
//! use higher_order_functions::Zip;
//!
//! let a = [1, 2, 3];
//! let b = [4, 5, 6];
//!
//! let arr = a.zip_with(b, |ax, bx| ax * bx);
//!
//! assert_eq!(arr, [4, 10, 18]);
//! ```
//!
//! # Section:
//!
//! ```rust
//! use higher_order_functions::Section;
//!
//! let a: [u32; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
//!
//! let arr: [u32; 4] = a.section(3); // Extracts 4 elements starting at a[3]
//!
//! assert_eq!(arr, [4, 5, 6, 7]);
//! ```
//!
//! To use this, add it as a dependency to your Cargo.toml:
//! ```toml
//! [dependencies]
//! higher_order_functions = "^0.2.0"
//! ```

#![no_std]

#![feature(generic_associated_types)]
#![feature(maybe_uninit_array_assume_init)]
#![feature(maybe_uninit_uninit_array)]

#![doc(html_root_url = "https://docs.rs/higher_order_functions/0.2.0")]

#[cfg(feature = "std")]
extern crate std as lib;

#[cfg(all(not(feature = "std"), feature = "alloc"))]
extern crate alloc as lib;

mod helpers;
mod init;
mod map;
mod zip;
mod section;

pub use {
	init::{
		Init,
	},
	map::{
		Map,
	},
	zip::{
		Zip,
	},
	section::{
		Section,
	},
};

// Include the readme and changelog as hidden documentation so they're tested by cargo test
#[doc = include_str!("../README.md")]
#[doc = include_str!("../CHANGELOG.md")]
type _Doctest = ();
