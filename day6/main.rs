use std::fmt::{Display, Write};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

/// Traversal directions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
	North, East, South, West,
}

impl Direction {
	/// Gets the direction by rotating right from the current direction
	fn get_right_direction(&self) -> Self {
		match self {
			Direction::North => Direction::East,
			Direction::East => Direction::South,
			Direction::South => Direction::West,
			Direction::West => Direction::North,
		}
	}

	/// Turns this direction right.
	fn go_right(&mut self) {
		*self = self.get_right_direction();
	}

	/// Gets the index in the tile visited array.
	fn get_visited_index(&self) -> usize {
		match self {
			Direction::North => 0,
			Direction::East => 1,
			Direction::South => 2,
			Direction::West => 3,
		}
	}
}

/// Represents a tile on the map.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
	Obsticle,
	Freespace { visited: [bool; 4] },
	/// Guard must persist the existing visited data, there was a sneaky bug here where setting Guard overrided all previous existing data.
	/// This bug wasted several hours of my life, ~3am EST Saturday December 7th.
	Guard { visited: [bool; 4] },
}

impl Tile {
	/// Creates a tile from a character
	fn from_char(value: char) -> Option<Self> {
		match value {
			'#' => Some(Self::Obsticle),
			'.' => Some(Self::Freespace { visited: [false; 4] }),
			'^' => Some(Self::Guard { visited: [false; 4] }),
			_ => None
		}
	}

	/// Checks if the tile has been visited
	fn is_visited(&self) -> bool {
		match self {
			Tile::Obsticle => false,
			Tile::Freespace { visited } => visited.iter().any(|x| *x),
			Tile::Guard { visited: _ } => true,
		}
	}

	/// Checks whether the tile has been traversed in a certain direction.
	fn is_traversed(&self, direction: Direction) -> bool {
		match self {
			Tile::Obsticle => false,
			Tile::Freespace { visited } => visited[direction.get_visited_index()],
			Tile::Guard { visited: _ } => false,
		}
	}

	/// Marks the tile as traversed, returns None if the tile is an obsticle.
	fn set_traversed(&mut self, direction: Direction) -> Option<()> {
		match self {
			Tile::Obsticle => None,
			Tile::Freespace { visited } => {
				visited[direction.get_visited_index()] = true;
				Some(())
			},
			Tile::Guard { visited } => {
				visited[direction.get_visited_index()] = true;
				*self = Tile::Freespace { visited: *visited };
				Some(())
			},
		}
	}

	/// Changes the current tile to hold the guard. Returns None if this tile is an obsticle
	fn set_guard(&mut self) -> Option<()> {
		match self {
			Tile::Obsticle => None,
			Tile::Freespace { visited } => {
				*self = Tile::Guard { visited: *visited };
				Some(())
			},
			Tile::Guard { visited: _ } => Some(()),
		}
	}

	/// Returns true if this tile contains a guard.
	fn is_guard(&self) -> bool {
		matches!(*self, Tile::Guard { visited: _ })
	}
}

impl Display for Tile {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Tile::Obsticle => f.write_char('#'),
			Tile::Freespace { visited } => if visited.iter().any(|x| *x) { f.write_char('X') } else { f.write_char('.') },
			Tile::Guard { visited: _ } => f.write_char('^'),
		}
	}
}

/// Possible errors during a single map traversal step
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TraversalStepError {
	GuardNotFound,
	InvalidObsticleEncountered,
	TraversalUpdateError,
	InfiniteLoopEncountered,
}

/// Possible errors during map traversal
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TraversalError {
	TraversalStepError(TraversalStepError),
	MaxIterationsReached,
}

/// Represents the full map in the puzzle. There is a grid of a Guard, Free spaces which can be moved on, and obsticles.
/// Upon encountering any obsticle, the guard turns right and continues.
#[derive(Clone)]
struct Map {
	/// 2d array containing the map.
	map: Vec<Vec<Tile>>,
	/// The direction we're currently travelling.
	direction: Direction,
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
	fn from_string(input: &str) -> Option<Self> {
		let mut map = Self {
			map: input.lines()
				.map(|line| line.chars().map(Tile::from_char).collect::<Option<Vec<Tile>>>())
				.collect::<Option<Vec<Vec<Tile>>>>()?,
			direction: Direction::North,
		};
		map.rotate_right();
		Some(map)
	}

	/// Rotates a 2d array rightt
	fn rotate_right(&mut self) {
		self.map = (0..self.map[0].len())
			.map(|i| self.map.iter().rev().map(|row| row[i]).collect())
			.collect()
	}
	
	/// Rotates a 2d array left
	fn rotate_left(&mut self) {
		self.map = (0..self.map[0].len())
			.rev()
			.map(|i| self.map.iter().map(|row| row[i]).collect())
			.collect()
	}

	/// Traverses the map by one step.
	/// Returns a tuple of:
	/// - Vec(y, x) of all locations traversed in this step
	/// - whether or not we can traverse further (true when we can still traverse)
	fn traverse(&mut self) -> Result<(Vec<(usize, usize)>, bool), TraversalStepError> {
		// Row the guard is in, and the x position of the guard.
		let (y, x, row) = self.map.iter_mut()
			.enumerate()
			.find_map(|(y, row)| Some((y, row.iter().position(|c| c.is_guard())?, row)))
			.ok_or(TraversalStepError::GuardNotFound)?;

		let mut traversed = Vec::new();

		// Check if there's an obsticle in the guard's path
		let obsticle_index = {
			let mut pos = None;
			for (x, tile) in row.iter_mut().enumerate().skip(x) {
				if tile.is_traversed(self.direction) { return Err(TraversalStepError::InfiniteLoopEncountered); }
				if tile.set_traversed(self.direction).is_none() { pos = Some(x); break; }
				else { traversed.push((y, x)); }
			}
			pos
		};

		if let Some(obsticle) = obsticle_index { // Obsticle found, go to it
			row[obsticle-1].set_guard();
			self.direction.go_right();
			self.rotate_left();
			Ok((traversed, true))
		} else { // There is no obsticle; We've exited the map.
			Ok((traversed, false))
		}
	}

	/// Counts the number of tiles that have been traversed thus far
	fn count_traversed(&self) -> usize {
		self.map.iter().flatten().filter(|&&tile| tile.is_visited()).count()
	}

	/// Traverses until either an error occurs, or we can no longer traverse.
	fn traverse_steps(&mut self, max_iters: usize) -> Result<(), TraversalError> {
		let mut counter = 0;
		while self.traverse().map_err(TraversalError::TraversalStepError)?.1 {
			// Ensure we don't exceed max iterations
			counter += 1;
			if counter > max_iters { return Err(TraversalError::MaxIterationsReached); }
		}
		Ok(())
	}

}

/// Possible errors in the part 1 solution.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Part1Error {
	TraversalError(TraversalError),
	MapParsingError,
}

/// Part 1 solution to the advent of code day 6.
/// Puzzle: traverse until the end, and find the number of traversed tiles.
pub fn part1_solution(input: &str, max_iters: usize) -> Result<usize, Part1Error> {
	let mut map = Map::from_string(input).ok_or(Part1Error::MapParsingError)?;
	map.traverse_steps(max_iters).map_err(Part1Error::TraversalError)?;
	Ok(map.count_traversed())
}

/// Possible errors in the part 2 solution.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Part2Error {
	MapParsingError,
}

/// Part 2 solution to the advent of code day 6.
/// Puzzle: Count the number of places we could add an obsticle to force the guard into an infinite loop.
pub fn part2_solution(input: &str, max_iters: usize) -> Result<usize, Part2Error> {
	let map = Map::from_string(input).ok_or(Part2Error::MapParsingError)?;
	let indices: Vec<(usize, usize)> = (0..map.map.len()).flat_map(|y| (0..map.map[0].len()).map(move |x| (y, x))).collect();
	
	Ok(indices.par_iter().filter(|(y, x)| {
		// Exclude anything which already had a barrier
		if map.map[*y][*x] == Tile::Obsticle { return false; }

		// Clone the map and add the obsticle, see if it is infinite.
		let mut map = map.clone();
		map.map[*y][*x] = Tile::Obsticle;
		let response = map.traverse_steps(max_iters);
		if let Err(err) = response {
			match err {
				TraversalError::TraversalStepError(traversal_step_error) => {
					traversal_step_error == TraversalStepError::InfiniteLoopEncountered
				},
				TraversalError::MaxIterationsReached => {
					println!("Max iterations reached.");
					false
				},
			}
		} else { false }
	}).count())
}

pub fn main() {
	let example = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
	let input = include_str!("day6.txt");

	println!("Part 1 solution for Example {:#?}", part1_solution(example, 20));
	println!("Part 1 solution for Input {:#?}", part1_solution(input, 10000));

	println!("Part 2 solution for Example {:#?}", part2_solution(example, 50));
	println!("Part 2 solution for Input {:#?}", part2_solution(input, 10000));
}
