use std::hashmap::HashSet;
use sudoku::Sudoku;

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

*/

impl ::sudoku::Sudoku {
	// Checks each square to see if it contains any lines that can be projected
	pub fn project_lines(&mut self) -> bool {
		let mut progress = false;
	
		for x in range(0, 3) {
			for y in range(0, 3) {
				progress = self.check_square(x * 3, y * 3) || progress;
			}
		}
		
		progress
	}
	
	// Check a single square to see if it contains any lines that can be projected
	// If such lines are found, project them
	fn check_square(&mut self, cornerX: int, cornerY: int) -> bool {
		let mut progress = false;
	
		// Horizontal lines
		for y in range(0, 3) {
			let found_numbers = self.get_h_difference(cornerX, y);
			for &num in found_numbers.iter() {
				progress = self.project_h_line(cornerX, y, num) || progress;
			}
		}

		// Vertical lines
		for x in range(0, 3) {
			let found_numbers = self.get_v_difference(x, cornerY);
			for &num in found_numbers.iter() {
				progress = self.project_v_line(x, cornerY, num) || progress;
			}
		}
		
		progress
	}
	
	// Get the set of possible numbers in the given horizontal line, within the square
	// and take the difference with the rest of the square
	fn get_h_difference(&mut self, cornerX: int, y: int) -> ~[int] {		
		// Set of possible numbers in given line
		let mut possible_numbers = HashSet::<int>::new();
		for i in range(0, 3) {
			for &num in self.fields[cornerX + i][y].possible_numbers.iter() {
				possible_numbers.insert(num);
			}
		}
		
		// Set of possible numbers in the rest of the square
		let (_, cornerY) = Sudoku::get_corner(cornerX, y);
		let mut other_numbers = HashSet::<int>::new();
		for offY in range(0, 3) {
			// Discard numbers in the row Y
			if cornerY + offY != y {
				for offX in range(0, 3) {
					for &num in self.fields[cornerX + offX][cornerY + offY].possible_numbers.iter() {
						other_numbers.insert(num);
					}
				}
			}
		}
		
		possible_numbers.difference(&other_numbers).map(|i| i.clone()).to_owned_vec()
	}
	
	fn get_v_difference(&mut self, x: int, cornerY: int) -> ~[int] {
		// Set of possible numbers in given line
		let mut possible_numbers = HashSet::<int>::new();
		for i in range(0, 3) {
			for &num in self.fields[x][cornerY + i].possible_numbers.iter() {
				possible_numbers.insert(num);
			}
		}
		
		// Set of possible numbers in the rest of the square
		let (cornerX, _) = Sudoku::get_corner(x, cornerY);
		let mut other_numbers = HashSet::<int>::new();
		for offX in range(0, 3) {
			// Discard numbers in the column X
			if cornerX + offX != x {
				for offY in range(0, 3) {
					for &num in self.fields[cornerX + offX][cornerY + offY].possible_numbers.iter() {
						other_numbers.insert(num);
					}
				}
			}
		}
		
		// Difference
		possible_numbers.difference(&other_numbers).map(|i| i.clone()).to_owned_vec()
	}
	
	// Project a number horizontally to other squares
	fn project_h_line(&mut self, cornerX: int, y: int, projected_number: int) -> bool {
		let mut progress = false;
		
		for x in range(0, 9) {
			// Do not project to same squre
			if x < cornerX || cornerX + 3 <= x {
				progress = self.fields[x][y].cannot_be(projected_number) || progress;
			}
		}
		
		progress
	}
	
	// Project a number vertically to other squares
	fn project_v_line(&mut self, x: int, cornerY: int, projected_number: int) -> bool {
		let mut progress = false;
		
		for y in range(0, 9) {
			// Do not project to same square
			if y < cornerY || cornerY + 3 <= y {
				progress = self.fields[x][y].cannot_be(projected_number) || progress;
			}
		}
		
		progress
	}
}