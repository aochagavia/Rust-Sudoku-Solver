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
	// Detect 
	pub fn detect_uniques(&mut self) -> bool {
		let mut progress = false;
	
		for x in range(0, 9) {
			for y in range(0, 9) {
				// Discard the field if we have already found a number for it
				if self.fields[x][y].number_found() {
					continue;
				}
				
				let possible_numbers = self.fields[x][y].possible_numbers.clone();
				
				// Not optimal, but otherwise I couldn't get through the compiler
				let other_numbers_v = self.other_numbers_v(x, y);
				let other_numbers_h = self.other_numbers_h(x, y);
				let other_numbers_square = self.other_numbers_square(x, y);
				
				progress = self.check_and_assign(x, y, &possible_numbers, &other_numbers_v)
					|| self.check_and_assign(x, y, &possible_numbers, &other_numbers_h)
					|| self.check_and_assign(x, y, &possible_numbers, &other_numbers_square)
					|| progress;
			}
		}
		
		progress
	}
	
	// Check if the set difference between the possible_numbers of the current field
	// and the other_numbers leaves a single value
	// If that is the case assign it to the field in the given coordinates and project it
	pub fn check_and_assign(&mut self, x: int, y: int, possible_numbers: &HashSet<int>, other_numbers: &HashSet<int>) -> bool {
		let difference = possible_numbers.difference(other_numbers).to_owned_vec();
		match difference {
			[a] => {
				self.fields[x][y].set_number(a.clone());
				self.project_number(x, y);
				true
			}
			_ => { false }
		}
	}
	
	// Get a set with the possible numbers of all fields in the vertical line,
	// discarding the number located in the given coordinates
	pub fn other_numbers_v(&mut self, x: int, y: int) -> HashSet<int> {
		let mut other_numbers = HashSet::<int>::new();
		for offY in range(0, 9) {
			if offY != y {
				for &num in self.fields[x][offY].possible_numbers.iter() {
					other_numbers.insert(num);
				}
			}
		}
		
		other_numbers
	}
	
	// Get a set with the possible numbers of all fields in the horizontal line,
	// discarding the number located in the given coordinates
	pub fn other_numbers_h(&mut self, x: int, y: int) -> HashSet<int> {
		let mut other_numbers = HashSet::<int>::new();
		for offX in range(0, 9) {
			if offX != x {
				for &num in self.fields[offX][y].possible_numbers.iter() {
					other_numbers.insert(num);
				}
			}
		}
	
		other_numbers
	}
	
	// Get a set with the possible numbers of all fields in the square,
	// discarding the number located in the given coordinates
	pub fn other_numbers_square(&mut self, x: int, y: int) -> HashSet<int> {
		let mut other_numbers = HashSet::<int>::new();
		let (cornerX, cornerY) = Sudoku::get_corner(x, y);
		for offX in range(0, 3) {
			for offY in range(0, 3) {
				// Push only the values of the other fields
				if cornerX + offX != x || cornerY + offY != y {
					for &num in self.fields[cornerX + offX][cornerY + offY].possible_numbers.iter() {
						other_numbers.insert(num);
					}
				}
			}
		}

		other_numbers
	}
}