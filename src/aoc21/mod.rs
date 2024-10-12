#![allow(dead_code)]

#[cfg(test)]
mod tests;

pub fn aoc21() {
	println!("\nDay 21: Scrambled Letters and Hash");
	println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

	let input = std::fs::read_to_string("input/21/input.txt").unwrap();

	println!("Part 1:\n{}", part1(&input));
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

fn parse_operation(input: &str) -> Option<Operation> {
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

fn run_operation(password: String, op: Operation) -> String {
	match op {
		Operation::SwapPosition(x, y) => {
			let mut chars: Vec<char> = password.chars().collect();
			chars.swap(x, y);
			chars.iter().collect::<String>()
		}
		Operation::SwapLetter(x, y) => {
			let mut chars: Vec<char> = password.chars().collect();
			let x_pos = chars.iter().position(|&c| c == x).unwrap();
			let y_pos = chars.iter().position(|&c| c == y).unwrap();
			chars.swap(x_pos, y_pos);
			chars.iter().collect::<String>()
		}
		Operation::RotateLeft(x) => {
			let chars: Vec<char> = password.chars().collect();
			let len = chars.len();
			let mut new_chars = vec![' '; len];
			for i in 0..len {
				let new_pos = (i + x) % len;
				new_chars[i] = chars[new_pos];
			}
			new_chars.iter().collect::<String>()
		}
		Operation::RotateRight(x) => {
			let chars: Vec<char> = password.chars().collect();
			let len = chars.len();
			let mut new_chars = vec![' '; len];
			for i in 0..len {
				let new_pos = (i + len - x) % len;
				new_chars[i] = chars[new_pos];
			}
			new_chars.iter().collect::<String>()
		}
		Operation::RotateOnLetter(x) => {
			let chars: Vec<char> = password.chars().collect();
			let pos = chars.iter().position(|&c| c == x).unwrap();
			let n = 1 + pos + if pos >= 4 { 1 } else { 0 };
			let shift = n % chars.len();
			let len = chars.len();
			let mut new_chars = vec![' '; len];
			for i in 0..len {
				let new_pos = (i + len - shift) % len;
				new_chars[i] = chars[new_pos];
			}
			new_chars.iter().collect::<String>()
		}
		Operation::Reverse(x, y) => {
			let chars: Vec<char> = password.chars().collect();
			let mut new_chars = chars.clone();
			for i in x..=y {
				new_chars[i] = chars[y - (i - x)];
			}
			new_chars.iter().collect::<String>()
		}
		Operation::Move(x, y) => {
			let mut chars: Vec<char> = password.chars().collect();
			let c = chars.remove(x);
			chars.insert(y, c);
			chars.iter().collect::<String>()
		}
	}
}

fn part1(input: &str) -> String {
	let mut password = "abcdefgh".to_string();
	for line in input.lines() {
		if let Some(op) = parse_operation(line) {
			password = run_operation(password, op);
		}
	}
	password
}
