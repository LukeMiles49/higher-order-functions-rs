use higher_order_functions::Zip;

#[test]
fn zip_empty_arrays() {
	let arr = [].zip([], |_: (), _: ()| panic!("Shouldn't call map function"));
	assert_eq!(arr, []);
}

#[test]
fn zip_singleton_arrays() {
	let mut called = false;
	let arr = [123].zip([456], |a, b| {
		assert_eq!(a, 123);
		assert_eq!(b, 456);
		if called { panic!("Should only call zip function once"); }
		else { called = true; }
		321
	});
	assert!(called);
	assert_eq!(arr, [321]);
}

#[test]
fn zip_tuples() {
	let a = [1, 2, 3];
	let b = ["a", "b", "c"];
	
	let arr = a.zip(b, |ax, bx| (ax, bx));
	
	assert_eq!(arr, [(1, "a"), (2, "b"), (3, "c")]);
}

#[test]
fn zip_multiply() {
	let a = [1, 2, 3];
	let b = [4, 5, 6];
	
	let arr = a.zip(b, |ax, bx| ax * bx);
	
	assert_eq!(arr, [4, 10, 18]);
}
