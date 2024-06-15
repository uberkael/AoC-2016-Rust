#[cfg(test)]
mod tests;

pub fn aoc02() {
	println!("\nDay 2: Bathroom Security");
	println!("━━━━━━━━━━━━━━━━━━━━━━━━");

	let input = std::fs::read_to_string("input/02/input.txt").unwrap();

	/* Part 1 */
	println!("Part 1:\n{}", part1(&input));

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
