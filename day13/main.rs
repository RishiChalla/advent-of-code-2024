use std::num::ParseIntError;
use regex::Regex;

/// Represents a direction vector
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Vector2 { x: i64, y: i64 }

impl Vector2 {
	/// Creates a new Vector2
	fn new(x: i64, y: i64) -> Self {
		Self { x, y }
	}
}

/// Describes a single slot machine - the change in position by button A, B, and the prize location.
#[derive(Debug)]
struct SlotMachine {
	button_a: Vector2,
	button_b: Vector2,
	prize: Vector2,
}

impl SlotMachine {
	/// Calculates the button presses needed on button A, and B to achieve the prize
	fn calculate_presses(&self) -> Option<(usize, usize)> {
		// System of linear equations:
		// self.button_a.x * a + self.button_b.x * b = self.prize.x
		// self.button_a.y * a + self.button_b.y * b = self.prize.y

		if self.button_b.x == 0 { return None; }

		let (ax, ay, bx, by, px, py) = (
			self.button_a.x as f64, self.button_a.y as f64,
			self.button_b.x as f64, self.button_b.y as f64,
			self.prize.x as f64, self.prize.y as f64,
		);

		let a_denom = ay - by * ax / bx;
		let a = (py - by * px / bx) / a_denom;
		let b = (px - ax * a) / bx;

		let (a, b) = (a.round() as i64, b.round() as i64);
		if self.button_a.x * a + self.button_b.x * b != self.prize.x ||
			self.button_a.y * a + self.button_b.y * b != self.prize.y { return None }
		Some((usize::try_from(a).ok()?, usize::try_from(b).ok()?))
	}
}

/// Possible errors when parsing a slot machine values
#[derive(Debug)]
enum SlotMachineParseError {
	#[allow(dead_code)]
	RegexParseError(regex::Error),
	#[allow(dead_code)]
	IntegerParseError { value: String, error: ParseIntError },
	InvalidVectorCount,
}

impl TryFrom<&str> for SlotMachine {
    type Error = SlotMachineParseError;

	/// Converts from a string in format:
	/// ```txt
	/// Button A: X+94, Y+34
	/// Button B: X+22, Y+67
	/// Prize: X=8400, Y=5400
	/// ```
    fn try_from(value: &str) -> Result<Self, SlotMachineParseError> {
		let regex = Regex::new("X=?([+-]?[0-9]+), Y=?([+-]?[0-9]+)").map_err(SlotMachineParseError::RegexParseError)?;
		let vectors = regex.captures_iter(value).map(|capture| -> Result<Vector2, SlotMachineParseError> {
			let (_, [x, y]) = capture.extract();
			let (x, y) = (
				x.parse::<i64>().map_err(|error| SlotMachineParseError::IntegerParseError { value: String::from(x), error })?,
				y.parse::<i64>().map_err(|error| SlotMachineParseError::IntegerParseError { value: String::from(y), error })?,
			);
			Ok(Vector2::new(x, y))
		}).collect::<Result<Vec<_>, _>>()?;
		let [button_a, button_b, prize] = vectors.as_slice() else { return Err(SlotMachineParseError::InvalidVectorCount) };
		Ok(Self { button_a: *button_a, button_b: *button_b, prize: *prize })
    }
}

/// Parses a list of slot machines
fn parse_slot_machines(input: &str) -> Result<Vec<SlotMachine>, SlotMachineParseError> {
	input.split("\n\n").map(SlotMachine::try_from).collect()
}

/// Calculates the tokens needed to win all given slot machines
fn part1_solution(input: &str) -> Result<usize, SlotMachineParseError> {
	let machines = parse_slot_machines(input)?;
	Ok(machines.iter()
		.flat_map(|machine| machine.calculate_presses())
		.filter_map(|(a, b)| (a <= 100 && b <= 100).then_some(a * 3 + b))
		.sum())
}

/// Calculates the tokens needed to win all given slot machines when the prize location is +10000000000000
fn part2_solution(input: &str) -> Result<usize, SlotMachineParseError> {
	let mut machines = parse_slot_machines(input)?;
	for machine in &mut machines { machine.prize.x += 10000000000000i64; machine.prize.y += 10000000000000i64; }
	Ok(machines.iter()
		.flat_map(|machine| machine.calculate_presses())
		.map(|(a, b)| a * 3 + b)
		.sum())
}

/// Entry point
fn main() {
	let example = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
	let input = include_str!("day13.txt");

	println!("Part 1 Solution on Example: {:#?}", part1_solution(example));
	println!("Part 1 Solution on Input: {:#?}", part1_solution(input));

	println!("Part 2 Solution on Example: {:#?}", part2_solution(example));
	println!("Part 2 Solution on Input: {:#?}", part2_solution(input));
}
