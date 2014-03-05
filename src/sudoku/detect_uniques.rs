use std::hashmap::HashSet;
use sudoku::Sudoku;

/*

Implements methods to detect numbers which can only be in one field
If such a number is found, it will be assigned to the field and then projected as described in project_numbers

Example:

---|---|12-
6--|---|---
---|6--|---

will become

---|---|126
6--|---|---
---|6--|---

The coordinate (9, 1) is allowed to have all numbers from 3 to 9,
but it is the only field in which the number 6 can be.

*/

impl ::sudoku::Sudoku {
	pub fn detect_uniques(&mut self) -> bool {
		let mut progress = false;
	
		for x in range(0, 9) {
			for y in range(0, 9) {
				// Discard the field if we have already found a number for it
				if self.fields[x][y].number_found() {
					continue;
				}
				
				let current_set = self.fields[x][y].possible_numbers.clone();
				
				// Not optimal, but otherwise I couldn't get through the compiler
				let rest_set_v = &self.rest_set_v(x, y);
				let rest_set_h = &self.rest_set_h(x, y);
				let rest_set_square = &self.rest_set_square(x, y);
				progress = self.check_and_assign(x, y, &current_set, rest_set_v)
					|| self.check_and_assign(x, y, &current_set, rest_set_h)
					|| self.check_and_assign(x, y, &current_set, rest_set_square)
					|| progress;
			}
		}
		
		progress
	}
	
	pub fn check_and_assign(&mut self, x: int, y: int, current_set: &HashSet<int>, rest_set: &HashSet<int>) -> bool {
		let difference_vec = current_set.difference(rest_set).to_owned_vec();
		
		// If only one number survives, then we assign it to the current field and project it
		match difference_vec {
			[a] => {
				self.fields[x][y].set_number(a.clone());
				self.project_number(x, y);
				true
			}
			_ => { false }
		}
	}
	
	pub fn rest_set_v(&mut self, x: int, y: int) -> HashSet<int> {
		// Make a set with the possible numbers of all other fields in the line
		let mut rest_set = HashSet::<int>::new();
		for lineY in range(0, 9) {
			// Push only the values of the other fields
			if lineY != y {
				for &num in self.fields[x][lineY].possible_numbers.iter() {
					rest_set.insert(num);
				}
			}
		}
		
		rest_set
	}
	
	pub fn rest_set_h(&mut self, x: int, y: int) -> HashSet<int> {
		// Make a set with the possible numbers of all other fields in the line
		let mut rest_set = HashSet::<int>::new();
		for lineX in range(0, 9) {
			// Push only the values of the other fields
			if lineX != x {
				for &num in self.fields[lineX][y].possible_numbers.iter() {
					rest_set.insert(num);
				}
			}
		}
	
		rest_set
	}
	
	pub fn rest_set_square(&mut self, x: int, y: int) -> HashSet<int> {
		// Make a set with the possible numbers of all other fields in the square
		let mut rest_set = HashSet::<int>::new();
		let (cornerX, cornerY) = Sudoku::get_corner(x, y);
		for offsetX in range(0, 3) {
			for offsetY in range(0, 3) {
				// Push only the values of the other fields
				if cornerX + offsetX != x || cornerY + offsetY != y {
					for &num in self.fields[cornerX + offsetX][cornerY + offsetY].possible_numbers.iter() {
						rest_set.insert(num);
					}
				}
			}
		}

		rest_set
	}
}