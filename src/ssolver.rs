use std::io::{File, io_error};
use std::io::buffered::BufferedReader;
use std::os;
use sudoku::Sudoku;

mod sudoku;

fn main() {
	println("---------------------");
	println("|   Sudoku-Solver   |");
	println("---------------------\n");

	let args = os::args();
	if args.len() < 2 {
		println("Usage: ssolver sudoku_to_solve [-b]");
		println("Use -b if you want to use brute forcing exclusively (to compare performance)");
		return;
	}
	
	let path_str = args[1].clone();
	let path = Path::new(path_str.clone());
	
	io_error::cond.trap(|_| {
		fail!("The file could not be read. Ensure that the path is correct and that you have the permissions to read it.");
	}).inside(|| {
		let file = File::open(&path);
		let mut sudoku = Sudoku::new(BufferedReader::new(file.unwrap()));
		
		if args.len() > 2 && args[2] == ~"-b" {
			println("Brute forcing...");
			sudoku.brute_force();
		} else {
			sudoku.fast_solve();
		}
		
		// Fast solve doesn't always have success
		if !sudoku.is_completed() {
			println("No solution found with fast method, attempting brute force...");
			if !sudoku.brute_force() {
				println("Brute force failed, make sure that the sudoku is valid.");
			}
		}
		
		// Maybe it is now completed
		if sudoku.is_completed() {
			println!("A solution for \"{}\" has been found!", path_str);
			println!("\n{}", sudoku.to_str());
		}
	});
}