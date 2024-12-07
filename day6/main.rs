use std::collections::HashMap;


/// Rotates a 2d array rightt
fn rotate_right<T: Clone>(input: Vec<Vec<T>>) -> Vec<Vec<T>> {
	(0..input[0].len())
		.map(|i| input.iter().rev().map(|row| row[i].clone()).collect())
		.collect()
}

/// Rotates a 2d array left
fn rotate_left<T: Clone>(input: Vec<Vec<T>>) -> Vec<Vec<T>> {
	(0..input[0].len())
		.rev()
		.map(|i| input.iter().map(|row| row[i].clone()).collect())
		.collect()
}

/// Converts a string into a 2d array of chars, each row separated by a new line
fn input_to_map(input: &String) -> Vec<Vec<char>> {
	input.lines().map(|line| line.chars().collect()).collect()
}

/// Possible errors in the part 1 solution.
#[derive(Debug)]
pub enum Part1Error {
	GuardNotFound,
	MaxIterationsReached,
}

/// Part 1 solution to the advent of code day 6.
pub fn part1_solution(input: &String, max_iters: usize) -> Result<usize, Part1Error> {
	let mut map = rotate_right(input_to_map(input));
	for _ in 0..max_iters {
		// Row the guard is in, and the x position of the guard.
		let (x, row) = map.iter_mut()
			.find_map(|row| Some((row.iter().position(|c| *c == '^')?, row)))
			.ok_or(Part1Error::GuardNotFound)?;

		// Check if there's an obsticle in the guard's path
		if let Some(obsticle) = row.iter().skip(x).position(|c| *c == '#') {
			// Obsticle found, go to it
			row.iter_mut().skip(x).take(obsticle).for_each(|item| *item = 'X');
			row[x+obsticle-1] = '^';
			map = rotate_left(map);
		} else {
			// There is no obsticle; change the path to be traversed.
			row.iter_mut().skip(x)
				.for_each(|item| if *item == '.' || *item == '^' { *item = 'X' });
			return Ok(map.iter().flatten().filter(|item| **item == 'X').count());
		}
	}

	Err(Part1Error::MaxIterationsReached)
}


/// Movement direction of the guard
#[derive(Eq, PartialEq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
enum Direction {
	North, East, South, West,
}

impl Direction {
	fn get_right_direction(&self) -> Self {
		match self {
			Direction::North => Direction::East,
			Direction::East => Direction::South,
			Direction::South => Direction::West,
			Direction::West => Direction::North,
		}
	}

	fn go_right(&mut self) {
		*self = self.get_right_direction();
	}
}

/// Possible errors in the part 2 solution.
#[derive(Debug)]
pub enum Part2Error {
	GuardNotFound,
	MaxIterationsReached,
}

/// Part 2 solution to the advent of code day 6.
pub fn part2_solution(input: &String, max_iters: usize) -> Result<usize, Part2Error> {
	// Map of all movements the guard has made. Key contains (x, Direction) -> Output y
	let mut movements: HashMap<(usize, Direction), Vec<usize>> = HashMap::new();
	let mut map = rotate_right(input_to_map(input));
	let mut direction = Direction::North;
	let mut barriers = 0;
	for _ in 0..max_iters {
		// This code prints the current map.
		println!("{}", map.iter().map(|x| x.iter().collect()).collect::<Vec<String>>().join("\n"));

		// Row the guard is in, and the x position of the guard.
		let (x, y, row) = map.iter()
			.enumerate()
			.find_map(|(y, row)| Some((row.iter().position(|c| *c == '^')?, y, row)))
			.ok_or(Part2Error::GuardNotFound)?;
		movements.entry((y, direction)).or_insert(Vec::new()).push(x);
		println!("(y={y}, x={x}) {:#?}", direction);

		// Check if there's an obsticle in the guard's path
		let obsticle = row.iter().enumerate().skip(x).position(|(x, c)| {
			// As we go through each possible item, we check if we've previously gone in the right direction here -
			// If so, adding a barrier here would result in an infinite loop.
			if let Some(movements) = movements.get(&(x, direction.get_right_direction())) {
				// If any of the previous movements are not obstructed by an obsticle, then adding a barrier here
				// would result in an infinite loop.
				if movements.iter().any(|&mov_y| {
					true
					// dbg!(mov_y);
					// if mov_y >= y { // Earlier movement is beneath us or at us
					// 	let is_true = map.iter().take(map.len() - mov_y).skip(y).all(|row| row[x] != '#');
					// 	dbg!(is_true);
					// 	is_true
					// } else { false } // Earlier movement is above us, we can't move in that direction.
				}) { barriers += 1 }
			}
			// Whether or not this item is an obsticle
			*c == '#'
		});
		if let Some(obsticle) = obsticle {
			// Obsticle found, go to it
			map[y][x] = '.';
			map[y][x+obsticle-1] = '^';
			direction.go_right();
			map = rotate_left(map);
		} else {
			// There is no obsticle; We've exited the map.
			break;
		}
	}

	Ok(barriers)
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

	println!("Part 2 solution for Example {:#?}", part2_solution(&example, 20));
	// println!("Part 2 solution for Input {:#?}", part2_solution(&input, 10000));
}
