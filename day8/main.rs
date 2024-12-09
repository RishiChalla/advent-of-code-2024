use std::{cmp, collections::HashMap, fmt::{self, Display, Formatter, Write}, ops::Range};

use itertools::Itertools;
use nalgebra::Vector2;

/// Describes a bounding box on the map
#[derive(Debug, Clone)]
struct BoundingBox {
	/// Everything within the bounding box must be >= top_left
	top_left: Vector2<i32>,
	/// Everything within the bounding box must be <= bottom_right
	bottom_right: Vector2<i32>,
}

impl BoundingBox {
	/// Returns whether or not the bounding box includes the given position (true when pos is in the bounds).
	fn includes(&self, pos: Vector2<i32>) -> bool {
		pos >= self.top_left && pos <= self.bottom_right
	}
}

/// Represents the variant of a single antenna on the map
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum AntennaVariant {
	// Lowercase variants
	VariantLowerA, VariantLowerB, VariantLowerC, VariantLowerD, VariantLowerE, VariantLowerF, VariantLowerG, VariantLowerH, VariantLowerI, VariantLowerJ,
	VariantLowerK, VariantLowerL, VariantLowerM, VariantLowerN, VariantLowerO, VariantLowerP, VariantLowerQ, VariantLowerR, VariantLowerS, VariantLowerT,
	VariantLowerU, VariantLowerV, VariantLowerW, VariantLowerX, VariantLowerY, VariantLowerZ,
	// Uppercase variants
	VariantUpperA, VariantUpperB, VariantUpperC, VariantUpperD, VariantUpperE, VariantUpperF, VariantUpperG, VariantUpperH, VariantUpperI, VariantUpperJ,
	VariantUpperK, VariantUpperL, VariantUpperM, VariantUpperN, VariantUpperO, VariantUpperP, VariantUpperQ, VariantUpperR, VariantUpperS, VariantUpperT,
	VariantUpperU, VariantUpperV, VariantUpperW, VariantUpperX, VariantUpperY, VariantUpperZ,
	// Digit variants
	Variant0, Variant1, Variant2, Variant3, Variant4, Variant5, Variant6, Variant7, Variant8, Variant9,
}

impl AntennaVariant {
	/// Gets a map from a char to the variants.
	fn get_char_map() -> HashMap<char, AntennaVariant> {
		HashMap::from([
			// Lowercase variants
			('a', AntennaVariant::VariantLowerA), ('b', AntennaVariant::VariantLowerB), ('c', AntennaVariant::VariantLowerC), ('d', AntennaVariant::VariantLowerD),
			('e', AntennaVariant::VariantLowerE), ('f', AntennaVariant::VariantLowerF), ('g', AntennaVariant::VariantLowerG), ('h', AntennaVariant::VariantLowerH),
			('i', AntennaVariant::VariantLowerI), ('j', AntennaVariant::VariantLowerJ), ('k', AntennaVariant::VariantLowerK), ('l', AntennaVariant::VariantLowerL),
			('m', AntennaVariant::VariantLowerM), ('n', AntennaVariant::VariantLowerN), ('o', AntennaVariant::VariantLowerO), ('p', AntennaVariant::VariantLowerP),
			('q', AntennaVariant::VariantLowerQ), ('r', AntennaVariant::VariantLowerR), ('s', AntennaVariant::VariantLowerS), ('t', AntennaVariant::VariantLowerT),
			('u', AntennaVariant::VariantLowerU), ('v', AntennaVariant::VariantLowerV), ('w', AntennaVariant::VariantLowerW), ('x', AntennaVariant::VariantLowerX),
			('y', AntennaVariant::VariantLowerY), ('z', AntennaVariant::VariantLowerZ),
			// Uppercase variants
			('A', AntennaVariant::VariantUpperA), ('B', AntennaVariant::VariantUpperB), ('C', AntennaVariant::VariantUpperC), ('D', AntennaVariant::VariantUpperD),
			('E', AntennaVariant::VariantUpperE), ('F', AntennaVariant::VariantUpperF), ('G', AntennaVariant::VariantUpperG), ('H', AntennaVariant::VariantUpperH),
			('I', AntennaVariant::VariantUpperI), ('J', AntennaVariant::VariantUpperJ), ('K', AntennaVariant::VariantUpperK), ('L', AntennaVariant::VariantUpperL),
			('M', AntennaVariant::VariantUpperM), ('N', AntennaVariant::VariantUpperN), ('O', AntennaVariant::VariantUpperO), ('P', AntennaVariant::VariantUpperP),
			('Q', AntennaVariant::VariantUpperQ), ('R', AntennaVariant::VariantUpperR), ('S', AntennaVariant::VariantUpperS), ('T', AntennaVariant::VariantUpperT),
			('U', AntennaVariant::VariantUpperU), ('V', AntennaVariant::VariantUpperV), ('W', AntennaVariant::VariantUpperW), ('X', AntennaVariant::VariantUpperX),
			('Y', AntennaVariant::VariantUpperY), ('Z', AntennaVariant::VariantUpperZ),
			// Digit variants
			('0', AntennaVariant::Variant0), ('1', AntennaVariant::Variant1), ('2', AntennaVariant::Variant2), ('3', AntennaVariant::Variant3),
			('4', AntennaVariant::Variant4), ('5', AntennaVariant::Variant5), ('6', AntennaVariant::Variant6), ('7', AntennaVariant::Variant7),
			('8', AntennaVariant::Variant8), ('9', AntennaVariant::Variant9),
		])
	}
}

impl TryFrom<char> for AntennaVariant {
	type Error = ();

	fn try_from(value: char) -> Result<Self, ()> {
		Self::get_char_map().get(&value).cloned().ok_or(())
	}
}

impl From<AntennaVariant> for char {
	fn from(value: AntennaVariant) -> Self {
		let char_map: HashMap<_, _> = AntennaVariant::get_char_map().iter().map(|(c, variant)| (*variant, *c)).collect();
		if let Some(c) = char_map.get(&value) { *c } else { '.' }
	}
}

impl Display for AntennaVariant {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_char((*self).into())
	}
}

/// Represents the full map from the puzzle, containing the bounds of the city and all antenna locations.
#[derive(Debug)]
struct Map {
	/// The bounds of the map
	bounds: BoundingBox,
	/// All antennas on the map.
	antennas: HashMap<AntennaVariant, Vec<Vector2<i32>>>,
}

impl From<&str> for Map {
	fn from(value: &str) -> Self {
		let lines = value.lines().collect_vec();
		let mut antennas = HashMap::new();
		let positions = lines.iter().enumerate().flat_map(|(y, line)| {
			line.chars().enumerate().filter_map(move |(x, c)| {
				Some((AntennaVariant::try_from(c).ok()?, Vector2::new(x as i32, y as i32)))
			})
		});
		for (variant, pos) in positions { antennas.entry(variant).or_insert(Vec::new()).push(pos) }
		Map {
			bounds: BoundingBox {
				top_left: Vector2::new(0, 0),
				bottom_right: Vector2::new(lines[0].len() as i32 - 1, lines.len() as i32 - 1)
			},
			antennas,
		}
	}
}

impl From<&Map> for String {
	fn from(map: &Map) -> Self {
		map.to_string(None)
	}
}

impl Display for Map {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(String::from(self).as_str())
	}
}

impl Map {

	/// Converts the map to a displayable string
	fn to_string(&self, antinodes: Option<&HashMap<AntennaVariant, Vec<Vector2<i32>>>>) -> String {
		let mut lines: Vec<Vec<char>> = vec![vec!['.'; self.bounds.bottom_right.x as usize + 1]; self.bounds.bottom_right.y as usize + 1];
		for (variant, positions) in &self.antennas {
			for pos in positions { lines[pos.y as usize][pos.x as usize] = (*variant).into(); }	
		}
		if let Some(antinodes) = antinodes {
			for pos in antinodes.values().flatten() { lines[pos.y as usize][pos.x as usize] = '#'; }
		}
		lines.iter()
			.map(|line| line.iter().collect::<String>())
			.collect::<Vec<String>>()
			.join("\n")
	}

	/// Gets all antinodes created by the antennas in the map. For each line from two antennas of the same frequency,
	/// Each item in the range rep will be given its own antinode.
	fn get_antinodes(&self, reps: Option<Range<usize>>) -> HashMap<AntennaVariant, Vec<Vector2<i32>>> {
		let reps = if let Some(reps) = reps { reps } else {
			0..cmp::max(self.bounds.bottom_right.x as usize, self.bounds.bottom_right.y as usize)
		};
		self.antennas.iter().map(|(variant, positions)| {
			let antinodes = positions.iter().permutations(2).flat_map(|antennas| {
				let (&&from, &&to) = antennas.iter().collect_tuple().expect("Expected permutations of 2 antennas");
				let step = to - from;
				reps.clone().filter_map(move |idx| {
					let antinode = to + step * idx as i32;
					self.bounds.includes(antinode).then_some(antinode)
				})
			}).collect_vec();
			(*variant, antinodes)
		}).collect()
	}
}

/// Finds the number of unique positions antinodes are present in when only 1 antinode is created per pair of antennas.
pub fn part1_solution(input: &str) -> usize {
	Map::from(input)
		.get_antinodes(Some(1..2))
		.drain()
		.flat_map(|(_variant, positions)| positions)
		.unique()
		.count()
}

/// Finds the number of unique positions antinodes are present in when any amount of antinodes are created per pair of antennas.
pub fn part2_solution(input: &str) -> usize {
	Map::from(input)
		.get_antinodes(None)
		.drain()
		.flat_map(|(_variant, positions)| positions)
		.unique()
		.count()
}

/// Entry point
pub fn main() {
	let example = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
	let input = include_str!("day8.txt");

	println!("Part 1 Solution on Example: {:#?}", part1_solution(example));
	println!("Part 1 Solution on Input: {:#?}", part1_solution(input));

	println!("Part 2 Solution on Example: {:#?}", part2_solution(example));
	println!("Part 2 Solution on Input: {:#?}", part2_solution(input));
}
