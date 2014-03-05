use std::iter::Repeat;
use std::io::buffered::BufferedReader;
use std::hashmap::HashSet;

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
		
		// Read a row per line in the reader
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
		
		return Sudoku { fields: rows };
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
		match ((x, y)) {
			(a, b) if a < 3 && b < 3 => { (0, 0) }

			(a, b) if 3 <= a && a < 6 && b < 3 => { (3, 0)	}
			
			(a, b) if 6 <= a && b < 3 => { (6, 0) }
			
			(a, b) if a < 3 && 3 <= b && b < 6 => { (0, 3) }
			
			(a, b) if a < 3 && 6 <= b => { (0, 6) }
			
			(a, b) if 3 <= a && a < 6 && 3 <= b && b < 6 => { (3, 3) }
			
			(a, b) if 3 <= a && a < 6 && 6 <= b => { (3, 6) }
			
			(a, b) if 6 <= a && 3 <= b && b < 6 => { (6, 3) }
			
			(a, b) if 6 <= a && 6 <= b => { (6, 6) }
			
			(_, _) => { fail!("Corner of {}, {} could not be found", x, y) }
		}
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
	
		return string;
	}
}


// Field
struct Field {
	possible_numbers: HashSet<int>,
	projected: bool
}

impl Field {
	fn new() -> Field {
		let mut set = HashSet::new();
		for i in range(1, 10) {
			set.insert(i);
		}
		
		Field { projected: false, possible_numbers: set }
	}

	// Returns true if a number has been found
	fn number_found(&self) -> bool {
		return self.possible_numbers.len() == 1;
	}
	
	// Sets the number of the current field
	fn set_number(&mut self, x: int) {
		self.possible_numbers.clear();
		self.possible_numbers.insert(x);
	}
	
	// Gets the number of the current field, if any
	// Fails if there is more than one possibility
	fn get_number(&self) -> int {
		match self.possible_numbers.iter().to_owned_vec() {
			[a] => { *a }
			_ => { fail!("Called get_number(), but there are many possible numbers") }
		}
	}
	
	// Removes a possibility from the field and returns true if it was contained
	fn remove_possibility(&mut self, x: int) -> bool {
		if self.possible_numbers.len() == 1 {
			return false;
		}
		
		let contains = self.possible_numbers.contains(&x);
		self.possible_numbers.remove(&x);
		return contains;
	}
	
	// Resets the possibilities to their default range
	fn reset_possibilities(&mut self) {
		self.possible_numbers.clear();
		for i in range(1,10) {
			self.possible_numbers.insert(i);
		}
	}
}

impl Clone for Field {
	fn clone(&self) -> Field {
		Field { projected: self.projected, possible_numbers: self.possible_numbers.clone() }
	}
}