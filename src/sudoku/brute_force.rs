/*

Implements a brute force algorithm to solve the sudoku when the
fast_solve method fails.

The steps are the following:
1- Get all coordinates of empty fields
2- Loop through each of them
	Assign a number, check if it is valid for the sudoku
	If it is valid, assign the next field
	If it is invalid, go back to the previous field and assign the next valid number
3- If we reach the last field and we can assign it a valid number, the sudoku is completed
4- If we get back to the first field and we cannot assign it a valid number, the sudoku is not valid

*/

use sudoku::Sudoku;
use std::slice::Items;

struct Point(uint, uint);

impl ::sudoku::Sudoku {
	// Attempts to brute force the sudoku.
	// Returns true if it works and false if not.
	pub fn brute_force(&mut self) -> bool {
		// Get a vector of tuples containing the coordinates of each empty field
		let empty_fields = self.get_empty_fields();
		
		// Assign numbers to them recursively
		self.assign_field(empty_fields.iter())
	}
	
	// Recursive function to brute force the empty fields
	fn assign_field(&mut self, mut empty_fields: Items<Point>) -> bool {
		// If all empty fields are assigned without errors, the sudoku is completed
        let x; let y;
        match empty_fields.next() {
            Some(&Point(field_x, field_y)) => { x = field_x; y = field_y; }
            None => { return true }
        }
	
        // set_next_number will return false when there is no number left to be assigned
		while self.get_mut(x, y).set_next_number() {
            // If the condition is not broken, assign the next field
            // If it is broken, test with the next available number
            if self.is_valid(x, y)
            && self.assign_field(empty_fields) {
                return true;
            }
		}
        
        false
	}
	
	// Check that the number in the given coordinates does not break
	// the sudoku condition
	fn is_valid(&mut self, x: uint, y: uint) -> bool {
		let number = self.get(x, y).get_number();
	
		// Check horizontal line
		for i in range(0u, 9) {
			if i != x
			&& self.get(i, y).number_found()
			&& self.get(i, y).get_number() == number {
				return false;
			}
		}
		
		// Check vertical line
		for i in range(0u, 9) {
			if i != y
			&& self.get(x, i).number_found()
			&& self.get(x, i).get_number() == number {
				return false;
			}
		}
		
		// Check square
		let (cornerX, cornerY) = Sudoku::get_corner(x, y);
		for offX in range(0u, 3) {
			for offY in range(0u, 3) {
				if cornerX + offX != x || cornerY + offY != y {
					if self.get(cornerX + offX, cornerY + offY).number_found()
					&& self.get(cornerX + offX, cornerY + offY).get_number() == number {
						return false;
					}
				}
			}
		}
		
		true
	}
	
	fn get_empty_fields(&mut self) -> Vec<Point> {
		let mut points = vec!();
        
		for x in range(0u, 9) {
			for y in range(0u, 9) {
				if !self.get(x, y).number_found() {
					points.push(Point(x, y));
				}
			}
		}
		
		points
	}
}