/*

Implements methods to detect lines of empty fields that are guaranteed to contain
a certain number. If such a line is found, the number will be projected.

Example:

---|123|45-
6--|---|---
---|---|---

will become

---|123|456
6--|---|---
---|---|---
	^^^
	In one of those there must be a 6
	
*/

use super::Sudoku;
use std::collections::HashSet;

pub trait ProjectLines {
    fn project_lines(&mut self) -> bool;
}

impl ProjectLines for Sudoku {
	// Checks each square to see if it contains any lines that can be projected
	fn project_lines(&mut self) -> bool {
		let mut progress = false;
	
		for x in range(0u, 3) {
			for y in range(0u, 3) {
				progress = check_square(self, x * 3, y * 3) || progress;
			}
		}
		
		progress
	}
}

// Check a single square to see if it contains any lines that can be projected
// If such lines are found, project them
fn check_square(sudoku: &mut Sudoku, corner_x: uint, corner_y: uint) -> bool {
    let mut progress = false;

    // Horizontal lines
    for y in range(0u, 3) {
        let diff = get_h_difference(sudoku, corner_x, y);
        for &num in diff.iter() {
            progress = project_h_line(sudoku, corner_x, y, num) || progress;
        }
    }

    // Vertical lines
    for x in range(0u, 3) {
        let diff = get_v_difference(sudoku, x, corner_y);
        for &num in diff.iter() {
            progress = project_v_line(sudoku, x, corner_y, num) || progress;
        }
    }
    
    progress
}

// Get the set of possible numbers in the given horizontal line, within the square
// and take the difference with the rest of the square
fn get_h_difference(sudoku: &mut Sudoku, corner_x: uint, y: uint) -> Vec<uint> {		
    // Set of possible numbers in given line
    let mut possible_numbers = HashSet::<uint>::new();
    for i in range(0u, 3) {
        possible_numbers.extend(sudoku.get(corner_x + i, y).possible_numbers.iter().map(|&n| n));
    }
    
    // Set of possible numbers in the rest of the square
    let (_, corner_y) = Sudoku::get_corner(corner_x, y);
    let mut other_numbers = HashSet::<uint>::new();
    for off_y in range(0u, 3) {
        // Discard numbers in the row Y
        if corner_y + off_y != y {
            for off_x in range(0u, 3) {
                other_numbers.extend(sudoku.get(corner_x + off_x, corner_y + off_y).possible_numbers.iter().map(|&n| n));
            }
        }
    }
    
    possible_numbers.difference(&other_numbers).map(|&x| x).collect::<Vec<uint>>()
}

fn get_v_difference(sudoku: &mut Sudoku, x: uint, corner_y: uint) -> Vec<uint> {
    // Set of possible numbers in given line
    let mut possible_numbers = HashSet::new();
    for i in range(0u, 3) {
        possible_numbers.extend(sudoku.get(x, corner_y + i).possible_numbers.iter().map(|&n| n));
    }
    
    // Set of possible numbers in the rest of the square
    let (corner_x, _) = Sudoku::get_corner(x, corner_y);
    let mut other_numbers = HashSet::new();
    for off_x in range(0u, 3) {
        // Discard numbers in the column X
        if corner_x + off_x != x {
            for off_y in range(0u, 3) {
                other_numbers.extend(sudoku.get(corner_x + off_x, corner_y + off_y).possible_numbers.iter().map(|&n| n));
            }
        }
    }
    
    // Difference
    possible_numbers.difference(&other_numbers).map(|&x| x).collect()
}

// Project a number horizontally to other squares
fn project_h_line(sudoku: &mut Sudoku, corner_x: uint, y: uint, projected_number: uint) -> bool {
    let mut progress = false;
    
    for x in range(0u, 9) {
        // Do not project to same squre
        if x < corner_x || corner_x + 3 <= x {
            progress = sudoku.get_mut(x, y).cannot_be(projected_number) || progress;
        }
    }
    
    progress
}

// Project a number vertically to other squares
fn project_v_line(sudoku: &mut Sudoku, x: uint, corner_y: uint, projected_number: uint) -> bool {
    let mut progress = false;
    
    for y in range(0u, 9) {
        // Do not project to same square
        if y < corner_y || corner_y + 3 <= y {
            progress = sudoku.get_mut(x, y).cannot_be(projected_number) || progress;
        }
    }
    
    progress
}