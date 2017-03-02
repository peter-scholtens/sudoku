#![feature(test)]
// Testbench to measure how fast a new Sudoku puzzle can be generated.
extern crate sudoku;
extern crate test;

use sudoku::*;

#[cfg(test)]
mod tests {
	use super::*;
	use test::Bencher;


	#[bench]
	fn bench_shake_randomly(b: &mut Bencher) {
		let mut field  = Sudoku::new(false);
		b.iter(|| field.shake_randomly() );
	}

	#[bench]
	fn bench_shake_till_reached_var(b: &mut Bencher) {
		let     p  = Sudoku::identity_sudoku();
		let mut n  = Sudoku::identity_sudoku();
		b.iter(|| {
			while count_equal_options(&p,&n) > 1 {
				n.shake_randomly();
			}
		} );
	}
}
