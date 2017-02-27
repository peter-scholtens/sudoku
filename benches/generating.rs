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
	fn bench_fill_randomly(b: &mut Bencher) {
		let mut field  = Sudoku::new();
		b.iter(|| field.fill_randomly() );
    }
}
