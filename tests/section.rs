use higher_order_functions::Section;

#[test]
fn empty_array_section() {
	let a: [(); 0] = [];
	
	let arr: [_; 0] = a.section(0);
	
	assert_eq!(arr, []);
}

#[test]
fn empty_section() {
	let a = [1, 2, 3, 4, 5];
	
	let arr: [_; 0] = a.section(1);
	
	assert_eq!(arr, []);
}

#[test]
#[should_panic]
fn empty_out_of_bounds() {
	let a = [1, 2, 3, 4, 5];
	
	let arr: [_; 0] = a.section(7);
}

#[test]
fn start_section() {
	let a = [1, 2, 3, 4, 5];
	
	let arr: [_; 3] = a.section(0);
	
	assert_eq!(arr, [1, 2, 3]);
}

#[test]
fn end_section() {
	let a = [1, 2, 3, 4, 5];
	
	let arr: [_; 3] = a.section(2);
	
	assert_eq!(arr, [3, 4, 5]);
}

#[test]
fn full_section() {
	let a = [1, 2, 3, 4, 5];
	
	let arr: [_; 5] = a.section(0);
	
	assert_eq!(arr, [1, 2, 3, 4, 5]);
}

#[test]
#[should_panic]
fn out_of_bounds() {
	let a = [1, 2, 3, 4, 5];
	
	let arr: [_; 3] = a.section(3);
}
