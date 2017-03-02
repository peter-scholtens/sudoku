// Testbench to measure how fast a new Sudoku puzzle can be generated.
extern crate sudoku;

#[test]
fn test_new_with_fault_data_rows() {
	let faulty = sudoku::Sudoku::new_with_data(vec![
	vec![1,1,1,1,1,1,1,1,1],
	vec![2,2,2,2,2,2,2,2,2],
	vec![3,3,3,3,3,3,3,3,3],
	vec![4,4,4,4,4,4,4,4,4],
	vec![5,5,5,5,5,5,5,5,5],
	vec![6,6,6,6,6,6,6,6,6],
	vec![7,7,7,7,7,7,7,7,7],
	vec![8,8,8,8,8,8,8,8,8],
	vec![9,9,9,9,9,9,9,9,9],
	]);
	assert_eq!(false, faulty.has_only_unique_options());
}

#[test]
fn test_new_with_fault_data_columns() {
	let faulty = sudoku::Sudoku::new_with_data(vec![
	vec![1,2,3,4,5,6,7,8,9],
	vec![1,2,3,4,5,6,7,8,9],
	vec![1,2,3,4,5,6,7,8,9],
	vec![1,2,3,4,5,6,7,8,9],
	vec![1,2,3,4,5,6,7,8,9],
	vec![1,2,3,4,5,6,7,8,9],
	vec![1,2,3,4,5,6,7,8,9],
	vec![1,2,3,4,5,6,7,8,9],
	vec![1,2,3,4,5,6,7,8,9],
	]);
	assert_eq!(false, faulty.has_only_unique_options());
}

#[test]
fn test_new_with_fault_data_groups() {
	let faulty = sudoku::Sudoku::new_with_data(vec![
	vec![1,2,4,3,5,6,7,8,9],
	vec![4,5,7,6,8,9,1,2,3],
	vec![7,8,1,9,2,3,4,5,6],
	vec![2,3,5,1,6,4,8,9,7],
	vec![5,6,8,4,9,7,2,3,1],
	vec![8,9,2,7,3,1,5,6,4],
	vec![3,1,6,2,4,5,9,7,8],
	vec![6,4,9,5,7,8,3,1,2],
	vec![9,7,3,8,1,2,6,4,5],
	]);
	assert_eq!(false, faulty.has_only_unique_options());
}

#[test]
fn test_new_with_correct_data() {
	let correct = sudoku::Sudoku::identity_sudoku();
	assert_eq!(true, correct.has_only_unique_options());
}
