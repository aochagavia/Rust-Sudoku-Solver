use sudoku::Sudoku;

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

struct Point { x: int, y: int }

impl ::sudoku::Sudoku {
	// Attempts to brute force the sudoku.
	// Returns true if it works and false if not.
	pub fn brute_force(&mut self) -> bool {
		// Get the coordinates of the empty fields
		let empty_fields = self.get_empty_fields();
		
		// Assign numbers to them recursively
		self.assign_field(0, empty_fields)
	}
	
	// Recursive function to brute force the empty fields
	fn assign_field(&mut self, current: uint, empty_fields: &[Point]) -> bool {
		// If the current field is out of bounds, the sudoku is completed
		if current == empty_fields.len() {
			return true;
		}
	
		let (x, y) = (empty_fields[current].x, empty_fields[current].y);
	
		loop {
			// Give the field the next value available
			if !self.fields[x][y].number_found() {
				self.fields[x][y].set_number(1);
			} else {
				// 9 is the last number available
				if self.fields[x][y].get_number() == 9 {
					self.fields[x][y].reset_possibilities();
					return false;
				}
				
				let number = self.fields[x][y].get_number();
				self.fields[x][y].set_number(number + 1);
			}
			
			// If the condition is not broken, solve the next field
			// If it is broken, continue with the loop and test the next available number
			if self.is_valid(x, y)
			&& self.assign_field(current + 1, empty_fields) {
				return true;
			}
		}
	}
	
	fn is_valid(&mut self, x: int, y: int) -> bool {
		let number = self.fields[x][y].get_number();
	
		// Check that the number is not repeated in the horizontal line
		for i in range(0, 9) {
			if i != x
			&& self.fields[i][y].number_found()
			&& self.fields[i][y].get_number() == number {
				return false;
			}
		}
		
		// Check that it is not repeated in the vertical line
		for i in range(0, 9) {
			if i != y
			&& self.fields[x][i].number_found()
			&& self.fields[x][i].get_number() == number {
				return false;
			}
		}
		
		// Check that it is not repeated in the square
		let (cornerX, cornerY) = Sudoku::get_corner(x, y);
		for offX in range(0, 3) {
			for offY in range(0, 3) {
				if cornerX + offX != x || cornerY + offY != y {
					if self.fields[cornerX + offX][cornerY + offY].number_found()
					&& self.fields[cornerX + offX][cornerY + offY].get_number() == number {
						return false;
					}
				}
			}
		}
		
		true
	}
	
	fn get_empty_fields(&mut self) -> ~[Point] {
		let mut points: ~[Point] = ~[];
		for x in range(0, 9) {
			for y in range(0, 9) {
				if !self.fields[x][y].number_found() {
					points.push(Point{ x: x, y: y });
				}
			}
		}
		
		points
	}
}