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
		// shift i 0,1 or 2 in case of 3x3 matrix. Will fail if not unique!
		for c_index in 0..DIMENSIONPWR2 {
			let rb_index =  DIMENSION*r_index.div(DIMENSION)+(r_index+shift).rem(DIMENSION);
			let value_a = self.data[r_index ][c_index];
			let value_b = self.data[rb_index][c_index];
			self.data[r_index ][c_index] = value_b;
			self.data[rb_index][c_index] = value_a;
		}
	}

	pub fn swap_column(&mut self, c_index: usize, shift: usize) {
		// shift i 0,1 or 2 in case of 3x3 matrix. Will fail if not unique!
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

	pub fn verify(&self) -> bool {
		// Verify uniqueness of all rows.
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
		// Verify uniqueness of all column.
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
		// TODO check 3x3 square;
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
		// First clear others in row
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
		// Secondly, clear others in column
		for c in 0..DIMENSIONPWR2 {
			if c != c_index {
				let d = &mut self.row[r_index].column[c].options[value];
				if *d {
					count+=1;
					*d = false;
				}
			}
		}
		// As third action, clear others in rectangle
		let r1 = 3*r_index.div(3)+(r_index+1).rem(3);
		let r2 = 3*r_index.div(3)+(r_index+2).rem(3);
		let c1 = 3*c_index.div(3)+(c_index+1).rem(3);
		let c2 = 3*c_index.div(3)+(c_index+2).rem(3);
		{
			let d = &mut self.row[r1].column[c1].options[value];
			if *d {
				count+=1;
				*d = false;
			}
		}
		{
			let d = &mut self.row[r1].column[c2].options[value];
			if *d {
				count+=1;
				*d = false;
			}
		}
		{
			let d = &mut self.row[r2].column[c1].options[value];
			if *d {
				count+=1;
				*d = false;
			}
		}
		{
			let d = &mut self.row[r2].column[c2].options[value];
			if *d {
				count+=1;
				*d = false;
			}
		}
		count
	}

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
		// As third action, check others in rectangle
		let r1 = DIMENSION*r_index.div(DIMENSION)+(r_index+1).rem(DIMENSION);
		let r2 = DIMENSION*r_index.div(DIMENSION)+(r_index+2).rem(DIMENSION);
		let c1 = DIMENSION*c_index.div(DIMENSION)+(c_index+1).rem(DIMENSION);
		let c2 = DIMENSION*c_index.div(DIMENSION)+(c_index+2).rem(DIMENSION);
		let opt = self.unique_option(r1,c1);
		match opt {
			Some(val) => if val == value {unique = true},
			None      => {},
		}
		let opt = self.unique_option(r1,c2);
		match opt {
			Some(val) => if val == value {unique = true},
			None      => {},
		}
		let opt = self.unique_option(r2,c1);
		match opt {
			Some(val) => if val == value {unique = true},
			None      => {},
		}
		let opt = self.unique_option(r2,c2);
		match opt {
			Some(val) => if val == value {unique = true},
			None      => {},
		}
	unique
	}

	// Function which removes all unique options in a row/column/3x3mat.
	// If e.g. a 9 has only one possibly position in a row, set this value
	// and remove others in column and 3x3 mat.
	// Reports number of set cells. Must be called repetitvely till 0 cells
	// are touched.
	pub fn reduce_options(&mut self) -> usize {
		let mut count = 0;
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
					count = count + self.remove_others(r_index,c_i_last,symbol);
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
					count = count + self.remove_others(r_i_last,c_index,symbol);
				}
			}
			// then all 3x3 mat.
			for sr_index in 0..DIMENSION {
				for sc_index in 0..DIMENSION {
					let mut occ = 0;
					let mut r_i_last = 0;
					let mut c_i_last = 0;
					for r_index in 0..DIMENSION {
						for c_index in 0..DIMENSION {
							let tr_index = DIMENSION*sr_index+r_index;
							let tc_index = DIMENSION*sc_index+c_index;
							if self.row[tr_index].column[tc_index].options[symbol] {
								r_i_last = tr_index;
								c_i_last = tc_index;
								occ = occ+1;
							}
						}
					}
					if occ == 1 {
						count = count + self.remove_others(r_i_last,c_i_last,symbol);
					}
				}
			}
		}
		//println!("count = {:?}",count);
		count
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
