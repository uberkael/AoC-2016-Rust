#![allow(dead_code)]

use rayon::prelude::*;

#[cfg(test)]
mod tests;

pub fn aoc20() {
	println!("\nDay 20: Firewall Rules");
	println!("━━━━━━━━━━━━━━━━━━━━━━");

	let input = std::fs::read_to_string("input/20/input.txt").unwrap();

	println!("Part 1:\n{}", part1(&input));
	// println!("Part 2:\n{}", part2(&input));
}

fn ranges(input: String) -> Vec<bool> {
	let mut ips = vec![true; 4294967296];
	for line in input.lines() {
		let parts: Vec<&str> = line.split('-').collect();
		let start = parts[0].parse::<u32>().unwrap();
		let end = parts[1].parse::<u32>().unwrap();
		// Divide el rango en chunks y procesa en paralelo
		ips[start as usize..=end as usize]
			.par_chunks_mut(1024) // Divide en chunks
			.for_each(|chunk| {
				for i in chunk.iter_mut() {
					*i = false;
				}
			});
	}
	ips
}

fn ip_min(ips: Vec<bool>) -> u32 {
	ips.iter().position(|&x| x).unwrap() as u32
}

fn part1(input: &String) -> u32 {
	let ips = ranges(input.to_string());
	ip_min(ips)
}
