extern crate sudoku;

use std::fs::File;
use std::io::BufReader;
use std::env;

use sudoku::Sudoku;

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
    let mut sudoku = Sudoku::from_reader(BufReader::new(file)).unwrap();

    // Apply brute force directly if "-b" is specified as argument
    if args.len() > 2 && &args[2] == "-b" {
        println!("Brute forcing...");
        sudoku.brute_force_solve();
    } else {
        sudoku.fast_solve();
    }

    // Fast solve doesn't always have success
    if !sudoku.is_solved() {
        println!("No solution found with fast method, attempting brute force...");
        sudoku.brute_force_solve();
        if !sudoku.is_solved() {
            println!("Brute force failed, make sure that the sudoku is valid.");
            println!("Current solution (for debugging purposes): ");
        }
    }

    // Maybe it is now completed
    if sudoku.is_solved() {
        println!("A solution for \"{}\" has been found!", args[1]);
    }

    println!("\n{}", sudoku);
}