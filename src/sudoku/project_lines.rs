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
	pub fn project_lines(&mut self) -> bool {
		let mut progress = false;
	
		// Check each square
		for x in range(0, 3) {
			for y in range(0, 3) {
				progress = self.check_lines_in_square(x * 3, y * 3);
			}
		}
		
		progress
	}
	
	fn check_lines_in_square(&mut self, cornerX: int, cornerY: int) -> bool {
		let mut progress = false;
	
		// Horizontal lines
		for y in range(0, 3) {
			let found_numbers = self.get_h_line_difference_square(cornerX, y);
			for &num in found_numbers.iter() {
				progress = self.project_square_line_h(cornerX, y, num) || progress;
			}
		}

		// Vertical lines
		for x in range(0, 3) {
			let found_numbers = self.get_v_line_difference_square(x, cornerY);
			for &num in found_numbers.iter() {
				progress = self.project_square_line_v(x, cornerY, num) || progress;
			}
		}
		
		progress
	}
	
	// Get the set of possible numbers in the given horizontal line, within the square
	// Take the difference with the rest of the square
	fn get_h_line_difference_square(&mut self, cornerX: int, y: int) -> ~[int] {		
		// Set of possible numbers in given line
		let mut possible_numbers = HashSet::<int>::new();
		for i in range(0, 3) {
			for &num in self.fields[cornerX + i][y].possible_numbers.iter() {
				possible_numbers.insert(num);
			}
		}
		
		// Rest
		let (_, cornerY) = Sudoku::get_corner(cornerX, y);
		let mut rest_numbers = HashSet::<int>::new();
		for offY in range(0, 3) {
			// Discard numbers in the row Y
			if cornerY + offY != y {
				for offX in range(0, 3) {
					// Add all other numbers of the square
					for &num in self.fields[cornerX + offX][cornerY + offY].possible_numbers.iter() {
						rest_numbers.insert(num);
					}
				}
			}
		}
		
		// Difference
		possible_numbers.difference(&rest_numbers).map(|i| i.clone()).to_owned_vec()
	}
	
	fn get_v_line_difference_square(&mut self, x: int, cornerY: int) -> ~[int] {
		// Set of possible numbers in given line
		let mut possible_numbers = HashSet::<int>::new();
		for i in range(0, 3) {
			for &num in self.fields[x][cornerY + i].possible_numbers.iter() {
				possible_numbers.insert(num);
			}
		}
		
		// Rest
		let (cornerX, _) = Sudoku::get_corner(x, cornerY);
		let mut rest_numbers = HashSet::<int>::new();
		for offX in range(0, 3) {
			// Discard numbers in the column X
			if cornerX + offX != x {
				for offY in range(0, 3) {
					// Add all other numbers of the square
					for &num in self.fields[cornerX + offX][cornerY + offY].possible_numbers.iter() {
						rest_numbers.insert(num);
					}
				}
			}
		}
		
		// Difference
		possible_numbers.difference(&rest_numbers).map(|i| i.clone()).to_owned_vec()
	}
	
	// Project a number horizontally to other squares
	fn project_square_line_h(&mut self, cornerX: int, y: int, projected_number: int) -> bool {
		let mut progress = false;
		
		for x in range(0, 9) {
			// Do not project to same squre
			if x < cornerX || cornerX + 3 <= x {
				progress = self.fields[x][y].remove_possibility(projected_number) || progress;
			}
		}
		
		progress
	}
	
	fn project_square_line_v(&mut self, x: int, cornerY: int, projected_number: int) -> bool {
		let mut progress = false;
		
		for y in range(0, 9) {
			// Do not project to same square
			if y < cornerY || cornerY + 3 <= y {
				progress = self.fields[x][y].remove_possibility(projected_number) || progress;
			}
		}
		
		progress
	}
}