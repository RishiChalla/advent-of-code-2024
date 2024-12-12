use std::collections::HashMap;

/// Describes a single stone
struct Stone {
	/// Each blink, this engraving will subdivide into the next item in the list, until all items are single digits.
	digits: Vec<Vec<usize>>,
}

impl Stone {
	/// Creates a new stone
	fn new(digits: Vec<Vec<usize>>) -> Self {
		Self { digits }
	}
}

/// Solver for day 11
struct Day11 {
	/// Static digit map used for quick cached access, contains subdivision modification of all single-digits
	digit_map: HashMap<usize, Stone>,
}

impl Day11 {

	/// Creates a new solver for day 11
	fn new() -> Self {
		Self {
			digit_map: HashMap::from([
				(0, Stone::new(vec![vec![1]])),
				(1, Stone::new(vec![vec![2024], vec![20, 24], vec![2, 0, 2, 4]])),
				(2, Stone::new(vec![vec![4048], vec![40, 48], vec![4, 0, 4, 8]])),
				(3, Stone::new(vec![vec![6072], vec![60, 72], vec![6, 0, 7, 2]])),
				(4, Stone::new(vec![vec![8096], vec![80, 96], vec![8, 0, 9, 6]])),
				(5, Stone::new(vec![vec![10120], vec![20482880], vec![2048, 2880], vec![20, 48, 28, 80], vec![2, 0, 4, 8, 2, 8, 8, 0]])),
				(6, Stone::new(vec![vec![12144], vec![24579456], vec![2457, 9456], vec![24, 57, 94, 56], vec![2, 4, 5, 7, 9, 4, 5, 6]])),
				(7, Stone::new(vec![vec![14168], vec![28676032], vec![2867, 6032], vec![28, 67, 60, 32], vec![2, 8, 6, 7, 6, 0, 3, 2]])),
				(9, Stone::new(vec![vec![18216], vec![36869184], vec![3686, 9184], vec![36, 86, 91, 84], vec![3, 6, 8, 6, 9, 1, 8, 4]])),
				// 8 is a special case since it actually recurses due to a leading 0 in one of the subdivisions.
				// 8 * 2024 is 16192 which has a cleaner
				(8, Stone::new(vec![vec![16192]])),
				(16192, Stone::new(vec![vec![32772608], vec![3277, 2608], vec![32, 77, 26, 8], vec![3, 2, 7, 7, 2, 6, 16192]])),
			])
		}
	}

	/// Counts the number of stones this stone would subdivide into after a certain amount of blinks.
	/// For each blink:
	/// - If Engraving 0 -> 1
	/// - Else if **Number of Digits** in Engraving is Even -> Digits split in half, (first half, second half).
	/// - Else Engraving -> Multiplied by 2024
	fn count_after_blinks(&self, engraving: usize, blinks: usize) -> usize {
		// Handle trivial case
		if blinks == 0 { return 1 }

		// Single digit cases have optimized lookups for quicker higher-blink recursion
		if let Some(stone) = self.digit_map.get(&engraving) {
			// We know how many blinks it takes to become length power of 2 - which can be subdivided into single digits
			// Check if the number of blinks is prior to single-digit subdivsion
			if let Some(digits) = stone.digits.get(blinks - 1) { return digits.len() }
			// The number of blinks is more than the single-digit subdivision, we need to recurse.
			let digits = stone.digits.last().unwrap();
			let blinks = blinks - stone.digits.len();
			digits.iter().map(|&digit| self.count_after_blinks(digit, blinks)).sum()
		} else {
			// It is not a single digit, we need to split it normally and recurse until it becomes a single digit.
			let mut engraving_str = engraving.to_string();
			if engraving_str.len() % 2 == 0 {
				// Split off returns the second half, and mutates the string to be the first half
				let second = engraving_str.split_off(engraving_str.len() / 2).parse().unwrap();
				let first = engraving_str.parse().unwrap();
				self.count_after_blinks(first, blinks - 1) + self.count_after_blinks(second, blinks - 1)
			} else {
				// Multiply by 2024
				self.count_after_blinks(engraving * 2024, blinks - 1)
			}
		}
	}
	
	/// Counts the number of stones the input stones would subdivide into after a certain number of blinks.
	fn count_arrangement_after_blinks(&self, input: &[usize], blinks: usize) -> usize {
		input.iter().map(|&engraving| self.count_after_blinks(engraving, blinks)).sum()
	}
}


/// Entry point
pub fn main() {
	let solver = Day11::new();
	let example = vec![125, 17];
	let input = vec![872027, 227, 18, 9760, 0, 4, 67716, 9245696];

	println!("Part 1 Solution on Example: {:#?}", solver.count_arrangement_after_blinks(&example, 25));
	println!("Part 1 Solution on Input: {:#?}", solver.count_arrangement_after_blinks(&input, 25));

	println!("Part 2 Solution on Example: {:#?}", solver.count_arrangement_after_blinks(&example, 75));
	println!("Part 2 Solution on Input: {:#?}", solver.count_arrangement_after_blinks(&input, 75));
}
