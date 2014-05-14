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
use collections::HashSet;

impl ::sudoku::Sudoku {
	// Checks each square to see if it contains any lines that can be projected
	pub fn project_lines(&mut self) -> bool {
		let mut progress = false;
	
		for x in range(0u, 3) {
			for y in range(0u, 3) {
				progress = self.check_square(x * 3, y * 3) || progress;
			}
		}
		
		progress
	}
	
	// Check a single square to see if it contains any lines that can be projected
	// If such lines are found, project them
	fn check_square(&mut self, cornerX: uint, cornerY: uint) -> bool {
		let mut progress = false;
	
		// Horizontal lines
		for y in range(0u, 3) {
            let diff = self.get_h_difference(cornerX, y);
			for &num in diff.iter() {
				progress = self.project_h_line(cornerX, y, num) || progress;
			}
		}

		// Vertical lines
		for x in range(0u, 3) {
			let diff = self.get_v_difference(x, cornerY);
			for &num in diff.iter() {
				progress = self.project_v_line(x, cornerY, num) || progress;
			}
		}
		
		progress
	}
	
	// Get the set of possible numbers in the given horizontal line, within the square
	// and take the difference with the rest of the square
	fn get_h_difference(&mut self, cornerX: uint, y: uint) -> Vec<uint> {		
		// Set of possible numbers in given line
		let mut possible_numbers = HashSet::<uint>::new();
		for i in range(0u, 3) {
            possible_numbers.extend(self.get(cornerX + i, y).possible_numbers.iter().map(|&n| n));
		}
		
		// Set of possible numbers in the rest of the square
		let (_, cornerY) = Sudoku::get_corner(cornerX, y);
		let mut other_numbers = HashSet::<uint>::new();
		for offY in range(0u, 3) {
			// Discard numbers in the row Y
			if cornerY + offY != y {
				for offX in range(0u, 3) {
                    other_numbers.extend(self.get(cornerX + offX, cornerY + offY).possible_numbers.iter().map(|&n| n));
				}
			}
		}
		
		possible_numbers.difference(&other_numbers).map(|&x| x).collect::<Vec<uint>>()
	}
	
	fn get_v_difference(&mut self, x: uint, cornerY: uint) -> Vec<uint> {
		// Set of possible numbers in given line
		let mut possible_numbers = HashSet::new();
		for i in range(0u, 3) {
            possible_numbers.extend(self.get(x, cornerY + i).possible_numbers.iter().map(|&n| n));
		}
		
		// Set of possible numbers in the rest of the square
		let (cornerX, _) = Sudoku::get_corner(x, cornerY);
		let mut other_numbers = HashSet::new();
		for offX in range(0u, 3) {
			// Discard numbers in the column X
			if cornerX + offX != x {
				for offY in range(0u, 3) {
                    other_numbers.extend(self.get(cornerX + offX, cornerY + offY).possible_numbers.iter().map(|&n| n));
				}
			}
		}
		
		// Difference
		possible_numbers.difference(&other_numbers).map(|&x| x).collect()
	}
	
	// Project a number horizontally to other squares
	fn project_h_line(&mut self, cornerX: uint, y: uint, projected_number: uint) -> bool {
		let mut progress = false;
		
		for x in range(0u, 9) {
			// Do not project to same squre
			if x < cornerX || cornerX + 3 <= x {
				progress = self.get_mut(x, y).cannot_be(projected_number) || progress;
			}
		}
		
		progress
	}
	
	// Project a number vertically to other squares
	fn project_v_line(&mut self, x: uint, cornerY: uint, projected_number: uint) -> bool {
		let mut progress = false;
		
		for y in range(0u, 9) {
			// Do not project to same square
			if y < cornerY || cornerY + 3 <= y {
				progress = self.get_mut(x, y).cannot_be(projected_number) || progress;
			}
		}
		
		progress
	}
}