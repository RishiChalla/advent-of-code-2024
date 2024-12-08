use std::{cmp::Ordering, collections::HashMap, fmt::{self, Display, Formatter, Write}};
use derive_more::{Sub, Add};

use itertools::Itertools;

/// Describes a single position
#[derive(Debug, Clone, Copy, Eq, PartialEq, Sub, Add, Hash)]
struct Position { x: i32, y: i32 }


impl Position {
	/// Creates a new position
	fn new(x: i32, y: i32) -> Self {
		Self { x, y }
	}
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.x < other.x && self.y < other.y, self.x > other.x && self.y > other.y) {
            (true, false) => Some(Ordering::Less),
            (false, true) => Some(Ordering::Greater),
            (false, false) => (self.x == other.x && self.y == other.y).then_some(Ordering::Equal),
            _ => None, // Should not occur
        }
    }
}

/// Describes a bounding box on the map
#[derive(Debug, Clone)]
struct BoundingBox {
	/// Everything within the bounding box must be >= top_left
	top_left: Position,
	/// Everything within the bounding box must be <= bottom_right
	bottom_right: Position,
}

impl BoundingBox {
	/// Returns whether or not the bounding box includes the given position (true when pos is in the bounds).
	fn includes(&self, pos: Position) -> bool {
		pos.x >= self.top_left.x && pos.y >= self.top_left.y && pos.x <= self.bottom_right.x && pos.y <= self.bottom_right.y
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
	antennas: HashMap<AntennaVariant, Vec<Position>>,
}

impl From<&str> for Map {
	fn from(value: &str) -> Self {
		let lines = value.lines().collect_vec();
		let mut antennas = HashMap::new();
		let positions = lines.iter().enumerate().flat_map(|(y, line)| {
			line.chars().enumerate().filter_map(move |(x, c)| {
				Some((AntennaVariant::try_from(c).ok()?, Position::new(x as i32, y as i32)))
			})
		});
		for (variant, pos) in positions { antennas.entry(variant).or_insert(Vec::new()).push(pos) }
		Map {
			bounds: BoundingBox {
				top_left: Position::new(0, 0),
				bottom_right: Position::new(lines[0].len() as i32 - 1, lines.len() as i32 - 1)
			},
			antennas,
		}
	}
}

impl From<&Map> for String {
	fn from(map: &Map) -> Self {
		let mut lines: Vec<Vec<char>> = vec![vec!['.'; map.bounds.bottom_right.x as usize + 1]; map.bounds.bottom_right.y as usize + 1];
		for (variant, positions) in &map.antennas {
			for pos in positions { lines[pos.y as usize][pos.x as usize] = (*variant).into(); }	
		}
		lines.iter()
			.map(|line| line.iter().collect::<String>())
			.collect::<Vec<String>>()
			.join("\n")
	}
}

impl Display for Map {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(String::from(self).as_str())
	}
}

impl Map {
	/// Gets all antinodes created by the antennas in the 
	fn get_antinodes(&self) -> HashMap<AntennaVariant, Vec<Position>> {
		self.antennas.iter().map(|(variant, positions)| {
			let antinodes = positions.iter().permutations(2).filter_map(|antennas| {
				let (&&from, &&to) = antennas.iter().collect_tuple().expect("Expected permutations of 2 antennas");
				let antinode = to + (to - from);
				self.bounds.includes(antinode).then_some(antinode)
			}).collect_vec();
			(*variant, antinodes)
		}).collect()
	}
}

/// Finds the number of unique positions antinodes are present in.
pub fn part1_solution(input: &str) -> usize {
	Map::from(input)
		.get_antinodes()
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
}
