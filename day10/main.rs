use std::{collections::HashMap, fmt::Display};

use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

/// Represents a position on the map, indexed by `map[x][y]`
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
struct Position { x: usize, y: usize }

impl Position {
    /// Creates a new position
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

/// Height map
struct Map {
    topology: Vec<Vec<u8>>,
}

impl Map {
    /// Gets all trailheads in the map from their origin.
    fn get_trailheads(&self) -> HashMap<Position, Vec<[Position; 10]>> {
        let positions = self.topology.iter().enumerate()
            .flat_map(|(x, line)| (0..line.len()).map(move |y| Position::new(x, y)))
            .collect::<Vec<_>>();
        positions.into_par_iter().filter(|&item| self.at(item) == 0).filter_map(|origin| {
            let trails = self.get_trailheads_from_origin(origin);
            (!trails.is_empty()).then_some((origin, trails))
        }).collect()
    }

    /// Height at position
    fn at(&self, pos: Position) -> u8 {
        self.topology[pos.x][pos.y]
    }

    /// Checks if map contains a position
    fn contains(&self, x: i32, y: i32) -> bool {
        if x < 0 || y < 0 { return false }
        let Some(line) = self.topology.get(x as usize) else { return false };
        (y as usize) < line.len()
    }

    /// Gets neighboring positions to a position
    fn neighbors(&self, pos: Position) -> [Option<Position>; 4] {
        let (x, y) = (pos.x as i32, pos.y as i32);
        let neighbors = [(1, 0), (0, 1), (-1, 0), (0, -1)];
        neighbors.map(|(d_x, d_y)| {
            self.contains(x + d_x, y + d_y)
                .then(|| Position::new((x + d_x) as usize, (y + d_y) as usize)) // Use .then to avoid negative unsigned casting
        })
    }

    /// Continues an existing trail by adding its valid neighbors
    fn continue_trail(&self, digit: u8, trail: Vec<Position>) -> Vec<Vec<Position>> {
        self.neighbors(*trail.last().unwrap()).iter()
            .filter_map(|&pos| (self.at(pos?) == digit + 1).then_some({
                let mut trail = trail.clone();
                trail.push(pos?);
                trail
            }))
            .collect()
    }

    /// Gets all trails from a single origin
    fn get_trailheads_from_origin(&self, origin: Position) -> Vec<[Position; 10]> {
        let trails: Vec<Vec<Position>> = (1..9u8).fold(self.continue_trail(0, vec![origin]), |trails, digit| {
            trails.into_iter()
                .flat_map(|trail| self.continue_trail(digit, trail))
                .collect()
        });
        trails.iter().map(|trail| {
            let [_, t1, t2, t3, t4, t5, t6, t7, t8, t9] = trail.as_slice() else { panic!("Invalid number of trails returned") };
            [origin, *t1, *t2, *t3, *t4, *t5, *t6, *t7, *t8, *t9]
        }).collect()
    }

    /// Marks a trail on the map and returns it
    #[allow(dead_code)]
    fn mark_trail(&self, trail: &[Position; 10]) -> String {
        let lines = self.topology.iter().enumerate().map(|(m_x, line)| line.iter().enumerate().map(|(m_y, digit)| {
            if trail.iter().any(|pos| pos.x == m_x && pos.y == m_y) { String::from("+") }
            else { digit.to_string() }
        }).collect::<String>()).collect::<Vec<_>>();
        lines.join("\n")
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lines = self.topology.iter()
            .map(|line| line.iter().map(|x| x.to_string()).collect::<String>()).collect::<Vec<_>>();
        f.write_str(lines.join("\n").as_str())
    }
}

/// Reports the location a map failed to parse.
#[derive(Debug)]
struct MapParseError { line: usize, col: usize }

impl Display for MapParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Encountered Non-Digit Character while parsing Map at line {}, col {}", self.line, self.col)
    }
}

/// Parses a height map from a grid string
impl TryFrom<&str> for Map {
    type Error = MapParseError;

    fn try_from(input: &str) -> Result<Self, MapParseError> {
        let topology = input.lines().enumerate().map(|(line_num, line)| {
            line.chars()
                .enumerate()
                .map(|(col_num, c)| c.to_digit(10).map(|digit| digit as u8).ok_or(col_num))
                .collect::<Result<Vec<_>, _>>()
                .map_err(|col_num| MapParseError { line: line_num, col: col_num })
        }).collect::<Result<Vec<_>, _>>()?;
        Ok(Map { topology })
    }
}

/// The sum of scores of trail ends
fn part1_solution(input: &str) -> Result<usize, MapParseError> {
    Ok(Map::try_from(input)?.get_trailheads().values()
        .map(|trails| trails.iter().unique_by(|trail| trail[9]).count()).sum())
}

/// The sum of scores of trail heads
fn part2_solution(input: &str) -> Result<usize, MapParseError> {
    Ok(Map::try_from(input)?.get_trailheads().values()
        .map(|trails| trails.len()).sum())
}

/// Entry point
pub fn main() {
    let example = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
    let input = include_str!("day10.txt");

    println!("Part 1 Solution on Example: {:#?}", part1_solution(example));
	println!("Part 1 Solution on Input: {:#?}", part1_solution(input));

    println!("Part 2 Solution on Example: {:#?}", part2_solution(example));
	println!("Part 2 Solution on Input: {:#?}", part2_solution(input));
}
