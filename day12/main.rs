use std::collections::{HashMap, HashSet, VecDeque};

/// A single position on the garden
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Position { x: usize, y: usize }

impl Position {
	/// Gets neighboring positions to this one in clockwise order. Returns None if the position would've had a negative.
	fn get_neighbors(&self) -> [Option<Position>; 4] {
		[
			// 0,0 is at top-left, x increases right, y increases down
			self.x.checked_sub(1).map(|x| Self { x, y: self.y }),
			self.y.checked_sub(1).map(|y| Self { x: self.x, y }),
			Some(Self { x: self.x + 1, y: self.y }),
			Some(Self { x: self.x, y: self.y + 1 }),
		]
	}
}

/// Describes a type of plant being grown in a garden.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Plant { variant: char }

/// A single region, a group of multiple plots growing the same plant with adjacent borders.
#[derive(Debug, Clone)]
struct Region { plots: HashSet<Position> }

impl Region {
	/// Gets the neighboring plots to a plot which are present in this region.
	fn get_neighbors(&self, plot: Position) -> [Option<Position>; 4] {
		plot.get_neighbors().map(|neighbor| {
			self.plots.contains(&neighbor?).then_some(neighbor?)
		})
	}

	/// Calculates the perimeter of this region
	fn calculate_perimeter(&self) -> usize {
		self.plots.iter().map(|&pos| {
			4 - self.get_neighbors(pos).into_iter().flatten().count()
		}).sum()
	}

	/// Calculates the number of unique sides on the perimeter of this region.
	/// This means adjacent walls of the perimeter facing in the same direction will be counted as 1.
	fn calculate_sides(&self) -> usize {
		// Set of plots and edges which are untested
		let mut unvisited: HashSet<(Position, usize)> = self.plots.iter().flat_map(|&plot| (0..4).map(move |x| (plot, x))).collect();
		let mut sides = 0;

		while let Some((mut pos, facing)) = unvisited.iter().cloned().next() {
			// Mark the plot/edge as visited, and check if it is an edge
			unvisited.remove(&(pos, facing));
			let mut neighbors = self.get_neighbors(pos);
			if neighbors[facing].is_some() { continue; }

			// It is an edge, first we need to go backward to the "left" corner, then loop forward to the "right" corner, and mark
			// everything in between as visited.
			for turn in [1, 3] {
				while let Some(next) = neighbors[(facing + turn) % 4] {
					pos = next;
					neighbors = self.get_neighbors(pos);
					unvisited.remove(&(pos, facing));
					if neighbors[facing].is_some() { break }
				}
			}

			sides += 1;
		}

		sides
	}
}

/// A map from plot positions to their plant type for all plots in the garden.
struct Garden { plots: HashMap<Position, Plant> }

impl From<&str> for Garden {
    fn from(value: &str) -> Self {
        Self {
			plots: value.lines().enumerate()
				.flat_map(|(y, line)| {
					line.chars().enumerate()
						.map(move |(x, variant)| (Position { x, y }, Plant { variant }))
				})
				.collect()
		}
    }
}

impl Garden {
	/// Calculates all regions of gardens growing the same crops and returns them.
	fn calculate_regions(&self) -> Vec<Region> {
		// Clone plots to drain into regions
		let mut plots = self.plots.clone();
		let mut regions = Vec::new();

		// Loop while plots is not empty
		while let Some((start_pos, region_plant)) = plots.iter().next().map(|(&p, &r)| (p, r)) {
			// Each region will have an exploring list, which will be continually updated with neighbors
			let mut exploring_list = VecDeque::from([start_pos]);
			let mut region = Region { plots: HashSet::new() };

			// Add exploring into region if it is the same plant, and expand the exploring list with the neighbors
			while let Some(exploring) = exploring_list.pop_back() {
				let Some(plant) = plots.get(&exploring) else { continue };
				if *plant != region_plant { continue; }
				exploring_list.extend(exploring.get_neighbors().into_iter().flatten());
				plots.remove(&exploring);
				region.plots.insert(exploring);
			}

			regions.push(region);
		}

		regions
	}
}

/// Calculates the sum of products of the perimeter and area of all regions.
fn part1_solution(input: &str) -> usize {
	Garden::from(input).calculate_regions()
		.iter()
		.map(|region| region.plots.len() * region.calculate_perimeter())
		.sum()
}

/// Calculates the sum of products of the sides and area of all regions.
fn part2_solution(input: &str) -> usize {
	Garden::from(input).calculate_regions()
		.iter()
		.map(|region| region.plots.len() * region.calculate_sides())
		.sum()
}

/// Entry point
pub fn main() {
	let example = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
	let input = include_str!("day12.txt");

	println!("Part 1 Solution on Example: {:#?}", part1_solution(example));
	println!("Part 1 Solution on Input: {:#?}", part1_solution(input));

	println!("Part 2 Solution on Example: {:#?}", part2_solution(example));
	println!("Part 2 Solution on Input: {:#?}", part2_solution(input));
}

#[cfg(test)]
mod tests {

	use super::*;

	/// Tests various use cases of calculating sides
	#[test]
	fn test_calculate_sides() {
		// Test single block
		let region = Region { plots: HashSet::from([ Position { x: 0, y: 0 } ]) };
		assert_eq!(region.calculate_sides(), 4);

		// Test trivial square (tests right turns & ending on right turn)
		let region = Region {
			plots: HashSet::from([
				Position { x: 2, y: 2 },
				Position { x: 3, y: 2 },
				Position { x: 3, y: 3 },
				Position { x: 2, y: 3 },
			])
		};
		assert_eq!(region.calculate_sides(), 4);

		// Test cross (+ shape) - (tests left turns, right turns, ending on left turn)
		let region = Region {
			plots: HashSet::from([
				Position { x: 1, y: 2 },
				Position { x: 2, y: 2 },
				Position { x: 3, y: 2 },
				Position { x: 2, y: 3 },
				Position { x: 2, y: 1 },
			])
		};
		assert_eq!(region.calculate_sides(), 12);

		// Test larger square (tests right turns & ending in middle of edge)
		let region = Region {
			plots: (0..5).flat_map(|x| (0..5).map(move |y| Position { x, y })).collect()
		};
		assert_eq!(region.calculate_sides(), 4);

		// Test hole - [] shape
		let region = Region {
			plots: HashSet::from([
				Position { x: 0, y: 0 },
				Position { x: 1, y: 0 },
				Position { x: 2, y: 0 },
				Position { x: 2, y: 1 },
				Position { x: 2, y: 2 },
				Position { x: 1, y: 2 },
				Position { x: 0, y: 2 },
				Position { x: 0, y: 1 },
			])
		};
		assert_eq!(region.calculate_sides(), 8);
	}

	/// Tests part 2 on trivial cases
	#[test]
	fn test_part2_trivial() {
		let garden = "AAAA\nBBCD\nBBCC\nEEEC";
		assert_eq!(part2_solution(garden), 80);

		let garden = "OOOOO\nOXOXO\nOOOOO\nOXOXO\nOOOOO";
		assert_eq!(part2_solution(garden), 436);

		let garden = "EEEEE\nEXXXX\nEEEEE\nEXXXX\nEEEEE";
		assert_eq!(part2_solution(garden), 236);

		let garden = "AAAAAA\nAAABBA\nAAABBA\nABBAAA\nABBAAA\nAAAAAA";
		assert_eq!(part2_solution(garden), 368);
	}

}
