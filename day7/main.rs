use std::{borrow::Borrow, fmt::{self, Display, Formatter}, ops::{Add, Mul}};

use itertools::Itertools;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

/// Operands used for evaluating equations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operand {
	Add, Mul,
}

impl Operand {
	/// Evaluates the operator on two items.
	fn evaluate<T: Add<Output = T> + Mul<Output = T>>(&self, a: T, b: T) -> T {
		match self {
			Operand::Add => a + b,
			Operand::Mul => a * b,
		}
	}
}

/// Represents a single equation from day 7 of advent of code.
#[derive(Debug)]
struct Equation {
	target: usize,
	values: Vec<usize>,
}

impl Display for Equation {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "{}:", self.target)?;
		for val in &self.values { write!(f, " {val}")?; }
		Ok(())
	}
}

impl Equation {
	/// Creates an equation from a string, returns None on failure.
	fn from_string(value: &String) -> Option<Self> {
		let strs = value.split(": ").map(String::from).collect::<Vec<String>>();
		let (target_str, values_str) = if let [target_str, values_str] = strs.as_slice() {
			(target_str, values_str)
		} else { return None };

		let values = values_str.split(' ').map(|val| { val.parse::<usize>().ok() }).collect::<Option<Vec<usize>>>()?;
		if values.is_empty() { return None }

		Some(Self { target: target_str.parse().ok()?, values })
	}

	/// Evaluates the equation by using some operands, will return None if the operands are of incorrect length.
	fn evaluate<Op: Borrow<Operand>, It: IntoIterator<Item = Op>>(&self, operands: It) -> Option<usize> {
		let ops = operands.into_iter().collect_vec();
		if ops.len() != self.values.len() - 1 { return None; }
		Some(self.values[1..].iter()
			.zip(ops.iter())
			.fold(self.values[0], |a, (&b, op)| op.borrow().evaluate(a, b)))
	}

	/// Whether or not the target is achievable by some left to right permutation of the operands + and * on the values.
	/// Returns true when the target is achievable. Returns None if there was an error encountered.
	fn target_achievable(&self) -> Option<bool> {
		let operators = [Operand::Add, Operand::Mul];
		let results = (0..self.values.len() - 1)
			.map(|_| operators.iter())
			.multi_cartesian_product()
			.map(|operands| self.evaluate(operands))
			.collect::<Option<Vec<usize>>>()?;
		Some(results.iter().any(|&result| result == self.target))
	}
}

/// Parses an input string into a list of equations, or provides the line number where parsing failed.
fn parse_input(input: &String) -> Result<Vec<Equation>, usize> {
	input.split('\n')
		.map(String::from)
        .enumerate()
        .map(|(line, eq)| Equation::from_string(&eq).ok_or(line))
        .collect()
}

/// Possible errors when attempting to solve part 1
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Part1Error {
	/// An error parsing the input
	ParseError { line: usize },
	/// An error evaluating an equation
	EvaluationError,
}

/// Solves part1 - returns the sum of all equation targets which are achievable left to right with
/// some permutation of the + and * operands.
pub fn part1_solution(input: &String) -> Result<usize, Part1Error> {
	let equations = parse_input(input).map_err(|line| Part1Error::ParseError { line })?;
	let achievable = equations.par_iter()
		.map(|eq| eq.target_achievable())
		.collect::<Option<Vec<bool>>>()
		.ok_or(Part1Error::EvaluationError)?;
	Ok(achievable.par_iter()
		.zip(equations)
		.filter_map(|(achievable, eq)| achievable.then_some(eq.target))
		.sum())
}


/// Entry point to the day 7 task.
pub fn main() {
	let example = String::from("190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20");
	let input = String::from(include_str!("day7.txt"));

	println!("Part 1 Solution on Example: {:#?}", part1_solution(&example));
	println!("Part 1 Solution on Input: {:#?}", part1_solution(&input));
}
