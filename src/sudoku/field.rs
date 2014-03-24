use std::hashmap::HashSet;

// This is the basic unit of the sudoku
pub struct Field {
	possible_numbers: HashSet<int>,
	projected: bool
}

impl Field {
	pub fn new() -> Field {
		let mut set = HashSet::new();
		for i in range(1, 10) {
			set.insert(i);
		}
		
		Field { projected: false, possible_numbers: set }
	}

	// Returns true if a number has been found
	pub fn number_found(&self) -> bool {
		self.possible_numbers.len() == 1
	}
	
	// Sets the number of the current field
	pub fn set_number(&mut self, x: int) {
		self.possible_numbers.clear();
		self.possible_numbers.insert(x);
	}
	
	// Gets the number of the current field, if any
	// Fails if there is more than one possibility
	pub fn get_number(&self) -> int {
		match self.possible_numbers.iter().to_owned_vec() {
			[a] => { *a }
			_ => { fail!("Called get_number(), but there are many possible numbers") }
		}
	}
	
	// Removes a possibility from the field and returns true if it was contained
	pub fn cannot_be(&mut self, x: int) -> bool {
		// If there is only one possibility, it cannot be removed
		if self.possible_numbers.len() == 1 {
			return false;
		}
		
		let contains = self.possible_numbers.contains(&x);
		self.possible_numbers.remove(&x);
		
		contains
	}
	
	// Resets the possibilities to their default range [1, 9]
	pub fn reset_possibilities(&mut self) {
		self.possible_numbers.clear();
		for i in range(1,10) {
			self.possible_numbers.insert(i);
		}
	}
    
    // Give the field the next value available
    pub fn set_next_number(&mut self) -> bool {
        if !self.number_found() {
            self.set_number(1);
        } else {
            let number = self.get_number();
        
            // 9 is the last number available
            if number == 9 {
                self.reset_possibilities();
                return false;
            }
            
            self.set_number(number + 1);
        }
        
        return true;
    }
}

impl Clone for Field {
	fn clone(&self) -> Field {
		Field { projected: self.projected, possible_numbers: self.possible_numbers.clone() }
	}
}