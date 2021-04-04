use super::Init;

/// Operator to retrieve a fixed-size section of an array-like structure.
///
/// # Examples
///
/// Extracting elements from an array:
///
/// ```rust
/// use higher_order_functions::Section;
///
/// let a: [u32; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
///
/// let arr: [u32; 4] = a.section(3); // Extracts 4 elements starting at a[3]
///
/// assert_eq!(arr, [4, 5, 6, 7]);
/// ```
pub trait Section<TOffset, TOut> {
	/// Get a section of type `TOut` starting at offset `offset`.
	///
	/// # Panics
	///
	/// Implementors may panic if `offset` results in the section going out of bounds.
	fn section(&self, offset: TOffset) -> TOut;
}

/// Get a sized slice of an array.
///
/// # Panics
///
/// Panics if `N_OUT + offset > N`.
impl<T: Copy, const N: usize, const N_OUT: usize> Section<usize, [T; N_OUT]> for [T; N] {
	fn section(&self, offset: usize) -> [T; N_OUT] {
		// Neat! Adding this assertion actually speeds up the code as it removes the bounds checks in the loop.
		assert!(offset <= N - N_OUT, "Out of bounds");
		<[T; N_OUT]>::init(|i| self[i + offset])
	}
}
