/*

Implements the Sudoku struct, with the functionality to solve a sudoku.

There is a fast_solve method used to solve the sudoku without brute forcing it. If
it fails, you can use the brute_force method defined in brute_force.rs

For details about how the algorithm works, take a look at project_numbers.rs,
detect_uniques.rs, project_lines.rs and brute_force.rs

*/

use std::fmt::{Show, Formatter, Result};
use std::io::BufferedReader;
use sudoku::field::Field;

mod field;
mod project_numbers;
mod detect_uniques;
mod project_lines;
mod brute_force;

// Sudoku
#[deriving(Clone)]
pub struct Sudoku {
	fields: Vec<Vec<Field>>
}

impl Sudoku {
	pub fn new<T: Reader>(mut reader: BufferedReader<T>) -> Sudoku {
		// Use one column of 9 fields to fill 9 rows
		let column = Vec::from_fn(9, |_| Field::new());
		let mut rows = Vec::from_fn(9, |_| column.clone());
		
		// Read a row per line
		for y in range(0u, 9) {
			let line = reader.read_line().ok().unwrap_or("".to_owned());
			let numbers = line.trim_right().chars().collect::<Vec<char>>();
			
			if numbers.len() < 9 {
				fail!("Invalid sudoku file! Line: {}", line.trim_right());
			}
			
			// Values that cannot be parsed are interpreted as empty fields
			for x in range(0u, 9) {
				let parsed = from_str::<uint>(numbers.get(x).to_str());
				if parsed.is_some() {
					rows.get_mut(x).get_mut(y).set_number(parsed.unwrap());
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
					|| self.detect_uniques()
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
	pub fn get_corner(x: uint, y: uint) -> (uint, uint) {
		assert!(x < 9 && y < 9);
		((x / 3) * 3, (y / 3) * 3)
	}
    
    pub fn get<'a>(&'a self, x: uint, y: uint) -> &'a Field {
        self.fields.get(x).get(y)
    }
    
    pub fn get_mut<'a>(&'a mut self, x: uint, y: uint) -> &'a mut Field {
        self.fields.get_mut(x).get_mut(y)
    }
}

impl Show for Sudoku {
	fn fmt(&self, f: &mut Formatter) -> Result {
		let mut buf = StrBuf::new();
        
		for y in range(0u, 9) {
			if y == 3 || y == 6 {
				buf.push_str("-".repeat(12));
				buf.push_str("\n");
			}
			for x in range(0u, 9) {
				if x == 3 || x == 6 {
					buf.push_char('|');
				}
			
				buf.push_str(
				if self.get(x, y).number_found() {
					self.get(x, y).get_number().to_str()
				} else {
					" ".to_owned()
				});
			}
			
			buf.push_str("\n");
		}
	
        f.pad(buf.as_slice())
	}
}