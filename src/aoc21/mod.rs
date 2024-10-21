#[cfg(test)]
mod tests;

pub fn aoc21() {
	println!("\nDay 21: Scrambled Letters and Hash");
	println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

	let input = std::fs::read_to_string("input/21/input.txt")
		.expect("Error reading the file");

	println!("Part 1:\n{}", part1(&input));
	println!("Part 2:\n{}", part2(&input));
}

#[derive(Debug, PartialEq)]
enum Operation {
	SwapPosition(usize, usize),
	SwapLetter(u8, u8),
	RotateLeft(usize),
	RotateRight(usize),
	RotateOnLetter(u8),
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
				let x = parts[2].chars().next()? as u8;
				let y = parts[5].chars().next()? as u8;
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
				let x = parts[6].chars().next()? as u8;
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
	fn scramble(self, password: &mut Vec<u8>) {
		match self {
			Operation::SwapPosition(x, y) => password.swap(x, y),
			Operation::SwapLetter(x, y)   => swap_letter(password, x, y),
			Operation::RotateLeft(x)      => password.rotate_left(x),
			Operation::RotateRight(x)     => password.rotate_right(x),
			Operation::RotateOnLetter(x)  => rotate_on_letter(password, x),
			Operation::Reverse(x, y)      => password[x..=y].reverse(),
			Operation::Move(x, y)         => p_move(password, x, y),
		}
	}
	fn unscramble(self, password: &mut Vec<u8>) {
		match self {
			Operation::SwapPosition(x, y) => password.swap(x, y),
			Operation::SwapLetter(x, y)   => swap_letter(password, x, y),
			Operation::RotateLeft(x)      => password.rotate_right(x),
			Operation::RotateRight(x)     => password.rotate_left(x),
			Operation::RotateOnLetter(x)  => rotate_on_letter_inverse(password, x),
			Operation::Reverse(x, y)      => password[x..=y].reverse(),
			Operation::Move(x, y)         => p_move(password, y, x),
		}
	}
}

fn p_move(password: &mut Vec<u8>, x: usize, y: usize) {
	let c = password.remove(x);
	password.insert(y, c);
}

fn swap_letter(password: &mut Vec<u8>, x: u8, y: u8) {
	let x_pos = password.iter().position(|&c| c == x).unwrap_or_default();
	let y_pos = password.iter().position(|&c| c == y).unwrap_or_default();
	password.swap(x_pos, y_pos);
}

fn rotate_on_letter(password: &mut Vec<u8>, x: u8) {
	let pos = password.iter().position(|&c| c == x).unwrap_or_default();
	let shift = if pos >= 4 { 2 + pos } else { 1 + pos } % password.len();
	password.rotate_right(shift);
}

fn rotate_on_letter_inverse(password: &mut Vec<u8>, x: u8) {
	let len = password.len();
	// Se rota a derecha e izquierda comparando con el original
	for i in 0..len {
		// password rotada izq i veces
		let mut candidate = password.clone();
		candidate.rotate_left(i);
		let pos = candidate.iter().position(|&c| c == x).expect("Letter not found");
		let shift = (if pos >= 4 { pos + 2 } else { pos + 1 }) % len;
		// Rotar a la derecha candidate para obtener candidate_forward
		let mut candidate_forward = candidate.clone();
		candidate_forward.rotate_right(shift);
		// Comparar candidate_forward con el original
		if candidate_forward == *password {
			*password = candidate;
			break;
		}
	}
}

fn part1(input: &str) -> String {
	let mut password: Vec<u8> = "abcdefgh".as_bytes().to_vec();
	for line in input.lines() {
		if let Some(op) = Operation::parse(line) {
			op.scramble(&mut password);
		}
	}
	String::from_utf8(password).expect("Invalid UTF-8")
}

fn part2(input: &str) -> String {
	let mut password: Vec<u8> = "fbgdceah".as_bytes().to_vec(); // CSpell: ignore fbgdceah
	for line in input.lines().rev() {
		if let Some(op) = Operation::parse(line) {
			op.unscramble(&mut password);
		}
	}
	String::from_utf8(password).expect("Invalid UTF-8")
}
