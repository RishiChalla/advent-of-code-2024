use std::fmt::{Display, Write};

/// Represents a tile on the map.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
	Obsticle,
	Freespace { visited: bool },
	Guard,
}

impl Tile {
	/// Creates a tile from a character
	fn from_char(value: char) -> Option<Self> {
		match value {
			'#' => Some(Self::Obsticle),
			'.' => Some(Self::Freespace { visited: false }),
			'^' => Some(Self::Guard),
			_ => None
		}
	}

	/// Checks if the tile has been visited
	fn is_visited(&self) -> bool {
		match self {
			Tile::Obsticle => false,
			Tile::Freespace { visited } => *visited,
			Tile::Guard => true,
		}
	}
}

impl Display for Tile {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Tile::Obsticle => f.write_char('#'),
			Tile::Freespace { visited } => if *visited { f.write_char('X') } else { f.write_char('.') },
			Tile::Guard => f.write_char('^'),
		}
	}
}

/// Possible errors during map traversal
#[derive(Debug)]
pub enum TraversalError {
	GuardNotFound,
}

/// Represents the full map in the puzzle.
struct Map {
	/// 2d array containing the map.
	map: Vec<Vec<Tile>>,
}

impl Display for Map {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for row in self.map.iter() {
			f.write_char('\n')?;
			for tile in row { Display::fmt(tile, f)? };
		};
		Ok(())
	}
}

impl Map {

	/// Creates a map from a string.
	fn from_string(input: &String) -> Option<Self> {
		let mut map = Self {
			map: input.lines()
				.map(|line| line.chars().map(|c| Tile::from_char(c)).collect::<Option<Vec<Tile>>>())
				.collect::<Option<Vec<Vec<Tile>>>>()?
		};
		map.rotate_right();
		Some(map)
	}

	/// Rotates a 2d array rightt
	fn rotate_right(&mut self) {
		self.map = (0..self.map[0].len())
			.map(|i| self.map.iter().rev().map(|row| row[i].clone()).collect())
			.collect()
	}
	
	/// Rotates a 2d array left
	fn rotate_left(&mut self) {
		self.map = (0..self.map[0].len())
			.rev()
			.map(|i| self.map.iter().map(|row| row[i].clone()).collect())
			.collect()
	}

	/// Traverses the map by one step. Returns whether or not we can traverse further (true when we can still traverse)
	fn traverse(&mut self) -> Result<bool, TraversalError> {
		// Row the guard is in, and the x position of the guard.
		let (x, y, row) = self.map.iter_mut()
			.enumerate()
			.find_map(|(y, row)| Some((row.iter().position(|c| *c == Tile::Guard)?, y, row)))
			.ok_or(TraversalError::GuardNotFound)?;

		// Check if there's an obsticle in the guard's path
		if let Some(obsticle) = row.iter().skip(x).position(|c| *c == Tile::Obsticle) {
			// Obsticle found, go to it
			row.iter_mut().skip(x).take(obsticle).for_each(|tile| {
				*tile = Tile::Freespace { visited: true }
			});
			row[x+obsticle-1] = Tile::Guard;
			self.rotate_left();
			Ok(true)
		} else {
			// There is no obsticle; We've exited the map.
			row.iter_mut().skip(x).for_each(|tile| {
				if *tile != Tile::Obsticle { *tile = Tile::Freespace { visited: true } }
			});
			Ok(false)
		}
	}

	/// Counts the number of tiles that have been traversed thus far
	fn count_traversed(&self) -> usize {
		self.map.iter().flatten().filter(|&&tile| tile.is_visited()).count()
	}


}

/// Possible errors in the part 1 solution.
#[derive(Debug)]
pub enum Part1Error {
	TraversalError(TraversalError),
	MapParsingError,
	MaxIterationsReached,
}

/// Part 1 solution to the advent of code day 6.
pub fn part1_solution(input: &String, max_iters: usize) -> Result<usize, Part1Error> {
	let mut map = Map::from_string(input).ok_or(Part1Error::MapParsingError)?;
	for _ in 0..max_iters {
		if !map.traverse().map_err(|err| Part1Error::TraversalError(err))? {
			return Ok(map.count_traversed());
		}
	}
	Err(Part1Error::MaxIterationsReached)
}

/// Possible errors in the part 2 solution.
#[derive(Debug)]
pub enum Part2Error {
	GuardNotFound,
	MaxIterationsReached,
}

/// Part 2 solution to the advent of code day 6.
pub fn part2_solution(input: &String, max_iters: usize) -> Result<usize, Part2Error> {
	// let mut map = rotate_right(input_to_map(input));
	// for _ in 0..max_iters {
	// 	// This code prints the current map.
	// 	// println!("{}", map.iter().map(|x| x.iter().collect()).collect::<Vec<String>>().join("\n"));

	// 	// Row the guard is in, and the x position of the guard.
	// 	let (x, y, row) = map.iter()
	// 		.enumerate()
	// 		.find_map(|(y, row)| Some((row.iter().position(|c| *c == '^')?, y, row)))
	// 		.ok_or(Part2Error::GuardNotFound)?;

	// 	// Check if there's an obsticle in the guard's path
	// 	if let Some(obsticle) = row.iter().skip(x).position(|c| *c == '#') {
	// 		// Obsticle found, go to it
	// 		map[y][x] = '.';
	// 		map[y][x+obsticle-1] = '^';
	// 		map = rotate_left(map);
	// 	} else {
	// 		// There is no obsticle; We've exited the map.
	// 		break;
	// 	}
	// }

	todo!()
}

pub fn main() {
	let example = String::from("....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...");
	let input = String::from(include_str!("day6.txt"));

	println!("Part 1 solution for Example {:#?}", part1_solution(&example, 20));
	println!("Part 1 solution for Input {:#?}", part1_solution(&input, 10000));

	// println!("Part 2 solution for Example {:#?}", part2_solution(&example, 20));
	// println!("Part 2 solution for Input {:#?}", part2_solution(&input, 10000));
}
