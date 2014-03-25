/*

Implements the Sudoku struct, with the functionality to solve a sudoku.

There is a fast_solve method used to solve the sudoku without brute forcing it. If
it fails, you can use the brute_force method defined in brute_force.rs

For details about how the algorithm works, take a look at project_numbers.rs,
detect_uniques.rs, project_lines.rs and brute_force.rs

*/

use std::iter::Repeat;
use std::io::buffered::BufferedReader;
use sudoku::field::Field;

mod field;
mod project_numbers;
mod detect_uniques;
mod project_lines;
mod brute_force;

// Sudoku
pub struct Sudoku {
	fields: ~[~[Field]]
}

impl Sudoku {
	pub fn new<T: Reader>(mut reader: BufferedReader<T>) -> Sudoku {
		// Use one column of 9 fields to fill 9 rows
		let column = Repeat::new(Field::new()).take(9).to_owned_vec();
		let mut rows = Repeat::new(column).take(9).to_owned_vec();
		
		// Read a row per line
		for y in range(0, 9) {
			let line = reader.read_line().unwrap_or(~"");
			let numbers = line.trim_right().chars().to_owned_vec();
			
			if numbers.len() < 9 {
				fail!("Invalid sudoku file! Line: {}", line.trim_right());
			}
			
			// Values that cannot be parsed are interpreted as empty fields
			for x in range(0, 9) {
				let parsed = from_str::<int>(numbers[x].to_str());
				if parsed.is_some() {
					rows[x][y].set_number(parsed.unwrap());
				}
			}
		}
		
		Sudoku { fields: rows }
	}
	
	// Attempts to solve the sudoku without brute forcing it
	pub fn fast_solve(&mut self) {
		let mut progress = true;
		
		// If the functions cannot discover new numbers, they will return false
		while progress {
			progress = self.project_numbers()
					|| self.detect_uniques();
					|| self.project_lines();
		}
	}
	
	// Returns true if the sudoku is completed
	pub fn is_completed(&self) -> bool {
		self.fields.iter().all(|column| column.iter().all(|field|
			field.number_found())
		)
	}
	
	// Returns the top-left corner of the square in which the given point is
	pub fn get_corner(x: int, y: int) -> (int, int) {
		assert!(0 <= x && x < 9 && 0 <= y && y < 9);
		((x / 3) * 3, (y / 3) * 3)
	}
}

impl ToStr for Sudoku {
	fn to_str(&self) -> ~str {
		let mut string = ~"";
	
		for y in range(0, 9) {
			if y == 3 || y == 6 {
				string.push_str("-".repeat(12));
				string.push_str("\n");
			}
			for x in range(0, 9) {
				if x == 3 || x == 6 {
					string.push_char('|');
				}
			
				string.push_str(
				if self.fields[x][y].number_found() {
					self.fields[x][y].get_number().to_str()
				} else {
					~" "
				});
			}
			
			string.push_str("\n");
		}
	
		string
	}
}