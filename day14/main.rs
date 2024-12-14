use std::num::ParseIntError;


/// Represents a 2d direction vector
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Vec2 { x: i32, y: i32 }

/// A bounding box containing a section of space
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Bounds { left: i32, top: i32, right: i32, bottom: i32 }

impl Bounds {
	/// The width of the bounding box
	fn width(&self) -> i32 { self.right - self.left }

	/// The height of the bounding box
	fn height(&self) -> i32 { self.top - self.bottom }

	/// Gets 4 quadrants within the current bounds. If the bounds are uneven, the middle axes are removed.
	fn get_quadrants(&self) -> [Bounds; 4] {
		let Self { left, top, right, bottom } = *self;
		let (width, height) = (self.width(), self.height());
		let (m_right, m_bottom, m_left, m_top) = (left + width / 2, bottom + height / 2 - 1, right - width / 2, top - height / 2 + 1);
		[
			Bounds { left, top, right: m_right, bottom: m_bottom }, // Top-left
			Bounds { left: m_left, top, right, bottom: m_bottom }, // Top-right
			Bounds { left, top: m_top, right: m_right, bottom }, // Bottom-left
			Bounds { left: m_left, top: m_top, right, bottom }, // Bottom-right
		]
	}

	/// Checks whether or not this bounding box contains a certain position.
	/// left/top are inclusive, right/bottom are exclusive.
	fn contains(&self, pos: Vec2) -> bool {
		pos.x >= self.left && pos.x < self.right && pos.y >= self.top && pos.y < self.bottom
	}
}

/// A single robot, its position, and its movement velocity
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Robot {
	position: Vec2,
	velocity: Vec2,
}

impl Robot {
	/// Constrains (**not clamps**) the position within a min/max by "looping" the value from an edge to the opposite side
	/// to fit within the bounds.
	fn constrain(mut pos: i32, min: i32, max: i32) -> i32 {
		let width = max - min;
		if pos < 0 { pos += width * (-pos / width + 1) }
		min + (pos % width)
	}

	/// Simulates a number of steps on the robot and updates the robot accordingly
	fn step_n(&mut self, bounds: Bounds, steps: usize) {
		self.position.x = Self::constrain(self.position.x + self.velocity.x * steps as i32, bounds.left, bounds.right);
		self.position.y = Self::constrain(self.position.y + self.velocity.y * steps as i32, bounds.top, bounds.bottom);
	}
}

/// Possible errors when parsing the map
#[derive(Debug)]
#[allow(dead_code)]
enum MapParseError {
	InvalidPosition { string: String },
	IntegerParseError { error: ParseIntError, string: String },
	InvalidVectors { string: String },
}

/// A full map where robots are simulated on
#[derive(Debug, Clone)]
struct Map {
	robots: Vec<Robot>,
	bounds: Bounds,
}

impl Map {
	/// Parses a map from a string, and given the bounds.
	fn parse(input: &str, bounds: Bounds) -> Result<Self, (usize, MapParseError)> {
		// Loop through all lines - each line is a robot
		let robots = input.lines().enumerate().map(|(line_num, line)| {

			// Loop through each vector - each line / robot has a position and a velocity
			let vecs = line.replace("p=", "").replace("v=", "").split(" ").map(|pos_str| {

				// Loop through each numeric value in the vector and parse it
				let values = pos_str.split(",").map(|num_str| {
					num_str.parse::<i32>()
						.map_err(|error| MapParseError::IntegerParseError { error, string: num_str.into() })
				}).collect::<Result<Vec<_>, _>>()?;

				// Ensure there are only 2 numeric values per vector
				let [x, y] = *values.as_slice() else {
					return Err(MapParseError::InvalidPosition { string: pos_str.into() })
				};

				Ok(Vec2 { x, y })

			}).collect::<Result<Vec<_>, _>>().map_err(|err| (line_num, err))?; // Report errors with line the number
			
			// Each robot should only have 2 vectors
			let [position, velocity] = *vecs.as_slice() else {
				return Err((line_num, MapParseError::InvalidVectors { string: line.into() }))
			};

			Ok(Robot { position, velocity })

		}).collect::<Result<Vec<_>, _>>()?;

		Ok(Self { robots, bounds })
	}

	/// Simulates n steps on the map, all robots will be moved by n steps.
	fn step_n(&mut self, steps: usize) {
		for robot in &mut self.robots { robot.step_n(self.bounds, steps); }
	}

	/// Gets all robots in the map, divided into their individual quadrants
	fn get_robots_by_quadrants(&self) -> [Vec<Robot>; 4] {
		self.bounds.get_quadrants().map(|quad| {
			self.robots.iter().cloned().filter(|robot| quad.contains(robot.position)).collect()
		})
	}
}

/// Part 1 solution - product of the number of robots in each quadrant after 100 steps.
fn part1_solution(input: &str, bounds: Bounds) -> Result<usize, (usize, MapParseError)> {
	let mut map = Map::parse(input, bounds)?;
	map.step_n(100);
	Ok(map.get_robots_by_quadrants().iter().map(|quad| quad.len()).product())
}

/// Entry point
fn main() {
	let example_robots = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
	let example_bounds = Bounds { left: 0, top: 0, right: 11, bottom: 7 };
	let input_robots = include_str!("day14.txt");
	let input_bounds = Bounds { left: 0, top: 0, right: 101, bottom: 103 };

	println!("Part 1 Solution on Example: {:#?}", part1_solution(example_robots, example_bounds));
	println!("Part 1 Solution on Input: {:#?}", part1_solution(input_robots, input_bounds));
}
