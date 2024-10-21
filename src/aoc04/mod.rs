#[cfg(test)]
mod tests;

use std::collections::HashMap;

pub fn aoc04() {
	println!("\nDay 4: Security Through Obscurity");
	println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

	let input = std::fs::read_to_string("input/04/input.txt")
		.expect("Error reading file");

	let rooms: Vec<Room> = input.lines().map(|x| Room::new(x)).collect();

	println!("Part 1:\n{}", parte1(&rooms));
	println!("Part 2:\n{}", parte2(rooms));
}

fn parte1(rooms: &Vec<Room>) -> usize {
	rooms.iter().filter(|r| r.is_real()).map(|r| r.sector).sum()
}

struct Room {
	name: String,
	sector: usize,
	checksum: String,
}

impl Room {
	fn new(s: &str) -> Room {
		let mut parts: Vec<&str> = s.split('-').collect();
		let mut last_part: Vec<&str> = parts // TODO: Simplificar
			.pop()
			.unwrap_or_default()
			.split('[')
			.collect();
		let checksum = last_part
			.pop()
			.unwrap_or_default()
			.replace("]", "")
			.to_string();
		let sector = last_part
			.pop()
			.unwrap_or_default()
			.parse::<usize>()
			.unwrap_or_default();
		let name = parts.join("-");
		Room { name, sector, checksum }
	}
	fn generate_checksum(&self) -> String {
		let mut chars: Vec<char> = self.name.replace("-","").chars().collect();
		chars.sort();
		let mut frequencies: HashMap<char, usize> = HashMap::new();

		for &ch in &chars {
			*frequencies.entry(ch).or_insert(0) += 1;
		}

		let mut freq_vec: Vec<(char, usize)> = frequencies.into_iter().collect();
		freq_vec.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
		freq_vec.into_iter().take(5).map(|(ch, _)| ch).collect()
	}
	fn is_real(&self) -> bool {
		self.generate_checksum() == self.checksum
	}

	/* Part 2 */
	fn decrypt(&self) -> String {
		self.name.chars().map(|c| cipher(c, self.sector)).collect()
	}
}

fn cipher(c: char, n: usize) -> char { // TODO: Simplificar
	if c == '-' {
		return ' ';
	}
	let z = 'z' as usize;
	let n = n % 26;
	let c = c as usize + n;
	if c > z {
		(c as u8 - 26) as char
	} else {
		(c as u8) as char
	}
}

fn parte2(rooms: Vec<Room>) -> usize {
	let reals = rooms.iter().filter(|r| r.is_real());
	for room in reals {
		let decrypted = room.decrypt();
		if decrypted.contains("north") {
			println!("{}", decrypted);
			return room.sector;
		}
	}
	0
}
