#[cfg(test)]
mod tests;

pub fn aoc01() {
	println!("\nDay 1: No Time for a Taxicab");
	println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
	let input = std::fs::read_to_string("input/01/input.txt")
		.expect("Error reading file");

	/* Part 1 */
	let mut mr_taxi = MrTaxi::new();
	let instructions = Instructions::new(&input);
	mr_taxi.moves(&instructions);
	println!("Part 1:\n{}", mr_taxi.distance());

	/* Part 2 */
	let mut mr_taxi = MrTaxi::new();
	mr_taxi.moves2(&instructions);
	println!("Part 2:\n{}", mr_taxi.distance());

}

#[derive(Clone)]
struct Instructions {
	instructions: Vec<Instruction>,
}

impl Instructions {
	fn new(instructions_str: &str) -> Self {
		let instructions: Vec<&str> = instructions_str.split(", ").collect();
		let instructions = instructions.iter().map( |&x| Instruction::new(x)).collect();
		Self { instructions }
	}
}

#[derive(Clone)]
struct Instruction {
	turn: char,
	distance: i32,
}

impl Instruction {
	fn new(str: &str) -> Self {
		let turn = str.chars().nth(0).expect("Invalid turn");
		let distance = str[1..].trim().parse().expect("Invalid distance");
		Self { turn, distance }
	}
}

enum Direction {
	Up,
	Down,
	Right,
	Left,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
	x: i32,
	y: i32,
}

impl Point {
	fn move_n(self, direction: &Direction, n: i32) -> Self {
		match direction {
			Direction::Up    => Point { x: self.x, y: self.y + n },
			Direction::Down  => Point { x: self.x, y: self.y - n },
			Direction::Right => Point { x: self.x + n, y: self.y },
			Direction::Left  => Point { x: self.x - n, y: self.y },
		}
	}
}

struct MrTaxi {
	point: Point,
	direction: Direction,
	visited: Vec<Point>,
}

impl MrTaxi {
	fn new() -> Self {
		Self {
			point: Point { x: 0, y: 0 },
			direction: Direction::Up,
			visited: Vec::new(),
		}
	}

	fn change_direction(&mut self, new: &char) {
		self.direction = match (&self.direction, new) {
			(Direction::Up, 'R')   | (Direction::Down, 'L')  => Direction::Right,
			(Direction::Up, 'L')   | (Direction::Down, 'R')  => Direction::Left,
			(Direction::Left, 'L') | (Direction::Right, 'R') => Direction::Down,
			(Direction::Left, 'R') | (Direction::Right, 'L') => Direction::Up,
			(_, _) => { panic!("Invalid turn") }
		}
	}

	fn moves(&mut self, instructions: &Instructions) {
		for instruction in &instructions.instructions {
			self.change_direction(&instruction.turn);
			self.point = self.point.move_n(&self.direction, instruction.distance);
		}
	}

	fn moves2(&mut self, instructions: &Instructions) {
		for instruction in &instructions.instructions {
			self.change_direction(&instruction.turn);
			for _ in 0..instruction.distance {
				self.point = self.point.move_n(&self.direction, 1);
				if self.visited.contains(&self.point) {
					return;
				}
				self.visited.push(self.point);
			}
		}
	}

	fn distance(&self) -> i32 {
		self.point.x.abs() + self.point.y.abs()
	}
}
