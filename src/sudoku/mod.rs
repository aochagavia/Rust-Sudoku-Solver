/*

Implements the Sudoku struct, with the functionality to solve a sudoku.

There is a fast_solve method used to solve the sudoku without brute forcing it. If
it fails, you can use the brute_force method defined in brute_force.rs

For details about how the algorithm works, take a look at project_numbers.rs,
detect_uniques.rs, project_lines.rs and brute_force.rs

*/

use std::fmt;
use std::fmt::{Show, Formatter};
use std::io::BufferedReader;
use self::field::Field;

pub use self::brute_force::BruteForce;
use self::detect_uniques::DetectUniques;
use self::project_numbers::ProjectNumbers;
use self::project_lines::ProjectLines;

mod field;
pub mod brute_force;
mod detect_uniques;
mod project_numbers;
mod project_lines;

// Sudoku
#[deriving(Clone)]
pub struct Sudoku {
	fields: Vec<Vec<Field>>
}

impl Sudoku {
	pub fn new<T: Reader>(mut reader: BufferedReader<T>) -> Sudoku {
		// Use one column of 9 fields to fill 9 rows
		let column = Vec::from_elem(9, Field::new());
		let mut rows = Vec::from_elem(9, column.clone());

		// Read a row per line
		for y in range(0u, 9) {
			let line = reader.read_line().ok().unwrap_or("".to_string());
			let numbers = line.trim_right().chars().collect::<Vec<char>>();

			if numbers.len() < 9 {
				panic!("Invalid sudoku file! Line: {}", line.as_slice().trim_right());
			}

			// Values that cannot be parsed are interpreted as empty fields
			for x in range(0u, 9) {
				let parsed = from_str::<uint>(numbers[x].to_string().as_slice());
				if let Some(i) = parsed {
					rows[x][y].set_number(i);
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

    pub fn get(&self, x: uint, y: uint) -> &Field {
        &self.fields[x][y]
    }

    pub fn get_mut(&mut self, x: uint, y: uint) -> &mut Field {
        &mut self.fields[x][y]
    }

    // Check that the number in the given coordinates does not break
    // the sudoku condition
    fn is_valid(&self, x: uint, y: uint) -> bool {
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
        let (corner_x, corner_y) = Sudoku::get_corner(x, y);
        for off_x in range(0u, 3) {
            for off_y in range(0u, 3) {
                if corner_x + off_x != x || corner_y + off_y != y {
                    if self.get(corner_x + off_x, corner_y + off_y).number_found()
                    && self.get(corner_x + off_x, corner_y + off_y).get_number() == number {
                        return false;
                    }
                }
            }
        }

        true
    }
}

impl Show for Sudoku {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		for y in range(0u, 9) {
			if y == 3 || y == 6 {
				try!(write!(f, "{}\n", "-".repeat(11)));
			}
			for x in range(0u, 9) {
				if x == 3 || x == 6 {
					try!(write!(f, "|"));
				}

				if self.get(x, y).number_found() {
					try!(write!(f, "{}", self.get(x, y).get_number()))
				} else {
					try!(write!(f, " "))
				}
			}

			try!(write!(f, "\n"));
		}

        Ok(())
	}
}