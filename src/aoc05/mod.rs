use std::fmt::Write;
use rayon::prelude::*;

#[cfg(test)]
mod tests;

pub fn aoc05() {
	println!("\nDay 5: How About a Nice Game of Chess?");
	println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

	println!("Part:1\n{}", part1());
	println!("Part:2\n{}", part2());
}

fn find_valid_in_range(id: &str, range: std::ops::Range<usize>) -> Vec<(usize, [u8; 16])> {
	range
		.into_par_iter()
		.filter_map(|num| {
			let mut buffer = String::with_capacity(id.len() + 10);
			buffer.push_str(id);
			write!(&mut buffer, "{}", num).unwrap();

			let digest = md5::compute(buffer.as_bytes());
			let digest_bytes: [u8; 16] = digest.into();

			// Check first 5 Hex (2 bytes + 4 bits) sean 0:
			if digest_bytes[0] == 0 && digest_bytes[1] == 0 && (digest_bytes[2] >> 4 == 0) {
				Some((num, digest_bytes))
			} else {
				None
			}
		})
		.collect()
}

/// Extrae el sexto (char)
fn get_char(digest: &[u8; 16]) -> char {
	let nibble = digest[2] & 0xF;
	std::char::from_digit(nibble as u32, 16).unwrap()
}

fn part1() -> String {
	let id = "ojvtpuvg";
	let mut current = 0;
	let chunk_size = 100_000;
	let mut results: Vec<(usize, [u8; 16])> = Vec::new();

	while results.len() < 8 {
		let mut valid = find_valid_in_range(id, current..current + chunk_size);
		results.append(&mut valid);
		results.sort_by_key(|&(num, _)| num);
		results.truncate(8);
		current += chunk_size;
	}
	results
		.iter()
		.map(|&(_, ref digest)| get_char(digest))
		.collect()
}

/// Extrae el sexto (pos) y el séptimo (char).
fn get_data(digest: &[u8; 16]) -> (usize, char) {
	let pos = (digest[2] & 0xF) as usize; // Pos del  nibble bajo de digest[2]
	let nibble = digest[3] >> 4; // Char del nibble alto de digest[3]
	let c = std::char::from_digit(nibble as u32, 16).unwrap();
	(pos, c)
}

fn assign_char(password: &mut [char; 8], (pos, c): (usize, char)) {
	if pos < 8 && password[pos] == '_' {
		password[pos] = c;
	}
}

fn check_finished(password: &[char; 8]) -> bool {
	password.iter().all(|&c| c != '_')
}

fn part2() -> String {
	let id = "ojvtpuvg";
	let mut current = 0;
	let chunk_size = 100_000;
	let mut password = ['_'; 8];

	while !check_finished(&password) {
		let mut valid = find_valid_in_range(id, current..current + chunk_size);
		valid.sort_by_key(|&(num, _)| num);
		for &(_, ref digest) in valid.iter() {
			let d = get_data(digest);
			assign_char(&mut password, d);
			if check_finished(&password) {
				break;
			}
		}
		current += chunk_size;
	}
	password.iter().collect()
}
