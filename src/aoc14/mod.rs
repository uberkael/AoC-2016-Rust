use std::collections::HashMap;
use rayon::prelude::*;

#[cfg(test)]
mod tests;

pub fn aoc14() {
	println!("\nDay 14: One-Time Pad");
	println!("━━━━━━━━━━━━━━━━━━━━");

	let input = std::fs::read_to_string("input/14/input.txt")
		.expect("Error reading the file");

	println!("Part 1:\n{}", part1(&input));
	println!("Part 2:\n{}", part2(&input));
}

fn find_keys(hashes: &[String], n: usize) -> Vec<usize> {
	let quintuples = precompute_quintuples(hashes);
	let mut keys = Vec::with_capacity(n);
	for (i, hash) in hashes.iter().enumerate() {
		if let Some(c) = check_triple(hash) {
			let min = i + 1;
			let max = i + 1001;
			if quintuples
				.get(&c)
				.map_or(false, |qs| qs.iter().any(|&q| q >= min && q < max))
			{
				keys.push(i);
				if keys.len() == n {
					break;
				}
			}
		}
	}
	keys
}

fn check_triple(hash: &str) -> Option<u8> {
	let bytes = hash.as_bytes();
	bytes
		.windows(3)
		.find(|w| w[0] == w[1] && w[1] == w[2])
		.map(|w| w[0])
}

fn generate_hashes<const N: usize>(input: &str) -> [String; N] {
	std::array::from_fn(|i| {
		let seed = format!("{}{}", input, i);
		format!("{:x}", md5::compute(seed))
	})
}

fn precompute_quintuples(hashes: &[String]) -> HashMap<u8, Vec<usize>> {
	hashes
		.iter()
		.enumerate()
		.filter_map(|(i, hash)| {
			let qs = check_quintuples(hash);
			(!qs.is_empty()).then_some((i, qs))
		})
		.fold(HashMap::new(), |mut acc, (i, qs)| {
			for c in qs {
				acc.entry(c).or_default().push(i);
			}
			acc
		})
}

fn check_quintuples(hash: &str) -> Vec<u8> {
	let bytes = hash.as_bytes();
	let mut result = Vec::new();
	let mut i = 0;
	while i <= bytes.len().saturating_sub(5) {
		if bytes[i] == bytes[i + 1]
			&& bytes[i + 1] == bytes[i + 2]
			&& bytes[i + 2] == bytes[i + 3]
			&& bytes[i + 3] == bytes[i + 4]
		{
			result.push(bytes[i]);
			i += 5;
		} else {
			i += 1;
		}
	}
	result
}

fn to_hex_bytes(digest: &md5::Digest, buffer: &mut [u8; 32]) {
	const HEX_CHARS: &[u8] = b"0123456789abcdef";
	for (i, &byte) in digest.iter().enumerate() {
		buffer[i * 2] = HEX_CHARS[(byte >> 4) as usize];
		buffer[i * 2 + 1] = HEX_CHARS[(byte & 0xf) as usize];
	}
}

fn key_stretching<const N: usize>(input: &str) -> [String; N] {
	let mut result: [String; N] = std::array::from_fn(|_| String::new());
	result.par_iter_mut().enumerate().for_each(|(i, s)| {
		let mut seed = format!("{}{}", input, i);
		let mut hex_buffer = [0u8; 32];
		for _ in 0..2017 {
			let digest = md5::compute(seed.as_bytes());
			to_hex_bytes(&digest, &mut hex_buffer);
			seed = unsafe { String::from_utf8_unchecked(hex_buffer.to_vec()) };
		}
		*s = seed;
	});
	result
}

/* Part1 */
fn part1(input: &str) -> usize {
	let input = input.trim();
	let hashes = generate_hashes::<30000>(input);
	let results = find_keys(&hashes, 64);
	results[63]
}

/* Part2 */
fn part2(input: &str) -> usize {
	let input = input.trim();
	let hashes = key_stretching::<30000>(input);
	let results = find_keys(&hashes, 64);
	results[63]
}
