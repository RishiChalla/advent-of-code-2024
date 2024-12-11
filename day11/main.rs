use core::fmt;
use std::{fmt::{Display, Formatter}, num::ParseIntError};


/// A single stone in the arrangement
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Stone {
	engraving: usize,
}

impl TryFrom<&str> for Stone {
    type Error = ParseIntError;

    fn try_from(value: &str) -> Result<Self, ParseIntError> {
        Ok(Self { engraving: value.parse()? })
    }
}

impl Display for Stone {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.engraving.to_string().as_str())
    }
}

impl Stone {
	/// Updates this single stone during a blink.
	/// - If Engraving 0 -> (1, None)
	/// - Else if **Number of Digits** in Engraving is Even -> Digits split in half, (first half, second half).
	/// - Else Engraving -> (Multiplied by 2024, None)
	fn blink_update(&self) -> [Option<Stone>; 2] {
		let mut engraving_str = self.engraving.to_string();
		match (self.engraving, engraving_str.len() % 2 == 0) {
			(0, _) => [Some(Self { engraving: 1 }), None],
			(_, true) => {
				// Split off returns the second half, and mutates the string to be the first half
				let second = Self { engraving: engraving_str.split_off(engraving_str.len() / 2).parse().unwrap() };
				let first = Self { engraving: engraving_str.parse().unwrap() };
				[Some(first), Some(second)]
			},
			_ => [Some(Self { engraving: self.engraving * 2024 }), None]
		}
	}
}

/// Full ordered line of stones, changes during a "blink", which causes an update.
#[derive(Debug)]
struct Arrangement {
	stones: Vec<Stone>,
}

impl TryFrom<&str> for Arrangement {
    type Error = ParseIntError;

    fn try_from(value: &str) -> Result<Self, ParseIntError> {
        Ok(Self { stones: value.split(' ').map(Stone::try_from).collect::<Result<Vec<_>, _>>()? })
    }
}

impl Display for Arrangement {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.stones.iter().map(|stone| stone.to_string()).collect::<Vec<_>>().join(" ").as_str())
    }
}

impl Arrangement {
	/// Updates the full arrangement of stones during a blink.
	fn blink_update(&mut self) {
		self.stones = self.stones.iter()
			.flat_map(|stone| {
				stone.blink_update().into_iter().flatten()
			})
			.collect();
	}
}

fn part1_solution(input: &str) -> Result<usize, ParseIntError> {
	let mut line = Arrangement::try_from(input)?;
	for _ in 0..25 { line.blink_update(); }
	Ok(line.stones.len())
}

/// Entry point
pub fn main() {
	let example = "125 17";
	let input = "872027 227 18 9760 0 4 67716 9245696";

    println!("Part 1 Solution on Example: {:#?}", part1_solution(example));
	println!("Part 1 Solution on Input: {:#?}", part1_solution(input));

    // println!("Part 2 Solution on Example: {:#?}", part2_solution(example));
	// println!("Part 2 Solution on Input: {:#?}", part2_solution(input));
}
