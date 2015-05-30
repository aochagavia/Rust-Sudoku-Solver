/*

Simple sudoku solver in Rust

For details on the implementation of the Sudoku look at sudoku/mod.rs

*/

use std::fs::File;
use std::io::BufReader;
use std::env;

use sudoku::{Sudoku, BruteForce};

mod sudoku;

fn main() {
    println!("---------------------");
    println!("|   Sudoku-Solver   |");
    println!("---------------------\n");

    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: ssolver sudoku_to_solve [-b]");
        println!("Use -b if you want to use brute forcing exclusively (to compare performance)");
        return;
    }

    let file = File::open(&args[1]).unwrap();
    let mut sudoku = Sudoku::new(BufReader::new(file));

    // Apply brute force directly if "-b" is specified as argument
    if args.len() > 2 && &args[2] == "-b" {
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