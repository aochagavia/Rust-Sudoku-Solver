/*

Simple sudoku solver in Rust

For details on the implementation of the Sudoku look at sudoku/mod.rs

*/

#![feature(if_let)]

use std::io::{File, BufferedReader};
use std::os;
use sudoku::{Sudoku, BruteForce};

mod sudoku;

fn main() {
	println!("---------------------");
	println!("|   Sudoku-Solver   |");
	println!("---------------------\n");

	let args = os::args();
	if args.len() < 2 {
		println!("Usage: ssolver sudoku_to_solve [-b]");
		println!("Use -b if you want to use brute forcing exclusively (to compare performance)");
		return;
	}

	let path = Path::new(&args[1]);

    let file = File::open(&path).unwrap();
    let mut sudoku = Sudoku::new(BufferedReader::new(file));

    // Apply brute force directly if "-b" is specified as argument
    if args.len() > 2 && args[2].as_slice() == "-b" {
        println!("Brute forcing...");
        sudoku.brute_force();
    } else {
        sudoku.fast_solve();
    }

    // Fast solve doesn't always have success
    if !sudoku.is_completed() {
        println!("No solution found with fast method, attempting brute force...");
        if !sudoku.brute_force() {
            println!("Brute force failed, make sure that the sudoku is valid.");
            println!("Current solution (for debugging purposes): ");
        }
    }

    // Maybe it is now completed
    if sudoku.is_completed() {
        println!("A solution for \"{}\" has been found!", args[1]);
    }

    println!("\n{}", sudoku);
}