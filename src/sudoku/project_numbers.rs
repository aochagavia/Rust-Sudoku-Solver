use sudoku::Sudoku;

/*

Implements methods to project each field
Projection means that we tell other fields which numbers they cannot contain
This will sometimes leave just one possibility, which becomes the number assigned to the field

Example:

123|456|78-

will become

123|456|789

If we project the numbers, we tell the blank field that it cannot be any number from 1 to 8
That means that it can only be 9

The same happens for squares:

123|	
456|
78-|	

will become

123|
456|
789|

*/

impl ::sudoku::Sudoku {
	// Projects all fields that are not empty and haven't yet been projected
	pub fn project_numbers(&mut self) -> bool {
		let mut progress = false;
		
		for x in range(0, 9) {
			for y in range(0, 9) {
				if (!self.fields[x][y].projected && self.fields[x][y].number_found()) {
					progress = self.project_number(x, y) || progress;
				}
			}
		}
		
		progress
	}
	
	// Will return true if we make progress so we can know if we are stuck
	pub fn project_number(&mut self, x: int, y: int) -> bool {
		self.fields[x][y].projected = true;
		return self.project_h_line(x, y)
			| self.project_v_line(x, y)
			| self.project_square(x, y);
	}
	
	// Project the number in its horizontal line
	fn project_h_line(&mut self, x: int, y: int) -> bool {
		let num = self.fields[x][y].get_number();
		let mut progress = false;
		for i in range(0, 9) {
			progress = self.fields[i][y].cannot_be(num) || progress;
		}
		
		return progress;
	}
	
	// Project the number in its vertical line
	fn project_v_line(&mut self, x: int, y: int) -> bool {
		let num = self.fields[x][y].get_number();
		let mut progress = false;
		for i in range(0, 9) {
			progress = self.fields[x][i].cannot_be(num) || progress;
		}
		
		return progress;
	}
	
	// Project the number in its square
	fn project_square(&mut self, x: int, y: int) -> bool {
		let num = self.fields[x][y].get_number();
		let mut progress = false;
		
		let (cX, cY) = Sudoku::get_corner(x, y);
		for i in range(cX, cX + 3) {
			for j in range(cY, cY + 3) {
				progress = self.fields[i][j].cannot_be(num) || progress;
			}
		}
		
		return progress;
	}
}