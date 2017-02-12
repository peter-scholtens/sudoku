//Sudoku solver
//- first generate a data structure of enum type:  Empty,1,2,3,4,5,6,7,8,9
//- then populate the sudoku fully.


//CREATE a DIFFICULT SUDOKU:
//- generate a datastructure with options, an array of option of numbers:
 //[true,false,true,false,true,false,true,false,false] means in this field a 1,3 5 or 7 are allowed.
//1- start with all rows and columns filled with full options for all fields.
//2- randomly, pick a field, and select one of the remaining options.
//3- reduce the option of the other fields. repeat this till no redcution is done anymore.
//4  if all fields still have more than one option, goto 2.
//5 exit: this is the most difficult sudoku, with solution.st enough data to solve it.


//Solve rules:
//1) - check in row, if only one field is empty. This one can be solved now.
//2) same for column,
//3) same for 3x3 square.

//solver algortihm is now:
//apply rule 1, then 2 then 3
//if none of then solved a field, exit with panic "cannot solve" else next



//Solving, see also the dancing links method:
//https://en.wikipedia.org/wiki/Sudoku

extern crate sudoku;

use sudoku::*;

fn main() {
	let mut field  = SudokuField::new();
	field.fill_randomly();
	//println!("field:");
	//field.print();
	let problem = create_puzzle(&field);
	println!("New sudoku problem:\n");
	problem.print();
	println!("");
	problem.stats();
}
