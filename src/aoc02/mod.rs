#[cfg(test)]
mod tests;

pub fn aoc02() {
	println!("\nDay 2: Bathroom Security");
	println!("━━━━━━━━━━━━━━━━━━━━━━━━");

	let input = std::fs::read_to_string("input/02/input.txt")
		.expect("Error reading file");

	/* Part 1 */
	println!("Part 1:\n{}", part1(&input));

	/* Part 2 */
	println!("Part 2:\n{}", part2(&input));

}

fn part1(input: &str) -> String {
	let mut keypad = Keypad::new();
	let mut result: String = String::new();

	for line in input.lines() {
		let instructions = Instructions::new(line);
		keypad.follow(instructions);
		result = result + &keypad.value().to_string();
	}
	result
}

struct Keypad {
	x: usize,
	y: usize,
	keys: [[u32; 3]; 3],
}

impl Keypad {
	fn new() -> Keypad {
		Keypad {
			/* Start at 5 */
			x: 1,
			y: 1,
			keys: [[7, 8, 9],
						 [4, 5, 6],
						 [1, 2, 3]],
		}
	}
	fn value(&self) -> u32 {
		self.keys[self.y][self.x]
	}
	fn follow(&mut self, instructions: Instructions) {
		for instruction in instructions.0 {
			self.move_instruction(&instruction);
		}
	}
	fn move_instruction(&mut self, instruction: &char) {
		match instruction {
			'U' => { if self.y < 2 { self.y += 1 } }
			'D' => { if self.y > 0 { self.y -= 1 } }
			'R' => { if self.x < 2 { self.x += 1 } }
			'L' => { if self.x > 0 { self.x -= 1 } }
			_ => panic!("Invalid instruction"),
		}
	}
}

struct Instructions(Vec<char>);

impl Instructions {
	fn new(instructions_str: &str) -> Self {
		Self(instructions_str.chars().collect())
	}
}

/* Part 2 */

/*
    1
  2 3 4
5 6 7 8 9
  A B C
    D
*/
struct Keypad2 {
	x: usize,
	y: usize,
	keys: [[char; 7]; 7],
}

impl Keypad2 {
	fn new() -> Keypad2 {
		Keypad2 {
			/* Start at 5 */
			x: 1,
			y: 3,
			/* Zero padding */
			keys: [
				['0', '0', '0', '0', '0', '0', '0'],
				['0', '0', '0', '1', '0', '0', '0'],
				['0', '0', '2', '3', '4', '0', '0'],
				['0', '5', '6', '7', '8', '9', '0'],
				['0', '0', 'A', 'B', 'C', '0', '0'],
				['0', '0', '0', 'D', '0', '0', '0'],
				['0', '0', '0', '0', '0', '0', '0'],
			],
		}
	}
	fn value(&self) -> char {
		self.keys[self.y][self.x]
	}
	fn follow(&mut self, instructions: Instructions) {
		for instruction in instructions.0 {
			self.move_instruction(&instruction);
		}
	}
	fn move_instruction(&mut self, instruction: &char) {
		match instruction {
			'U' => { if self.keys[self.y - 1][self.x] != '0' { self.y -= 1 } }
			'D' => { if self.keys[self.y + 1][self.x] != '0' { self.y += 1 } }
			'R' => { if self.keys[self.y][self.x + 1] != '0' { self.x += 1 } }
			'L' => { if self.keys[self.y][self.x - 1] != '0' { self.x -= 1 } }
			_ => panic!("Invalid instruction"),
		}
	}
}

fn part2(input: &str) -> String {
	let mut keypad = Keypad2::new();
	let mut result: String = String::new();

	for line in input.lines() {
		let instructions = Instructions::new(line);
		keypad.follow(instructions);
		result = result + &keypad.value().to_string();
	}
	result
}
