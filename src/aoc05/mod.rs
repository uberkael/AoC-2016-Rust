use rayon::prelude::*;

#[cfg(test)]
mod tests;

pub fn aoc05() {
	println!("\nDay 5: How About a Nice Game of Chess?");
	println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

	/* Part 1 */
	let password: String = part1();
	println!("Part:1\n{password}");

	/* Part 2 */
	let password: String = part2();
	println!("Part:2\n{password}");
}

/// Process a given range in parallel, returning (number, hash) pairs which meet the condition.
fn find_valid_in_range(id: &str, range: std::ops::Range<usize>) -> Vec<(usize, String)> {
	range.into_par_iter()
		.map(|num| {
			let s = format!("{}{}", id, num);
			let hash = format!("{:x}", md5::compute(s.as_bytes()));
			(num, hash)
		})
		.filter(|(_, hash)| check_zeros(hash))
		.collect()
}

fn part1() -> String {
	let id = "ojvtpuvg";
	let mut current = 0;
	let chunk_size = 100_000;
	let mut results: Vec<(usize, String)> = Vec::new();

	// Process in chunks until we have at least 8 results (sorted by number)
	while results.len() < 8 {
		let mut valid = find_valid_in_range(id, current..current + chunk_size);
		results.append(&mut valid);
		results.sort_by_key(|&(num, _)| num);
		results.truncate(8);
		current += chunk_size;
	}
	// For each valid hash take its sixth hexadecimal character.
	results.iter().map(|&(_, ref hash)| get_char(hash)).collect()
}

fn get_char(hash: &str) -> char {
	// returns the 6th character (index 5) of the hash
	hash.chars().nth(5).unwrap_or('_')
}

fn check_zeros(s: &str) -> bool {
	s.starts_with("00000")
}

fn get_data(s: &str) -> (usize, char) {
	// Get position and character from the MD5 hash
	let chars: Vec<_> = s.chars().collect();
	let pos = chars.get(5).and_then(|c| c.to_digit(16)).unwrap_or(0) as usize;
	let c   = *chars.get(6).unwrap_or(&'_');
	(pos, c)
}

fn assign_char(password: &mut [char; 8], (pos, c): (usize, char)) {
	if pos < 8 && password[pos] == '_' {
		password[pos] = c;
	}
}

fn check_finished(password: &[char; 8]) -> bool {
	password.par_iter().all(|&c| c != '_')
}

fn part2() -> String {
	let id = "ojvtpuvg";
	let mut current = 0;
	let chunk_size = 100_000;
	let mut password = ['_'; 8];

	while !check_finished(&password) {
		let mut valid = find_valid_in_range(id, current..current + chunk_size);
		valid.sort_by_key(|&(num, _)| num);
		for &(_, ref hash) in valid.iter() {
			let d = get_data(hash);
			assign_char(&mut password, d);
			if check_finished(&password) {
				break;
			}
		}
		current += chunk_size;
	}
	password.iter().collect()
}
