extern crate test;
extern crate collections;

use sudoku::Sudoku;
use self::test::Bencher;
use std::io::{File, BufferedReader};

mod sudoku;

#[bench]
fn bench_solve_wicked(b: &mut Bencher) {
    let file = File::open(&Path::new("../samples/wicked.txt")).unwrap();
    let original = Sudoku::new(BufferedReader::new(file));

    // We measure the time needed to solve the sudoku
    b.iter(|| {
        let mut sudoku = original.clone();
        sudoku.fast_solve();
        if !sudoku.is_completed() {
            sudoku.brute_force();
        }
    });
}