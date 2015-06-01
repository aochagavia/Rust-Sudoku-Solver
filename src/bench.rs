#![feature(test)]

extern crate test;

use sudoku::{Sudoku, BruteForce};
use self::test::Bencher;
use std::fs::File;
use std::io::BufReader;

mod sudoku;

#[bench]
fn bench_solve_wicked(b: &mut Bencher) {
    let file = File::open("../samples/wicked.txt").unwrap();
    let original = Sudoku::new(BufReader::new(file));

    // We measure the time needed to solve the sudoku
    b.iter(|| {
        let mut sudoku = original.clone();
        sudoku.fast_solve();
        if !sudoku.is_completed() {
            sudoku.brute_force();
        }
    });
}