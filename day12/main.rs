use std::collections::{HashMap, HashSet, VecDeque};

/// A single position on the garden
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Position { x: usize, y: usize }

impl Position {
	/// Gets neighboring positions to this one. Returns None if the position would've had a negative.
	fn get_neighbors(&self) -> [Option<Position>; 4] {
		[
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
struct Region { plant: Plant, plots: HashSet<Position> }

impl Region {
	/// Calculates the perimeter of this region
	fn calculate_perimeter(&self) -> usize {
		self.plots.iter().map(|pos| {
			pos.get_neighbors().into_iter().filter(|neighbor| {
				let Some(neighbor) = neighbor else { return true };
				!self.plots.contains(neighbor)
			}).count()
		}).sum()
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
			let mut region = Region { plant: region_plant, plots: HashSet::new() };

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

	// println!("Part 2 Solution on Example: {:#?}", part2_solution(example));
	// println!("Part 2 Solution on Input: {:#?}", part2_solution(input));

}
