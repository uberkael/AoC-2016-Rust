#![allow(unused)]

use std::collections::HashMap;

#[cfg(test)]
mod tests;

pub fn aoc14() {
	println!("\nDay 14: One-Time Pad");
	println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

	let input = std::fs::read_to_string("input/14/input.txt").unwrap();

	println!("Part 1:\n{}", part1(&input));
}

fn find_keys(hashes: &[String], n: usize) -> Vec<usize> {
	let mut keys = Vec::new();
	let mut i = 0;
	while keys.len() < n && i < hashes.len() {
		if let Some(c) = check_triple(&hashes[i]) {
			let max = std::cmp::min(i + 1001, hashes.len());
			if validate(&hashes[i + 1..max], c) {
				// println!("{} - {}", i, hashes[i]);
				keys.push(i);
			}
		}
		i += 1;
	}
	keys
}

fn validate(hashes: &[String], c: u8) -> bool {
	hashes.iter().any(|hash| check_quintuple(hash, c))
}

fn check_triple(hash: &str) -> Option<u8> {
	let bytes = hash.as_bytes();
	bytes
		.windows(3)
		.find(|w| w[0] == w[1] && w[1] == w[2])
		.map(|w| w[0])
}

fn check_quintuple(hash: &str, c: u8) -> bool {
	let target = [c; 5];
	hash.as_bytes().windows(5).any(|w| w == target)
}

fn generate_hashes<const N: usize>(input: &str) -> [String; N] {
	std::array::from_fn(|i| {
		let seed = format!("{}{}", input, i);
		format!("{:x}", md5::compute(seed))
	})
}

/* Part1 */
fn part1(input: &str) -> usize {
	let input = input.trim();
	let hashes = generate_hashes::<30000>(input);
	let results = find_keys(&hashes, 64);
	// println!("{:?}", results.len());
	results[63]
}
