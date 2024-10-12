#![allow(dead_code)]

#[cfg(test)]
mod tests;

pub fn aoc21() {
	println!("\nDay 21: Scrambled Letters and Hash");
	println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

	let input = std::fs::read_to_string("input/21/input.txt").unwrap();

	println!("Part 1:\n{}", part1(&input));
	println!("Part 2:\n{}", part2(&input));
}

#[derive(Debug, PartialEq)]
enum Operation {
	SwapPosition(usize, usize),
	SwapLetter(char, char),
	RotateLeft(usize),
	RotateRight(usize),
	RotateOnLetter(char),
	Reverse(usize, usize),
	Move(usize, usize),
}

impl Operation {
	fn parse(input: &str) -> Option<Self> {
		let parts: Vec<&str> = input.split_whitespace().collect();
		match (parts[0], parts[1]) {
			("swap", "position") => {
				let x = parts[2].parse::<usize>().ok()?;
				let y = parts[5].parse::<usize>().ok()?;
				Some(Operation::SwapPosition(x, y))
			}
			("swap", "letter") => {
				let x = parts[2].chars().next()?;
				let y = parts[5].chars().next()?;
				Some(Operation::SwapLetter(x, y))
			}
			("rotate", "left") => {
				let x = parts[2].parse::<usize>().ok()?;
				Some(Operation::RotateLeft(x))
			}
			("rotate", "right") => {
				let x = parts[2].parse::<usize>().ok()?;
				Some(Operation::RotateRight(x))
			}
			("rotate", "based") => {
				let x = parts[6].chars().next()?;
				Some(Operation::RotateOnLetter(x))
			}
			("reverse", _) => {
				let x = parts[2].parse::<usize>().ok()?;
				let y = parts[4].parse::<usize>().ok()?;
				Some(Operation::Reverse(x, y))
			}
			("move", _) => {
				let x = parts[2].parse::<usize>().ok()?;
				let y = parts[5].parse::<usize>().ok()?;
				Some(Operation::Move(x, y))
			}
			_ => None,
		}
	}
	fn scramble(self, password: String) -> String {
		match self {
			Operation::SwapPosition(x, y) => swap_position(password, x, y),
			Operation::SwapLetter(x, y) => swap_letter(password, x, y),
			Operation::RotateLeft(x) => rotate_left(password, x),
			Operation::RotateRight(x) => rotate_right(password, x),
			Operation::RotateOnLetter(x) => rotate_on_letter(password, x),
			Operation::Reverse(x, y) => reverse(password, x, y),
			Operation::Move(x, y) => p_move(password, x, y),
		}
	}
	fn unscramble(self, password: String) -> String {
		match self {
			Operation::SwapPosition(x, y) => swap_position(password, x, y),
			Operation::SwapLetter(x, y) => swap_letter(password, x, y),
			Operation::RotateLeft(x) => rotate_right(password, x),
			Operation::RotateRight(x) => rotate_left(password, x),
			Operation::RotateOnLetter(x) => rotate_on_letter_inverse(password, x),
			Operation::Reverse(x, y) => reverse(password, x, y),
			Operation::Move(x, y) => p_move(password, y, x),
		}
	}
}

fn swap_position(password: String, x: usize, y: usize) -> String {
	let mut chars: Vec<char> = password.chars().collect();
	chars.swap(x, y);
	chars.iter().collect::<String>()
}

fn swap_letter(password: String, x: char, y: char) -> String {
	let mut chars: Vec<char> = password.chars().collect();
	let x_pos = chars.iter().position(|&c| c == x).unwrap_or_default();
	let y_pos = chars.iter().position(|&c| c == y).unwrap_or_default();
	chars.swap(x_pos, y_pos);
	chars.iter().collect::<String>()
}

fn rotate_left(password: String, x: usize) -> String {
	let mut chars: Vec<char> = password.chars().collect();
	chars.rotate_left(x);
	chars.iter().collect::<String>()
}

fn rotate_right(password: String, x: usize) -> String {
	let mut chars: Vec<char> = password.chars().collect();
	chars.rotate_right(x);
	chars.iter().collect::<String>()
}

fn rotate_on_letter(password: String, x: char) -> String {
	let mut chars: Vec<char> = password.chars().collect();
	let pos = chars.iter().position(|&c| c == x).unwrap_or_default();
	let shift = if pos >= 4 { 2 + pos } else { 1 + pos } % chars.len();
	chars.rotate_right(shift);
	chars.iter().collect::<String>()
}

fn rotate_on_letter_inverse(password: String, x: char) -> String {
	let chars: Vec<char> = password.chars().collect();
	let len = chars.len();
	// Se rota a derecha e izquierda comparando con el original
	for i in 0..len {
		let mut candidate = chars.clone(); // password rotada izq i veces
		candidate.rotate_left(i);
		let pos = candidate.iter().position(|&c| c == x).unwrap();
		let shift = (if pos >= 4 { pos + 2 } else { pos + 1 }) % len;
		// Rotar a la derecha candidate para obtener candidate_forward
		let mut candidate_forward = candidate.clone();
		candidate_forward.rotate_right(shift);
		// Comparar candidate_forward con el original
		if candidate_forward == chars {
			return candidate.into_iter().collect();
		}
	}
	password
}

fn reverse(password: String, x: usize, y: usize) -> String {
	let mut chars: Vec<char> = password.chars().collect();
	chars[x..=y].reverse();
	chars.iter().collect::<String>()
}

fn p_move(password: String, x: usize, y: usize) -> String {
	let mut chars: Vec<char> = password.chars().collect();
	let c = chars.remove(x);
	chars.insert(y, c);
	chars.iter().collect::<String>()
}

fn part1(input: &str) -> String {
	let mut password = "abcdefgh".to_string();
	for line in input.lines() {
		if let Some(op) = Operation::parse(line) {
			password = op.scramble(password);
		}
	}
	password
}

fn part2(input: &str) -> String {
	let mut password = "fbgdceah".to_string(); // CSpell: ignore fbgdceah
	for line in input.lines().rev() {
		if let Some(op) = Operation::parse(line) {
			password = op.unscramble(password);
		}
	}
	password
}
