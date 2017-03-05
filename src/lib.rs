extern crate rand;

use std::ops::Rem;
use std::ops::Div;
use rand::Rng;

pub const  DIMENSION: usize = 3; // For fun we may choose 4 for hexadecimal sudoku's
pub const  DIMENSIONPWR2: usize = DIMENSION*DIMENSION;

pub struct Sudoku {
	data: Vec<Vec<Vec<bool>>>,
}

impl Sudoku {

	pub fn new(init_b: bool) -> Sudoku {
		Sudoku{data: vec![vec![vec![init_b;DIMENSIONPWR2];DIMENSIONPWR2];DIMENSIONPWR2] }
	}

	pub fn set(&mut self, r:usize, c:usize, v:usize) {
		for vi in 0..DIMENSIONPWR2 {
			self.data[r][c][vi]=false;
		}
		self.data[r][c][v]=true;
	}

	pub fn print(&self) {
		for ri in 0..DIMENSIONPWR2 {
			// Print lines between rows, different one for first row.
			if ri == 0 {
				println!("┌───┬───┬───┬───┬───┬───┬───┬───┬───┐");
			} else
			{
				println!("├───┼───┼───┼───┼───┼───┼───┼───┼───┤");
			}
			print!("│");
			for ci in 0..DIMENSIONPWR2 {
				let mut count_v = 0;
				let mut value   = 0;
				for vi in 0..DIMENSIONPWR2 {
					if self.data[ri][ci][vi] {
						count_v = count_v+1;
						value = vi;
					}
				}
				if count_v == 1 {
					print!(" {:?} │",value+1);
				} else {
					print!("   │");
				}
			}
			println!("");
		}
		println!("└───┴───┴───┴───┴───┴───┴───┴───┴───┘");
	}

	pub fn stats(&self) {
		let mut count = 0;
		for ri in 0..DIMENSIONPWR2 {
			for ci in 0..DIMENSIONPWR2 {
				let mut count_v = 0;
				for vi in 0..DIMENSIONPWR2 {
					if self.data[ri][ci][vi] {
						count_v = count_v+1;
					}
				}
				// if unique then count as revealed.
				if count_v == 1 {
					count = count+1;
				}
			}
		}
		println!("Revealed {:?} cells of {:?}",count,DIMENSIONPWR2*DIMENSIONPWR2);
	}

	pub fn new_with_data(input: Vec<Vec<usize>>) -> Sudoku {
		let mut retval = Sudoku::new(false);
		// Caution, all data was numbers 1..9 but should be changed to index 0..8
		for (ri,row) in retval.data.iter_mut().enumerate() {
			for (ci,column) in row.iter_mut().enumerate() {
				column[input[ri][ci]-1] = true;
			}
		}
		retval
	}

	pub fn swap_row(&mut self, r_index: usize, shift: usize) {
		// shift i 0,1 or 2 in case of 3x3 group. Will fail if not unique!
		let rb_index =  DIMENSION*r_index.div(DIMENSION)+(r_index+shift).rem(DIMENSION);
		self.data.swap(r_index,rb_index);
	}

	pub fn swap_column(&mut self, c_index: usize, shift: usize) {
		// shift i 0,1 or 2 in case of 3x3 group. Will fail if not unique!
		let cb_index =  DIMENSION*c_index.div(DIMENSION)+(c_index+shift).rem(DIMENSION);
		for r_index in 0..DIMENSIONPWR2 {
			self.data[r_index].swap(c_index,cb_index);
		}
	}

	pub fn identity_sudoku() -> Sudoku {
		// Creates an identity sudoku:
		//
		// 123 456 789 
		// 456 789 123  (shift left in groups of three)
		// 789 123 456  (idem)
		// 231          (shift left inside group of three)
		//     312      (shift left in groupd of three)
		//         312
		// 231
		//     231
		//         231
		let mut s = Sudoku::new(false);
		for r_index in 0..DIMENSIONPWR2 {
			for c_index in 0..DIMENSIONPWR2 {
				let val: usize = (c_index+DIMENSION*r_index).rem(DIMENSIONPWR2);
				let shift: usize = r_index.div(DIMENSION);
				let newval = DIMENSION*val.div(DIMENSION)+(val+shift).rem(DIMENSION);
				s.set(r_index,c_index,newval);
			}
		}
	s
	}

	pub fn shake_randomly(&mut self) {
		// Swap full rows or columns in within 3-range.
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

	// Verifies if every number is only seen at most once in each row, column and group.
	pub fn has_only_unique_options(&self) -> bool {
		// Verify uniqueness of in all columns.
		for ci in 0..DIMENSIONPWR2 {
			let mut seen = vec![false;DIMENSIONPWR2];
			for ri in 0..DIMENSIONPWR2 {
				for (i,v) in self.data[ri][ci].iter().enumerate() {
					if *v {
						if seen[i] {
							return false;
						} else {
							seen[i] = true;
						}
					}
				}
			}
		}
		// Verify uniqueness of all rows.
		for ri in 0..DIMENSIONPWR2 {
			let mut seen = vec![false;DIMENSIONPWR2];
			for ci in 0..DIMENSIONPWR2 {
				for (i,v) in self.data[ri][ci].iter().enumerate() {
					if *v {
						if seen[i] {
							return false;
						} else {
						seen[i] = true;
						}
					}
				}
			}
		}
		// Verify uniqueness of all groups;
		for sr_index in 0..DIMENSION {
			for sc_index in 0..DIMENSION {
				let mut seen = vec![false;DIMENSIONPWR2];
				for r_index in 0..DIMENSION {
					for c_index in 0..DIMENSION {
						let tr_index = DIMENSION*sr_index+r_index;
						let tc_index = DIMENSION*sc_index+c_index;
						for (i,v) in self.data[tr_index][tc_index].iter().enumerate() {
							if *v {
								if seen[i] {
									return false;
								} else {
									seen[i] = true;
								}
							}
						}
					}
				}
			}
		}
		// If we arrive here, the sudoku field is correct.
		return true
	}

	pub fn print_options(&self) {
		for r in self.data.iter() {
			for c in r.iter() {
				for (index, value) in c.iter().enumerate() {
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

	// Verifies if at position (r,c) a unique option is present.
	// Can be faster with take_while() ??
	pub fn unique_option(&self,r_index: usize, c_index: usize) -> Option<usize> {
		let mut uniq = 0;
		let a = self.data[r_index][c_index].iter().enumerate().fold(
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
		self.data[r_index][c_index].iter().fold(
			true,|empty:bool ,opt|{
				empty & (*opt == false)
			}
		);
		true
	}

	pub fn undecided_cells(&self)-> Vec<(usize,usize,usize)> {
		let mut undec_cells = Vec::new();
		for (ri,rv) in self.data.iter().enumerate() {
			for (ci,_) in rv.iter().enumerate() {
				let mut count = 0;
				for oi in 0..DIMENSIONPWR2 {
					if self.data[ri][ci][oi] == true {
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
		for (ri,rv) in self.data.iter().enumerate() {
			for (ci,_) in rv.iter().enumerate() {
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
					let d = &mut self.data[r][c_index][value];
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
				let d = &mut self.data[r_index][c][value];
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
				let d = &mut self.data[r][c][value];
				if *d {
					count+=1;
					*d = false;
				}
			}
		}
		count
	}

	// Function which removes all options of others if a unique options in a
	// row/column/3x3group is detected.
	// If e.g. a 9 has only one possibly position in a cell, set this value
	// and remove others in row, column and 3x3 group.
	// Reports number of set cells. Must be called repetitively till 0 cells
	// are touched: removal of one option may lead to another cell being unique.
	pub fn reduce_options(&mut self) -> usize {
		let mut candidates_set = Vec::new();
		let mut count_u = 0;
		// First check all unique cells: queue action to remove others in vector.
		for r_index in 0..DIMENSIONPWR2 {
			for c_index in 0..DIMENSIONPWR2 {
				match self.unique_option(r_index,c_index) {
					Some(value) => {
						candidates_set.push( (r_index,c_index,value) );
						count_u = count_u + 1;
						},
					None        => {},
				}
			}
		}

		// Verify if a symbol/value has only one option in row.
		let mut count_r = 0;
		for value in 0..DIMENSIONPWR2 {
			for r_index in 0..DIMENSIONPWR2 {
				let mut num_options = 0;
				let mut c_last = 0;
				for c_index in 0..DIMENSIONPWR2 {
					if self.data[r_index][c_index][value] == true {
						num_options = num_options + 1;
						c_last = c_index;
					}
				}
				if num_options == 1 {
					candidates_set.push( (r_index,c_last,value) );
					count_r = count_r + 1;
				}
			}
		}

		// Verify if a symbol/value has only one option in column.
		let mut count_c = 0;
		for value in 0..DIMENSIONPWR2 {
			for c_index in 0..DIMENSIONPWR2 {
				let mut num_options = 0;
				let mut r_last = 0;
				for r_index in 0..DIMENSIONPWR2 {
					if self.data[r_index][c_index][value] == true {
						num_options = num_options + 1;
						r_last = r_index;
					}
				}
				if num_options == 1 {
					candidates_set.push( (r_last,c_index,value) );
					count_c = count_c + 1;
				}
			}
		}

		// Verify if a symbol/value has only one option in group.
		let mut count_g = 0;
		for value in 0..DIMENSION {
			for super_r_index in 0..DIMENSION {
				for super_c_index in 0..DIMENSION {
					let mut num_options = 0;
					let mut r_last = 0;
					let mut c_last = 0;
					for sub_r_index in 0..DIMENSION {
						for sub_c_index in 0..DIMENSION {
							let r_index = super_r_index*DIMENSION+sub_r_index;
							let c_index = super_c_index*DIMENSION+sub_c_index;
							if self.data[r_index][c_index][value] == true {
								num_options = num_options + 1;
								r_last = r_index;
								c_last = c_index;
							}
						}
					}
					if num_options == 1 {
						candidates_set.push( (r_last,c_last,value) );
					count_g = count_g + 1;
					}
				}
			}
		}

		//Next remove others of these candidates.
		let mut count_o = 0;
		for &(r,c,v) in candidates_set.iter() {
			//println!("{:?}-{:?}-{:?}",r,c,v);
			self.set(r,c,v);
			count_o = count_o + self.remove_others(r,c,v);
		}

		let mut count_e = 0;
		for symbol in 0..DIMENSIONPWR2 {
			// Symbol is unique in two rows and two columns, but outside the same group?
			// Then this group has only one option left.
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
					let mut count_row = DIMENSION;
					let mut last_r  = 0;
					for r in 0..DIMENSION {
						if unique_row[r] {
							count_row = count_row - 1;
						} else {
							last_r = r; // get remaining non-unique row.
						}
					}
					let mut count_column = DIMENSION;
					let mut last_c  = 0;
					for c in 0..DIMENSION {
						if unique_column[c] {
							count_column = count_column - 1;
						} else {
							last_c = c; // get remaining non-unique column.
						}
					}
					// okay, so if only one non-unique row and one non-unique column remain,
					// then we can set this one:
					let r = super_r_index*DIMENSION+last_r;
					let c = super_c_index*DIMENSION+last_c;
					// Make sure that it not was unique already (to avoid inf. loop)
					if count_row == 1 && count_column == 1 && self.unique_option(r,c).is_none() {
						self.set(r,c,symbol);
						count_e = count_e + self.remove_others(r,c,symbol);
					}
				}
			}
		}
		let sum = count_e+count_o;// to avoid loop
		if sum > 0 {
			//println!("Removed (r={:?}, c={:?}, g={:?}), e={:?}, o={:?}",count_r,count_c,count_g,count_e,count_o);
		}
		sum
	}

}

pub fn create_puzzle(field: &Sudoku) -> Sudoku {
	let mut space = Sudoku::new(true);
	let mut problem = Sudoku::new(false);
	//println!("space:");
	//space.print_options();
	//println!("problem:");
	//problem.print_options();
	// As long as the solution space is not unique, reveal another cell of the problem matrix.
	while !space.unique_sudoku() {
		//println!("--------");
		let undec_cells = space.undecided_cells();
		// Choose one of the first quarter of sorted cells (the ones with the most options).
		let i = rand::thread_rng().gen_range(0, 1+undec_cells.len().div(4));
		let (ri,ci,_) = undec_cells[i];
		let mut value = 0;
		while field.data[ri][ci][value] == false {
			value = value + 1;
		}
		problem.set(ri,ci,value);
		space.set(ri,ci,value);
		loop {
			if space.reduce_options() == 0 {
				break;
			}
		}
		//println!("space:");
		//space.print_options();
		//println!("problem:");
		//problem.print_options();
		//println!("equal options: {}",count_equal_options(&space,&problem));
	}
	problem
}


pub fn count_equal_options(a: &Sudoku,b: &Sudoku) -> usize {
	let mut count = 0;
	for ri in 0..DIMENSIONPWR2 {
		for ci in 0..DIMENSIONPWR2 {
			for oi in 0..DIMENSIONPWR2 {
				if a.data[ri][ci][oi] && b.data[ri][ci][oi] {
					count = count+1;
				}
			}
		}
	}
	count
}
