extern crate rand;

use std::ops::Rem;
use std::ops::Div;
use rand::Rng;

const  DIMENSION: usize = 3; // For fun we may choose 4 for hexadecimal sudoku's
const  DIMENSIONPWR2: usize = DIMENSION*DIMENSION;

pub struct SudokuCell {
	options: Vec<bool>,
}

struct SudokuRow {
	column: Vec<SudokuCell>,
}

pub struct SudokuSolution {
	row: Vec<SudokuRow>,
}

pub struct SudokuField {
	data: Vec<Vec<usize>>,
}

pub struct SudokuProblem {
	data: Vec<Vec<Option<usize>>>,
}

impl SudokuProblem {

	pub fn new() -> SudokuProblem {
		SudokuProblem{ data: vec![vec![None;DIMENSIONPWR2];DIMENSIONPWR2]}
	}

	pub fn set(&mut self, r:usize, c:usize, v:usize) {
		self.data[r][c]=Some(v);
	}

	pub fn print(&self) {
		for ri in 0..DIMENSIONPWR2 {
			for ci in 0..DIMENSIONPWR2 {
				match self.data[ri][ci] {
					Some(value) => {
						print!("{:?}",value+1);
					},
					None => {
						print!(" ");
					}
				}
			}
			println!("");
		}
	}

	pub fn stats(&self) {
		let mut count = 0;
		for ri in 0..DIMENSIONPWR2 {
			for ci in 0..DIMENSIONPWR2 {
				if self.data[ri][ci].is_some() {
					count = count+1;
				}
			}
		}
		println!("Revealed {:?} cells of {:?}",count,DIMENSIONPWR2*DIMENSIONPWR2);
	}
}

impl SudokuField {

	pub fn new() -> SudokuField {
		SudokuField{ data: vec![vec![0;DIMENSIONPWR2];DIMENSIONPWR2]}
	}

	pub fn new_with_data(input: Vec<Vec<usize>>) -> SudokuField {
		let mut retval = SudokuField{data: input};
		// Caution, all data was numbers 1..9 but should be changed to index 0..8
		for row in retval.data.iter_mut() {
			for column in row.iter_mut() {
				*column = *column-1;
			}
		}
		retval
	}

	pub fn set(&mut self, r:usize, c:usize, v:usize) {
		self.data[r][c]=v;
	}

	pub fn print(&self) {
		for ri in 0..DIMENSIONPWR2 {
			for ci in 0..DIMENSIONPWR2 {
				print!("{:?}",self.data[ri][ci]+1);
			}
			println!("");
		}
	}

	pub fn swap_row(&mut self, r_index: usize, shift: usize) {
		// shift i 0,1 or 2 in case of 3x3 group. Will fail if not unique!
		for c_index in 0..DIMENSIONPWR2 {
			let rb_index =  DIMENSION*r_index.div(DIMENSION)+(r_index+shift).rem(DIMENSION);
			let value_a = self.data[r_index ][c_index];
			let value_b = self.data[rb_index][c_index];
			self.data[r_index ][c_index] = value_b;
			self.data[rb_index][c_index] = value_a;
		}
	}

	pub fn swap_column(&mut self, c_index: usize, shift: usize) {
		// shift i 0,1 or 2 in case of 3x3 group. Will fail if not unique!
		for r_index in 0..DIMENSIONPWR2 {
			let cb_index =  DIMENSION*c_index.div(DIMENSION)+(c_index+shift).rem(DIMENSION);
			let value_a = self.data[r_index][c_index ];
			let value_b = self.data[r_index][cb_index];
			self.data[r_index][c_index ]=value_b;
			self.data[r_index][cb_index]=value_a;
		}
	}

	pub fn fill_randomly(&mut self) {
		// Some description:
		//First generate an identity sudoku:
		//
		// 123 456 789 
		// 456 789 123 
		// 789 123 456
		// 231
		//     312
		//         312
		// 231
		//     231
		//         231
		//
		// Secondly, swap full rows or columns in within 3-range.
		for r_index in 0..DIMENSIONPWR2 {
			for c_index in 0..DIMENSIONPWR2 {
				let val: usize = (c_index+DIMENSION*r_index).rem(DIMENSIONPWR2);
				let shift: usize = r_index.div(DIMENSION);
				let newval = DIMENSION*val.div(DIMENSION)+(val+shift).rem(DIMENSION);
				self.data[r_index][c_index] = newval;
			}
		}
		// When enough shaken?? Assume 199 times...
		for _ in 0..199 {
			let row_index    = rand::thread_rng().gen_range(0, DIMENSIONPWR2);
			let row_shift    = rand::thread_rng().gen_range(1, DIMENSION);
			let column_index = rand::thread_rng().gen_range(0, DIMENSIONPWR2);
			let column_shift = rand::thread_rng().gen_range(1, DIMENSION);
			self.swap_row(   row_index   ,row_shift    );
			self.swap_column(column_index,column_shift);
		}
	}

	// Verifies if every number is noly seen once in each row, column and group.
	pub fn verify(&self) -> bool {
		// Verify uniqueness of in all columns.
		for ci in 0..DIMENSIONPWR2 {
			let mut seen = vec![false;DIMENSIONPWR2];
			for ri in 0..DIMENSIONPWR2 {
				let v = self.data[ri][ci];
				if seen[v] {
					return false;
				} else {
					seen[v] = true;
				}
			}
		}
		// Verify uniqueness of all rows.
		for ri in 0..DIMENSIONPWR2 {
			let mut seen = vec![false;DIMENSIONPWR2];
			for ci in 0..DIMENSIONPWR2 {
				let v = self.data[ri][ci];
				if seen[v] {
					return false;
				} else {
					seen[v] = true;
				}
			}
		}
		// Verify uniquness of all groups;
		for sr_index in 0..DIMENSION {
			for sc_index in 0..DIMENSION {
				let mut seen = vec![false;DIMENSIONPWR2];
				for r_index in 0..DIMENSION {
					for c_index in 0..DIMENSION {
						let tr_index = DIMENSION*sr_index+r_index;
						let tc_index = DIMENSION*sc_index+c_index;
						let v = self.data[tr_index][tc_index];
						if seen[v] {
							return false;
						} else {
							seen[v] = true;
						}
					}
				}
			}
		}
		// If we arrive here, the sudoku field is correct.
		return true
	}
}

impl SudokuSolution {
	pub fn new() -> SudokuSolution {
		let mut s = SudokuSolution{row: Vec::new()};
		for _ in 0..DIMENSIONPWR2 {
			let mut r = SudokuRow{column: Vec::new()};
			for _ in 0..DIMENSIONPWR2 {
				let e = SudokuCell{options: vec![true;DIMENSIONPWR2]};
				r.column.push(e);
			}
		s.row.push(r);
		}
		s
	}


	pub fn print(&self) {
		for r in self.row.iter() {
			for c in r.column.iter() {
				for (index, value) in c.options.iter().enumerate() {
					if *value
						{
							print!("{}",index+1);
						}
					else
						{
							print!("_");
						}
				}
				print!(" ");
			}
			println!("");
		}
	}


	pub fn print_compact(&self) {
		for (ri,rv) in self.row.iter().enumerate() {
			for (ci,_) in rv.column.iter().enumerate() {
				let opt = self.unique_option(ri,ci);
				match opt {
					Some(val) => print!("{}",val+1),
					None      => print!("*"),
				}
			}
			println!("");
		}
	}

	pub fn set(&mut self,r_index: usize, c_index: usize, value: usize) -> bool {
		// first check if is allowed, anyhow do it.
		let test = self.row[r_index].column[c_index].options[value];
		for i in self.row[r_index].column[c_index].options.iter_mut()
		{
			*i = false;
		}
		self.row[r_index].column[c_index].options[value] = true;
		self.remove_others(r_index,c_index,value);
		return test;
	}

	// Verifies if at position (r,c) a unique option is present.
	// Can be faster with take_while() ??
	pub fn unique_option(&self,r_index: usize, c_index: usize) -> Option<usize> {
		let mut uniq = 0;
		let a = self.row[r_index].column[c_index].options.iter().enumerate().fold(
			0,|mut acc,(index,value)|{
				if *value {
					uniq = index;
					acc = acc+1;
					};
				acc
				}
			);
		if a == 1 {
			return Some(uniq);
		}
		else {
			return None
		}
	}

	pub fn empty_option(&self,r_index: usize, c_index: usize) -> bool {
		self.row[r_index].column[c_index].options.iter().fold(
			true,|empty:bool ,opt|{
				empty & (*opt == false)
			}
		);
		true
	}

	pub fn undecided_cells(&self)-> Vec<(usize,usize,usize)> {
		let mut undec_cells = Vec::new();
		for (ri,rv) in self.row.iter().enumerate() {
			for (ci,_) in rv.column.iter().enumerate() {
				let mut count = 0;
				for i in 0..DIMENSIONPWR2 {
					if self.row[ri].column[ci].options[i] == true {
						count = count + 1;
					}
				}
				if count > 1 {
					undec_cells.push((ri,ci,count));
				}
			}
		}
		undec_cells.sort_by_key(|tuple| DIMENSIONPWR2-tuple.2);
		//println!("{:?}",undec_cells);
		//println!("");
		undec_cells
	}

	pub fn unique_sudoku(&self) -> bool {
		let mut unique = true;
		for (ri,rv) in self.row.iter().enumerate() {
			for (ci,_) in rv.column.iter().enumerate() {
				let t = self.unique_option(ri,ci).is_some();
				unique = unique & t;
				//if t {print!(" ")} else {print!("X")};
			}
		}
		//println!("");
		unique
	}

	pub fn remove_others(&mut self,r_index: usize, c_index: usize, value: usize) -> usize {
		let mut count = 0;
		// First clear option of others in row
		for r in 0..DIMENSIONPWR2 {
			if r != r_index {
				{
					let d = &mut self.row[r].column[c_index].options[value];
					if *d {
						count+=1;
						*d = false;
					}
				}
			}
		}
		// Secondly, clear option of others in column
		for c in 0..DIMENSIONPWR2 {
			if c != c_index {
				let d = &mut self.row[r_index].column[c].options[value];
				if *d {
					count+=1;
					*d = false;
				}
			}
		}
		// As third action, clear option others in group
		for sub_r_index in 1..DIMENSION {
			for sub_c_index in 1..DIMENSION {
				let r = DIMENSION*r_index.div(DIMENSION)+(r_index+sub_r_index).rem(DIMENSION);
				let c = DIMENSION*c_index.div(DIMENSION)+(c_index+sub_c_index).rem(DIMENSION);
				let d = &mut self.row[r].column[c].options[value];
				if *d {
					count+=1;
					*d = false;
				}
			}
		}
		count
	}

	// Verifies if one of the others cells in the same row, column or group has a unique option.
	pub fn unique_others(&self,r_index: usize, c_index: usize, value: usize) -> bool {
		let mut unique = false;
		// First check others in row
		for r in 0..DIMENSIONPWR2 {
			if r != r_index {
				let opt = self.unique_option(r,c_index);
				match opt {
					Some(val) => if val == value {unique = true},
					None      => {},
				}
			}
		}
		// Secondly, check others in column
		for c in 0..DIMENSIONPWR2 {
			if c != c_index {
				let opt = self.unique_option(r_index,c);
				match opt {
					Some(val) => if val == value {unique = true},
					None      => {},
				}
			}
		}
		// As third action, check others in group
		for sub_r_index in 1..DIMENSION {
			for sub_c_index in 1..DIMENSION {
				let r = DIMENSION*r_index.div(DIMENSION)+(r_index+sub_r_index).rem(DIMENSION);
				let c = DIMENSION*c_index.div(DIMENSION)+(c_index+sub_c_index).rem(DIMENSION);
				let opt = self.unique_option(r,c);
				match opt {
					Some(val) => if val == value {unique = true},
					None      => {},
				}
			}
		}
	unique
	}

	// Function which removes all unique options in a row/column/3x3group.
	// If e.g. a 9 has only one possibly position in a row, set this value
	// and remove others in column and 3x3 group.
	// Reports number of set cells. Must be called repetitvely till 0 cells
	// are touched.
	pub fn reduce_options(&mut self) -> usize {
		let mut count_reduc = 0;
		for symbol in 0..DIMENSIONPWR2 {
			// first check rows.
			for r_index in 0..DIMENSIONPWR2 {
				let mut occ = 0;
				let mut c_i_last = 0;
				for c_index in 0..DIMENSIONPWR2 {
					if self.row[r_index].column[c_index].options[symbol] {
						c_i_last = c_index;
						occ = occ+1;
					}
				}
				if occ == 1 {
					count_reduc = count_reduc + self.remove_others(r_index,c_i_last,symbol);
				}
			}
			// then all columns.
			for c_index in 0..DIMENSIONPWR2 {
				let mut occ = 0;
				let mut r_i_last = 0;
				for r_index in 0..DIMENSIONPWR2 {
					if self.row[r_index].column[c_index].options[symbol] {
						r_i_last = r_index;
						occ = occ+1;
					}
				}
				if occ == 1 {
					count_reduc = count_reduc + self.remove_others(r_i_last,c_index,symbol);
				}
			}
			// then all 3x3 group.
			for super_r_index in 0..DIMENSION {
				for super_c_index in 0..DIMENSION {
					let mut occ = 0;
					let mut r_i_last = 0;
					let mut c_i_last = 0;
					for r_index in 0..DIMENSION {
						for c_index in 0..DIMENSION {
							let tr_index = DIMENSION*super_r_index+r_index;
							let tc_index = DIMENSION*super_c_index+c_index;
							if self.row[tr_index].column[tc_index].options[symbol] {
								r_i_last = tr_index;
								c_i_last = tc_index;
								occ = occ+1;
							}
						}
					}
					if occ == 1 {
						count_reduc = count_reduc + self.remove_others(r_i_last,c_i_last,symbol);
					}
				}
			}
			// Symbol is unique in two rows and two columns of the same group?
			// Then groups has only one option left.
			for super_r_index in 0..DIMENSION {
				for super_c_index in 0..DIMENSION {
					// Verify cells with shared column, outside group.
					let mut unique_column = vec![false; DIMENSION];
					for sub_c_index in 0..DIMENSION {
						let c_index = super_c_index*DIMENSION+sub_c_index;
						// First cells before in the group.
						for r_index in 0..DIMENSION*super_r_index {
							let opt = self.unique_option(r_index,c_index);
							match opt {
								Some(val) => if val == symbol {unique_column[sub_c_index] = true},
								None      => {},
							}
						}
						// Second cells after in the group. Same code... DRY?
						for r_index in DIMENSION*(super_r_index+1)..DIMENSION*DIMENSION {
							let opt = self.unique_option(r_index,c_index);
							match opt {
								Some(val) => if val == symbol {unique_column[sub_c_index] = true},
								None      => {},
							}
						}
					}
					// Now same for rows...
					let mut unique_row = vec![false; DIMENSION];
					for sub_r_index in 0..DIMENSION {
						let r_index = super_r_index*DIMENSION+sub_r_index;
						// First cells before in the group.
						for c_index in 0..DIMENSION*super_c_index {
							let opt = self.unique_option(r_index,c_index);
							match opt {
								Some(val) => if val == symbol {unique_row[sub_r_index] = true},
								None      => {},
							}
						}
						// Second cells after in the group. Same code... DRY?
						for c_index in DIMENSION*(super_c_index+1)..DIMENSION*DIMENSION {
							let opt = self.unique_option(r_index,c_index);
							match opt {
								Some(val) => if val == symbol {unique_column[sub_r_index] = true},
								None      => {},
							}
						}
					}
					// If both unique vectors have DIM-1 uniques, than we can set the remaining one.
					let mut count_r = DIMENSION;
					let mut last_r  = 0;
					for r in 0..DIMENSION {
						if unique_row[r] {
							count_r = count_r - 1;
						} else {
							last_r = r; // get remaining non-unique row.
						}
					}
					let mut count_c = DIMENSION;
					let mut last_c  = 0;
					for c in 0..DIMENSION {
						if unique_column[c] {
							count_c = count_c - 1;
						} else {
							last_c = c; // get remaining non-unique column.
						}
					}
					// okay, so if only one non-unique row and one non-unique column remain,
					// then we can set this one:
					let r = super_r_index*DIMENSION+last_r;
					let c = super_c_index*DIMENSION+last_c;
					// Make sure that it not was unique already (to avoid inf. loop)
					if count_r == 1 && count_c == 1 && self.unique_option(r,c).is_none() {
						self.set(r,c,symbol);
						count_reduc = count_reduc + 1;
					}
				}
			}
		}
		//println!("count_reduc = {:?}",count_reduc);
		count_reduc
	}

}

pub fn create_puzzle(field: &SudokuField) -> SudokuProblem {
	let mut space = SudokuSolution::new();
	let mut problem = SudokuProblem::new();
	// As long as the solution space is not unique, reveal another cell of the problem matrix.
	while !space.unique_sudoku() {
		let undec_cells = space.undecided_cells();
		// Choose one of the first quarter of sorted cells (the ones with the most options).
		let i = rand::thread_rng().gen_range(0, 1+undec_cells.len().div(4));
		let (ri,ci,_) = undec_cells[i];
		let value = field.data[ri][ci];
		problem.set(ri,ci,value);
		space.set(ri,ci,value);
		loop {
			if space.reduce_options() == 0 {
				break;
			}
		}
	}
	problem
}
